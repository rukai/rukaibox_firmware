use embedded_hal::digital::InputPin;
use rp2040_hal::gpio::{DynPinId, FunctionSioInput, Pin, PullUp};
use rukaibox_config::{LogicalButtonToPhysicalButton, PhysicalButton};

/// The buttons are named by the finger you press them with.
/// Starting at your thumb, fingers are named, thumb -> index -> middle -> ring -> pinky
/// The lower row is considered the base row and then a `2` is added to describe the row above.
pub struct ButtonInput {
    pub left_pinky: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub left_ring: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub left_middle: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub left_index: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub left_middle_2: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub left_thumb_left: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub left_thumb_right: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub right_index: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_middle: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_ring: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_pinky: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub right_index_2: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_middle_2: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_ring_2: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_pinky_2: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub right_thumb_left: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_thumb_right: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_thumb_up: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_thumb_down: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_thumb_middle: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub start: Pin<DynPinId, FunctionSioInput, PullUp>,
}

impl ButtonInput {
    pub fn get_pin_state(&mut self) -> ButtonInputResults {
        ButtonInputResults {
            left_pinky: self.left_pinky.is_low().unwrap(),
            left_ring: self.left_ring.is_low().unwrap(),
            left_middle: self.left_middle.is_low().unwrap(),
            left_index: self.left_index.is_low().unwrap(),

            left_middle_2: self.left_middle_2.is_low().unwrap(),

            left_thumb_left: self.left_thumb_left.is_low().unwrap(),
            left_thumb_right: self.left_thumb_right.is_low().unwrap(),

            right_index: self.right_index.is_low().unwrap(),
            right_middle: self.right_middle.is_low().unwrap(),
            right_ring: self.right_ring.is_low().unwrap(),
            right_pinky: self.right_pinky.is_low().unwrap(),

            right_index_2: self.right_index_2.is_low().unwrap(),
            right_middle_2: self.right_middle_2.is_low().unwrap(),
            right_ring_2: self.right_ring_2.is_low().unwrap(),
            right_pinky_2: self.right_pinky_2.is_low().unwrap(),

            right_thumb_left: self.right_thumb_left.is_low().unwrap(),
            right_thumb_right: self.right_thumb_right.is_low().unwrap(),
            right_thumb_up: self.right_thumb_up.is_low().unwrap(),
            right_thumb_down: self.right_thumb_down.is_low().unwrap(),
            right_thumb_middle: self.right_thumb_middle.is_low().unwrap(),

            start: self.start.is_low().unwrap(),
        }
    }
}

pub struct ButtonInputResults {
    pub left_pinky: bool,
    pub left_ring: bool,
    pub left_middle: bool,
    pub left_index: bool,

    pub left_middle_2: bool,

    pub left_thumb_left: bool,
    pub left_thumb_right: bool,

    pub right_index: bool,
    pub right_middle: bool,
    pub right_ring: bool,
    pub right_pinky: bool,

    pub right_index_2: bool,
    pub right_middle_2: bool,
    pub right_ring_2: bool,
    pub right_pinky_2: bool,

    pub right_thumb_left: bool,
    pub right_thumb_right: bool,
    pub right_thumb_up: bool,
    pub right_thumb_down: bool,
    pub right_thumb_middle: bool,

    pub start: bool,
}

impl ButtonInputResults {
    pub fn to_gc(&self, map: &LogicalButtonToPhysicalButton) -> ButtonInputLogical {
        ButtonInputLogical {
            mod_x: self.get_button_value(map.mod_x),
            mod_y: self.get_button_value(map.mod_y),
            start: self.get_button_value(map.start),
            a: self.get_button_value(map.a),
            b: self.get_button_value(map.b),
            x: self.get_button_value(map.x),
            y: self.get_button_value(map.y),
            z: self.get_button_value(map.z),
            dpad_up: self.get_button_value(map.dpad_up),
            // TODO
            dpad_down: false,
            dpad_left: false,
            dpad_right: false,
            l_digital: self.get_button_value(map.l_digital),
            r_digital: self.get_button_value(map.r_digital),
            l_analog: self.get_button_value(map.l_analog),
            r_analog: self.get_button_value(map.r_analog),
            stick_left: self.get_button_value(map.stick_left),
            stick_right: self.get_button_value(map.stick_right),
            stick_up: self.get_button_value(map.stick_up) || self.get_button_value(map.stick_up2),
            stick_down: self.get_button_value(map.stick_down),
            cstick_left: self.get_button_value(map.cstick_left),
            cstick_right: self.get_button_value(map.cstick_right),
            cstick_up: self.get_button_value(map.cstick_up),
            cstick_down: self.get_button_value(map.cstick_down),
        }
    }

    pub fn get_button_value(&self, button: PhysicalButton) -> bool {
        match button {
            PhysicalButton::Start => self.start,
            PhysicalButton::LeftPinky => self.left_pinky,
            PhysicalButton::LeftRing => self.left_ring,
            PhysicalButton::LeftMiddle => self.left_middle,
            PhysicalButton::LeftIndex => self.left_index,
            PhysicalButton::LeftMiddle2 => self.left_middle_2,
            PhysicalButton::LeftThumbLeft => self.left_thumb_left,
            PhysicalButton::LeftThumbRight => self.left_thumb_right,

            PhysicalButton::RightIndex => self.right_index,
            PhysicalButton::RightMiddle => self.right_middle,
            PhysicalButton::RightRing => self.right_ring,
            PhysicalButton::RightPinky => self.right_pinky,
            PhysicalButton::RightIndex2 => self.right_index_2,
            PhysicalButton::RightMiddle2 => self.right_middle_2,
            PhysicalButton::RightRing2 => self.right_ring_2,
            PhysicalButton::RightPinky2 => self.right_pinky_2,

            PhysicalButton::RightThumbLeft => self.right_thumb_left,
            PhysicalButton::RightThumbRight => self.right_thumb_right,
            PhysicalButton::RightThumbUp => self.right_thumb_up,
            PhysicalButton::RightThumbDown => self.right_thumb_down,
            PhysicalButton::RightThumbMiddle => self.right_thumb_middle,
            PhysicalButton::None => false,
        }
    }
}

pub struct ButtonInputLogical {
    pub mod_x: bool,
    pub mod_y: bool,

    pub start: bool,
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub z: bool,

    // TODO
    pub dpad_up: bool,
    #[allow(dead_code)]
    pub dpad_down: bool,
    #[allow(dead_code)]
    pub dpad_left: bool,
    #[allow(dead_code)]
    pub dpad_right: bool,

    pub l_digital: bool,
    pub r_digital: bool,
    pub l_analog: bool,
    pub r_analog: bool,

    pub stick_left: bool,
    pub stick_right: bool,
    pub stick_up: bool,
    pub stick_down: bool,

    pub cstick_left: bool,
    pub cstick_right: bool,
    pub cstick_up: bool,
    pub cstick_down: bool,
}
