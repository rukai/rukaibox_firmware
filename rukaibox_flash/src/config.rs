use arrayvec::ArrayVec;
use kdl::{KdlDocument, KdlNode};
use kdl_config::{KdlConfig, KdlConfigFinalize, Parsed, error::ParseError};
use kdl_config_derive::{KdlConfig, KdlConfigFinalize};
use miette::{IntoDiagnostic, NamedSource, miette};
use rkyv::rancor::Error;
use rukaibox_config::Config;
use std::path::PathBuf;

pub fn encode_config(config: &Config) -> miette::Result<Vec<u8>> {
    let bytes = rkyv::to_bytes::<Error>(config).map_err(|e| miette!(e))?;
    let mut result = vec![];
    result.extend((bytes.len() as u32).to_be_bytes());
    result.extend(bytes.iter());
    Ok(result)
}

pub fn load() -> miette::Result<Config> {
    let input = load_source(None)?;
    // TODO: upstream a way to tell KDL parser what the filename is.
    let kdl: KdlDocument = input.inner().parse()?;
    let (profile, error): (Parsed<ConfigKdl>, ParseError) = kdl_config::parse(input, kdl);

    // TODO: extra diagnostics here.

    if !error.diagnostics.is_empty() {
        return Err(error.into());
    }

    Ok(profile.value.finalize())
}

fn load_source(path: Option<PathBuf>) -> miette::Result<NamedSource<String>> {
    let path = if let Some(path) = path {
        path
    } else if let Ok(cargo_manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(cargo_manifest_dir)
            .parent()
            .unwrap()
            .join("config.kdl")
    } else {
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("config.kdl")
    };
    let filename = path.file_name().unwrap().to_str().unwrap();
    let text = std::fs::read_to_string(&path)
        .into_diagnostic()
        .map_err(|e| e.context(format!("Failed to load config file at {path:?}")))?;
    Ok(NamedSource::new(filename, text))
}

#[derive(KdlConfig, KdlConfigFinalize, Default, Debug)]
#[kdl_config_finalize_into = "rukaibox_config::Config"]
pub struct ConfigKdl {
    pub version: Parsed<u32>,
    pub profiles: Parsed<ArrayVec<Parsed<ProfileKdl>, 2>>,
}

// TODO: add derive side validation that Parsed is used everywhere.
#[derive(KdlConfig, KdlConfigFinalize, Default, Debug)]
#[kdl_config_finalize_into = "rukaibox_config::Profile"]
pub struct ProfileKdl {
    pub activation_combination: Parsed<ArrayVec<Parsed<PhysicalButtonKdl>, 10>>,
    pub logic: Parsed<BaseLogicKdl>,
    pub socd: Parsed<SocdTypeKdl>,
    pub left_hand: Parsed<LeftHandMapKdl>,
    pub right_hand: Parsed<RightHandMapKdl>,
}

#[derive(KdlConfig, KdlConfigFinalize, Default, Debug)]
#[kdl_config_finalize_into = "rukaibox_config::LeftHandMap"]
pub struct LeftHandMapKdl {
    pub pinky: Parsed<LogicalButtonKdl>,
    pub ring: Parsed<LogicalButtonKdl>,
    pub middle: Parsed<LogicalButtonKdl>,
    pub index: Parsed<LogicalButtonKdl>,

    pub middle_2: Parsed<LogicalButtonKdl>,

    pub thumb_left: Parsed<LogicalButtonKdl>,
    pub thumb_right: Parsed<LogicalButtonKdl>,
}

#[derive(KdlConfig, KdlConfigFinalize, Default, Debug)]
#[kdl_config_finalize_into = "rukaibox_config::RightHandMap"]
pub struct RightHandMapKdl {
    pub index: Parsed<LogicalButtonKdl>,
    pub middle: Parsed<LogicalButtonKdl>,
    pub ring: Parsed<LogicalButtonKdl>,
    pub pinky: Parsed<LogicalButtonKdl>,

    pub index_2: Parsed<LogicalButtonKdl>,
    pub middle_2: Parsed<LogicalButtonKdl>,
    pub ring_2: Parsed<LogicalButtonKdl>,
    pub pinky_2: Parsed<LogicalButtonKdl>,

    pub thumb_left: Parsed<LogicalButtonKdl>,
    pub thumb_right: Parsed<LogicalButtonKdl>,
    pub thumb_up: Parsed<LogicalButtonKdl>,
    pub thumb_down: Parsed<LogicalButtonKdl>,
    pub thumb_middle: Parsed<LogicalButtonKdl>,
}

#[derive(KdlConfig, KdlConfigFinalize, Default, Debug)]
#[kdl_config_finalize_into = "rukaibox_config::SocdType"]
pub enum SocdTypeKdl {
    #[default]
    SecondInputPriority,
    Neutral,
}

#[derive(KdlConfig, KdlConfigFinalize, Default, Debug)]
#[kdl_config_finalize_into = "rukaibox_config::BaseLogic"]
pub enum BaseLogicKdl {
    #[default]
    ProjectPlus,
    Rivals2,
}

#[derive(KdlConfig, KdlConfigFinalize, Default, Debug)]
#[kdl_config_finalize_into = "rukaibox_config::PhysicalButton"]
pub enum PhysicalButtonKdl {
    #[default]
    Start,
    LeftHandPinky,
    LeftHandRing,
    LeftHandMiddle,
    LeftHandIndex,

    LeftHandMiddle2,

    LeftHandThumbLeft,
    LeftHandThumbRight,

    RightHandIndex,
    RightHandMiddle,
    RightHandRing,
    RightHandPink,

    RightHandIndex2,
    RightHandMiddle2,
    RightHandRing2,
    RightHandPink2,

    RightHandThumbLeft,
    RightHandThumbRight,
    RightHandThumbUp,
    RightHandThumbDown,
    RightHandThumbMiddle,
}

#[derive(KdlConfig, KdlConfigFinalize, Default, Debug)]
#[kdl_config_finalize_into = "rukaibox_config::LogicalButton"]
pub enum LogicalButtonKdl {
    #[default]
    LAnalog,
    RAnalog,
    LDigital,
    RDigital,
    StickUp,
    StickDown,
    StickLeft,
    StickRight,
    CstickUp,
    CstickDown,
    CstickLeft,
    CstickRight,
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
    ModX,
    ModY,
    A,
    B,
    X,
    Y,
    Z,
}
