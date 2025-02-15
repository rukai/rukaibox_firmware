use std::path::PathBuf;

use arrayvec::ArrayVec;
use kdl::{KdlDocument, KdlEntry, KdlNode};
use kdl_config::{KdlConfig, Parsed, error::ParseError};
use kdl_config_derive::KdlConfig;
use miette::{IntoDiagnostic, NamedSource};

pub fn load() -> miette::Result<ConfigParsed> {
    let input = load_source(None)?;
    // TODO: upstream a way to tell KDL parser what the filename is.
    let kdl: KdlDocument = input.inner().parse()?;
    let (profile, error): (Parsed<ConfigParsed>, ParseError) = kdl_config::parse(input, kdl);

    // TODO: extra diagnostics here.

    if !error.diagnostics.is_empty() {
        return Err(error.into());
    }

    Ok(profile.value)
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

#[derive(KdlConfig, Default, Debug)]
pub struct ConfigParsed {
    pub version: Parsed<u32>,
    pub profiles: Parsed<ArrayVec<Parsed<ProfileParsed>, 10>>,
}

// TODO: add derive side validation that Parsed is used everywhere.
#[derive(KdlConfig, Default, Debug)]
pub struct ProfileParsed {
    pub activation_combination: Parsed<ArrayVec<Parsed<PhysicalButtonParsed>, 10>>,
    pub logic: Parsed<BaseLogicParsed>,
    pub socd: Parsed<SocdTypeParsed>,
    pub left_hand: Parsed<LeftHandMapParsed>,
    pub right_hand: Parsed<RightHandMapParsed>,
}

#[derive(KdlConfig, Default, Debug)]
pub struct LeftHandMapParsed {
    pub pinky: Parsed<LogicalButtonParsed>,
    pub ring: Parsed<LogicalButtonParsed>,
    pub middle: Parsed<LogicalButtonParsed>,
    pub index: Parsed<LogicalButtonParsed>,

    pub middle_2: Parsed<LogicalButtonParsed>,

    pub thumb_left: Parsed<LogicalButtonParsed>,
    pub thumb_right: Parsed<LogicalButtonParsed>,
}

#[derive(KdlConfig, Default, Debug)]
pub struct RightHandMapParsed {
    pub index: Parsed<LogicalButtonParsed>,
    pub middle: Parsed<LogicalButtonParsed>,
    pub ring: Parsed<LogicalButtonParsed>,
    pub pinky: Parsed<LogicalButtonParsed>,

    pub index_2: Parsed<LogicalButtonParsed>,
    pub middle_2: Parsed<LogicalButtonParsed>,
    pub ring_2: Parsed<LogicalButtonParsed>,
    pub pinky_2: Parsed<LogicalButtonParsed>,

    pub thumb_left: Parsed<LogicalButtonParsed>,
    pub thumb_right: Parsed<LogicalButtonParsed>,
    pub thumb_up: Parsed<LogicalButtonParsed>,
    pub thumb_down: Parsed<LogicalButtonParsed>,
    pub thumb_middle: Parsed<LogicalButtonParsed>,
}

#[derive(KdlConfig, Default, Debug)]
pub enum SocdTypeParsed {
    #[default]
    SecondInputPriority,
    Neutral,
}

#[derive(KdlConfig, Default, Debug)]
pub enum BaseLogicParsed {
    #[default]
    ProjectPlus,
    Rivals2,
}

#[derive(KdlConfig, Default, Debug)]
pub enum PhysicalButtonParsed {
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

#[derive(KdlConfig, Default, Debug)]
pub enum LogicalButtonParsed {
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
