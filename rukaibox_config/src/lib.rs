#![no_std]

use arrayvec::ArrayVec;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default)]
#[rkyv(derive(Debug))]
pub struct Config {
    pub version: u32,
    pub profiles: ArrayVec<Profile, 10>,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default)]
#[rkyv(derive(Debug))]
pub struct Profile {
    pub activation_combination: ArrayVec<PhysicalButton, 10>,
    pub logic: BaseLogic,
    pub socd: SocdType,
    pub left_hand: LeftHandMap,
    pub right_hand: RightHandMap,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default)]
#[rkyv(derive(Debug))]
pub struct LeftHandMap {
    pub pinky: LogicalButton,
    pub ring: LogicalButton,
    pub middle: LogicalButton,
    pub index: LogicalButton,

    pub middle_2: LogicalButton,

    pub thumb_left: LogicalButton,
    pub thumb_right: LogicalButton,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default)]
#[rkyv(derive(Debug))]
pub struct RightHandMap {
    pub index: LogicalButton,
    pub middle: LogicalButton,
    pub ring: LogicalButton,
    pub pinky: LogicalButton,

    pub index_2: LogicalButton,
    pub middle_2: LogicalButton,
    pub ring_2: LogicalButton,
    pub pinky_2: LogicalButton,

    pub thumb_left: LogicalButton,
    pub thumb_right: LogicalButton,
    pub thumb_up: LogicalButton,
    pub thumb_down: LogicalButton,
    pub thumb_middle: LogicalButton,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default)]
#[rkyv(derive(Debug))]
pub enum SocdType {
    #[default]
    SecondInputPriority,
    Neutral,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default)]
#[rkyv(derive(Debug))]
pub enum BaseLogic {
    #[default]
    ProjectPlus,
    Rivals2,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default)]
#[rkyv(derive(Debug))]
pub enum PhysicalButton {
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

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default)]
#[rkyv(derive(Debug))]
pub enum LogicalButton {
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
