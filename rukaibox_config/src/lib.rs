#![no_std]

// Memory layout
pub const RP2040_FLASH_OFFSET: usize = 0x10000000;
pub const RP2040_FLASH_SIZE: usize = 1024 * 1024 * 16; // 16 MiB

pub const FIRMWARE_OFFSET: usize = 0;
pub const FIRMWARE_SIZE: usize = 1024 * 1024 * 15; // 15 MiB
pub const CONFIG_OFFSET: usize = 1024 * 1024 * 15;
pub const CONFIG_SIZE: usize = 256; // 10 KiB

use arrayvec::ArrayVec;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default)]
#[rkyv(derive(Debug))]
pub struct Config {
    pub version: u32,
    pub profiles: ArrayVec<Profile, 2>,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default)]
#[rkyv(derive(Debug))]
pub struct Profile {
    pub activation_combination: ArrayVec<PhysicalButton, 10>,
    pub logic: BaseLogic,
    pub socd: SocdType,
    pub buttons: LogicalButtonToPhysicalButton,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
#[rkyv(derive(Debug))]
pub struct LogicalButtonToPhysicalButton {
    pub mod_x: PhysicalButton,
    pub mod_y: PhysicalButton,

    pub start: PhysicalButton,
    pub a: PhysicalButton,
    pub b: PhysicalButton,
    pub x: PhysicalButton,
    pub y: PhysicalButton,
    pub z: PhysicalButton,

    pub dpad_up: PhysicalButton,
    pub dpad_down: PhysicalButton,
    pub dpad_left: PhysicalButton,
    pub dpad_right: PhysicalButton,

    pub l_digital: PhysicalButton,
    pub r_digital: PhysicalButton,
    pub l_analog: PhysicalButton,
    pub r_analog: PhysicalButton,

    pub stick_left: PhysicalButton,
    pub stick_right: PhysicalButton,
    pub stick_up: PhysicalButton,
    /// quick hack to work around lack of OR
    pub stick_up2: PhysicalButton,
    pub stick_down: PhysicalButton,

    pub cstick_left: PhysicalButton,
    pub cstick_right: PhysicalButton,
    pub cstick_up: PhysicalButton,
    pub cstick_down: PhysicalButton,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default, Clone, Copy)]
#[rkyv(derive(Debug))]
pub enum SocdType {
    #[default]
    SecondInputPriority,
    Neutral,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default, Clone, Copy)]
#[rkyv(derive(Debug))]
pub enum BaseLogic {
    #[default]
    ProjectPlus,
    Rivals2,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default, Clone, Copy)]
#[rkyv(derive(Debug))]
pub enum PhysicalButton {
    #[default]
    Start,
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,

    LeftMiddle2,

    LeftThumbLeft,
    LeftThumbRight,

    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,

    RightIndex2,
    RightMiddle2,
    RightRing2,
    RightPinky2,

    RightThumbLeft,
    RightThumbRight,
    RightThumbUp,
    RightThumbDown,
    RightThumbMiddle,

    // This will never be pressed
    None,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default, Clone, Copy)]
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
