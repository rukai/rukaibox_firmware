use embedded_hal::digital::InputPin;

use crate::{
    gamecube_controller::GamecubeInput,
    input::ButtonInput,
    socd::{SocdState, SocdType},
};

pub struct ProjectPlusMapping {
    pub input: ButtonInput,
    pub socd_state: SocdState,
    pub socd_type: SocdType,
}

impl ProjectPlusMapping {
    pub fn new(input: ButtonInput) -> Self {
        ProjectPlusMapping {
            input,
            socd_state: Default::default(),
            socd_type: SocdType::SecondInputPriority,
        }
    }

    pub fn poll(&mut self) -> GamecubeInput {
        let stick_left = self.input.left_hand_ring.is_low().unwrap();
        let stick_right = self.input.left_hand_index.is_low().unwrap();
        let stick_x = match self.socd_type.resolve(
            stick_left,
            stick_right,
            &mut self.socd_state.prev_left,
            &mut self.socd_state.prev_right,
        ) {
            (true, false) => 0,
            (false, true) => 255,
            _ => 128,
        };

        let stick_up = self.input.right_hand_pinky.is_low().unwrap()
            || self.input.left_hand_middle_2.is_low().unwrap();
        let stick_down = self.input.left_hand_middle.is_low().unwrap();
        let stick_y = match self.socd_type.resolve(
            stick_up,
            stick_down,
            &mut self.socd_state.prev_up,
            &mut self.socd_state.prev_down,
        ) {
            (true, false) => 255,
            (false, true) => 0,
            _ => 128,
        };
        let cstick_x = match self.socd_type.resolve(
            self.input.right_hand_thumb_left.is_low().unwrap(),
            self.input.right_hand_thumb_right.is_low().unwrap(),
            &mut self.socd_state.prev_cstick_left,
            &mut self.socd_state.prev_cstick_right,
        ) {
            (true, false) => 0,
            (false, true) => 255,
            _ => 128,
        };
        let cstick_y = match self.socd_type.resolve(
            self.input.right_hand_thumb_up.is_low().unwrap(),
            self.input.right_hand_thumb_down.is_low().unwrap(),
            &mut self.socd_state.prev_cstick_up,
            &mut self.socd_state.prev_cstick_down,
        ) {
            (true, false) => 255,
            (false, true) => 0,
            _ => 128,
        };

        GamecubeInput {
            start: self.input.start.is_low().unwrap(),
            a: self.input.right_hand_thumb_middle.is_low().unwrap(),
            b: self.input.right_hand_index.is_low().unwrap(),
            x: self.input.right_hand_middle.is_low().unwrap(),
            y: self.input.right_hand_middle_2.is_low().unwrap(),
            z: self.input.right_hand_ring.is_low().unwrap(),
            dpad_up: self.input.right_hand_pinky_2.is_low().unwrap(),
            dpad_down: self.input.right_hand_ring_2.is_low().unwrap(),
            dpad_left: false,
            dpad_right: false,
            l_digital: self.input.left_hand_pinky.is_low().unwrap(),
            r_digital: self.input.right_hand_index_2.is_low().unwrap(),
            stick_x,
            stick_y,
            cstick_x,
            cstick_y,
            l_analog: 0,
            r_analog: 0,
        }
    }
}
