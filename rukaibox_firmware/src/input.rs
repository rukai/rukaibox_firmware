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
