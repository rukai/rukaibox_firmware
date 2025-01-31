use embedded_hal::digital::InputPin;
use rp2040_hal::gpio::{DynPinId, FunctionSioInput, Pin, PullUp};

pub struct Input {
    pub left: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub up: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub up2: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub down: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub c_left: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub c_right: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub c_up: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub c_down: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub a: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub b: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub l: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub r: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub x: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub y: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub z: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub taunt: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub x_modifier: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub y_modifier: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub start: Pin<DynPinId, FunctionSioInput, PullUp>,
}

pub struct GamecubeInput {
    pub start: bool,
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub dpad_up: bool,
    pub dpad_down: bool,
    pub dpad_left: bool,
    pub dpad_right: bool,
    pub l_digital: bool,
    pub r_digital: bool,
    pub stick_x: u8,
    pub stick_y: u8,
    pub cstick_x: u8,
    pub cstick_y: u8,
    pub l_analog: u8,
    pub r_analog: u8,
}

impl Input {
    pub fn poll(&mut self) -> GamecubeInput {
        let stick_x = match (self.left.is_low().unwrap(), self.right.is_low().unwrap()) {
            (true, false) => 0,
            (false, true) => 255,
            _ => 128,
        };
        let stick_y = match (
            self.up.is_low().unwrap() || self.up2.is_low().unwrap(),
            self.down.is_low().unwrap(),
        ) {
            (true, false) => 255,
            (false, true) => 0,
            _ => 128,
        };
        let cstick_x = match (
            self.c_left.is_low().unwrap(),
            self.c_right.is_low().unwrap(),
        ) {
            (true, false) => 0,
            (false, true) => 255,
            _ => 128,
        };
        let cstick_y = match (self.c_up.is_low().unwrap(), self.c_down.is_low().unwrap()) {
            (true, false) => 255,
            (false, true) => 0,
            _ => 128,
        };

        GamecubeInput {
            start: self.start.is_low().unwrap(),
            a: self.a.is_low().unwrap(),
            b: self.b.is_low().unwrap(),
            x: self.x.is_low().unwrap(),
            y: self.y.is_low().unwrap(),
            z: self.z.is_low().unwrap(),
            dpad_up: self.taunt.is_low().unwrap(),
            dpad_down: false,
            dpad_left: false,
            dpad_right: false,
            l_digital: self.l.is_low().unwrap(),
            r_digital: self.r.is_low().unwrap(),
            stick_x,
            stick_y,
            cstick_x,
            cstick_y,
            l_analog: 0,
            r_analog: 0,
        }
    }
}
