use miette::Result;

impl Config {
    pub fn parse() -> Result<Self> {
        let config = include_str!("../../config.kdl");
        Ok(knus::parse::<Config>("config.kdl", config)?)
    }
}

#[derive(knus::Decode, Debug)]
pub struct Config {
    #[knus(child, unwrap(children))]
    pub profiles: Vec<Profile>,
}

#[derive(knus::Decode, Debug)]
pub struct Profile {
    #[knus(child, unwrap(argument))]
    pub default: bool,
    //#[knus(child, unwrap(arguments))]
    //pub activation_combination: Vec<PhysicalButton>,
    #[knus(child, unwrap(argument))]
    pub logic: BaseLogic,
    #[knus(child, unwrap(argument))]
    pub socd: SocdType,
    #[knus(child)]
    pub left_hand: LeftHandMap,
    #[knus(child)]
    pub right_hand: RightHandMap,
}

#[derive(knus::Decode, Debug)]
pub struct LeftHandMap {
    #[knus(child, unwrap(argument))]
    pub pinky: LogicalButton,
    #[knus(child, unwrap(argument))]
    pub ring: LogicalButton,
    #[knus(child, unwrap(argument))]
    pub middle: LogicalButton,
    #[knus(child, unwrap(argument))]
    pub index: LogicalButton,

    #[knus(child, unwrap(argument))]
    pub middle_2: LogicalButton,

    #[knus(child, unwrap(argument))]
    pub thumb_left: LogicalButton,
    #[knus(child, unwrap(argument))]
    pub thumb_right: LogicalButton,
}

#[derive(knus::Decode, Debug)]
pub struct RightHandMap {
    #[knus(child, unwrap(argument))]
    pub index: LogicalButton,
    #[knus(child, unwrap(argument))]
    pub middle: LogicalButton,
    #[knus(child, unwrap(argument))]
    pub ring: LogicalButton,
    #[knus(child, unwrap(argument))]
    pub pinky: LogicalButton,

    #[knus(child, unwrap(argument))]
    pub index_2: LogicalButton,
    #[knus(child, unwrap(argument))]
    pub middle_2: LogicalButton,
    #[knus(child, unwrap(argument))]
    pub ring_2: LogicalButton,
    #[knus(child, unwrap(argument))]
    pub pinky_2: LogicalButton,

    #[knus(child, unwrap(argument))]
    pub thumb_left: LogicalButton,
    #[knus(child, unwrap(argument))]
    pub thumb_right: LogicalButton,
    #[knus(child, unwrap(argument))]
    pub thumb_up: LogicalButton,
    #[knus(child, unwrap(argument))]
    pub thumb_down: LogicalButton,
    #[knus(child, unwrap(argument))]
    pub thumb_middle: LogicalButton,
}

#[derive(knus::DecodeScalar, Debug)]
pub enum SocdType {
    SecondInputPriority,
    Neutral,
}

#[derive(knus::DecodeScalar, Debug)]
pub enum BaseLogic {
    ProjectPlus,
    Rivals2,
}

#[derive(knus::DecodeScalar, Debug)]
pub enum PhysicalButton {
    Start,
    LeftHandPinky,
    LeftHandRing,
    LeftHandMiddle,
    LeftHandIndex,
    // TODO
}

#[derive(knus::DecodeScalar, Debug)]
pub enum LogicalButton {
    //LTrigger(u8),
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
