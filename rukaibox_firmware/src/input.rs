use embedded_hal::digital::InputPin;
use rp2040_hal::gpio::{DynPinId, FunctionSioInput, Pin, PullUp};

/// The buttons are named by the finger you press them with.
/// Starting at your thumb, fingers are named, thumb -> index -> middle -> ring -> pinky
/// The lower row is considered the base row and then a `2` is added to describe the row above.
pub struct ButtonInput {
    pub left_hand_pinky: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub left_hand_ring: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub left_hand_middle: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub left_hand_index: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub left_hand_middle_2: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub left_hand_thumb_left: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub left_hand_thumb_right: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub right_hand_index: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_hand_middle: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_hand_ring: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_hand_pinky: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub right_hand_index_2: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_hand_middle_2: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_hand_ring_2: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_hand_pinky_2: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub right_hand_thumb_left: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_hand_thumb_right: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_hand_thumb_up: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_hand_thumb_down: Pin<DynPinId, FunctionSioInput, PullUp>,
    pub right_hand_thumb_middle: Pin<DynPinId, FunctionSioInput, PullUp>,

    pub start: Pin<DynPinId, FunctionSioInput, PullUp>,
}

impl ButtonInput {
    pub fn get_pin_state(&mut self) -> ButtonInputResults {
        ButtonInputResults {
            left_hand_pinky: self.left_hand_pinky.is_low().unwrap(),
            left_hand_ring: self.left_hand_ring.is_low().unwrap(),
            left_hand_middle: self.left_hand_middle.is_low().unwrap(),
            left_hand_index: self.left_hand_index.is_low().unwrap(),

            left_hand_middle_2: self.left_hand_middle_2.is_low().unwrap(),

            left_hand_thumb_left: self.left_hand_thumb_left.is_low().unwrap(),
            left_hand_thumb_right: self.left_hand_thumb_right.is_low().unwrap(),

            right_hand_index: self.right_hand_index.is_low().unwrap(),
            right_hand_middle: self.right_hand_middle.is_low().unwrap(),
            right_hand_ring: self.right_hand_ring.is_low().unwrap(),
            right_hand_pinky: self.right_hand_pinky.is_low().unwrap(),

            right_hand_index_2: self.right_hand_index_2.is_low().unwrap(),
            right_hand_middle_2: self.right_hand_middle_2.is_low().unwrap(),
            right_hand_ring_2: self.right_hand_ring_2.is_low().unwrap(),
            right_hand_pinky_2: self.right_hand_pinky_2.is_low().unwrap(),

            right_hand_thumb_left: self.right_hand_thumb_left.is_low().unwrap(),
            right_hand_thumb_right: self.right_hand_thumb_right.is_low().unwrap(),
            right_hand_thumb_up: self.right_hand_thumb_up.is_low().unwrap(),
            right_hand_thumb_down: self.right_hand_thumb_down.is_low().unwrap(),
            right_hand_thumb_middle: self.right_hand_thumb_middle.is_low().unwrap(),

            start: self.start.is_low().unwrap(),
        }
    }
}

pub struct ButtonInputResults {
    pub left_hand_pinky: bool,
    pub left_hand_ring: bool,
    pub left_hand_middle: bool,
    pub left_hand_index: bool,

    pub left_hand_middle_2: bool,

    pub left_hand_thumb_left: bool,
    pub left_hand_thumb_right: bool,

    pub right_hand_index: bool,
    pub right_hand_middle: bool,
    pub right_hand_ring: bool,
    pub right_hand_pinky: bool,

    pub right_hand_index_2: bool,
    pub right_hand_middle_2: bool,
    pub right_hand_ring_2: bool,
    pub right_hand_pinky_2: bool,

    pub right_hand_thumb_left: bool,
    pub right_hand_thumb_right: bool,
    pub right_hand_thumb_up: bool,
    pub right_hand_thumb_down: bool,
    pub right_hand_thumb_middle: bool,

    pub start: bool,
}
