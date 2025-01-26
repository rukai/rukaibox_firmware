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

impl Input {
    pub fn poll(&mut self) -> [u8; 8] {
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
        let c_x = match (
            self.c_left.is_low().unwrap(),
            self.c_right.is_low().unwrap(),
        ) {
            (true, false) => 0,
            (false, true) => 255,
            _ => 128,
        };
        let c_y = match (self.c_up.is_low().unwrap(), self.c_down.is_low().unwrap()) {
            (true, false) => 255,
            (false, true) => 0,
            _ => 128,
        };
        let analog_l = 0;
        let analog_r = 0;

        let buttons1 = if self.a.is_low().unwrap() {
            0b0000_0001
        } else {
            0
        } | if self.b.is_low().unwrap() {
            0b0000_0010
        } else {
            0
        } | if self.x.is_low().unwrap() {
            0b0000_0100
        } else {
            0
        } | if self.y.is_low().unwrap() {
            0b0000_1000
        } else {
            0
        } | if self.start.is_low().unwrap() {
            0b0001_0000
        } else {
            0
        };

        #[rustfmt::skip]
            let buttons2 = 0b1000_0000
            // up taunt
            | if self.taunt.is_low().unwrap() { 0b0000_1000 } else { 0 }
            | if self.z.is_low().unwrap() { 0b0001_0000 } else { 0 }
            | if self.r.is_low().unwrap() { 0b0010_0000 } else { 0 }
            | if self.l.is_low().unwrap() { 0b0100_0000 } else { 0 };

        [
            buttons1, buttons2, stick_x, stick_y, c_x, c_y, analog_l, analog_r,
        ]
    }
}
