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
        // Query pins for input

        let start = self.input.start.is_low().unwrap();

        let mod_x = self.input.left_hand_thumb_left.is_low().unwrap();
        let mod_y = self.input.left_hand_thumb_right.is_low().unwrap();

        let a = self.input.right_hand_thumb_middle.is_low().unwrap();
        let b = self.input.right_hand_index.is_low().unwrap();
        let x = self.input.right_hand_middle.is_low().unwrap();
        let y = self.input.right_hand_middle_2.is_low().unwrap();
        let z = self.input.right_hand_ring.is_low().unwrap();
        let dpad_up = self.input.right_hand_pinky_2.is_low().unwrap();
        let r_analog = self.input.right_hand_ring_2.is_low().unwrap();
        let l_digital = self.input.left_hand_pinky.is_low().unwrap();
        let r_digital = self.input.right_hand_index_2.is_low().unwrap();

        let stick_left = self.input.left_hand_ring.is_low().unwrap();
        let stick_right = self.input.left_hand_index.is_low().unwrap();
        let stick_up = self.input.right_hand_pinky.is_low().unwrap()
            || self.input.left_hand_middle_2.is_low().unwrap();
        let stick_down = self.input.left_hand_middle.is_low().unwrap();

        let cstick_left = self.input.right_hand_thumb_left.is_low().unwrap();
        let cstick_right = self.input.right_hand_thumb_right.is_low().unwrap();
        let cstick_up = self.input.right_hand_thumb_up.is_low().unwrap();
        let cstick_down = self.input.right_hand_thumb_down.is_low().unwrap();

        // Resolve SOCD

        let (stick_left, stick_right) = self.socd_type.resolve(
            stick_left,
            stick_right,
            &mut self.socd_state.prev_left,
            &mut self.socd_state.prev_right,
        );

        let (stick_up, stick_down) = self.socd_type.resolve(
            stick_up,
            stick_down,
            &mut self.socd_state.prev_up,
            &mut self.socd_state.prev_down,
        );

        let (cstick_left, cstick_right) = self.socd_type.resolve(
            cstick_left,
            cstick_right,
            &mut self.socd_state.prev_cstick_left,
            &mut self.socd_state.prev_cstick_right,
        );

        let (cstick_up, cstick_down) = self.socd_type.resolve(
            cstick_up,
            cstick_down,
            &mut self.socd_state.prev_cstick_up,
            &mut self.socd_state.prev_cstick_down,
        );

        // Some up front queries

        let horizontal = stick_left || stick_right;
        let vertical = stick_up || stick_down;
        let diagonal = horizontal && vertical;

        let cstick_horizontal = cstick_left || cstick_right;

        let stick_x_direction: i16 = match (stick_left, stick_right) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };
        let stick_y_direction: i16 = match (stick_down, stick_up) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };
        let cstick_x_direction: i16 = match (cstick_left, cstick_right) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };
        let cstick_y_direction: i16 = match (cstick_down, cstick_up) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };

        // Derive stick values, applying modifiers

        let (stick_x_offset, stick_y_offset) = if mod_x {
            if diagonal {
                if cstick_right {
                    ((stick_x_direction * 72), (stick_y_direction * 61))
                } else if cstick_left {
                    ((stick_x_direction * 84), (stick_y_direction * 50))
                } else if cstick_down {
                    ((stick_x_direction * 82), (stick_y_direction * 36))
                } else if cstick_up {
                    ((stick_x_direction * 77), (stick_y_direction * 55))
                } else if r_digital {
                    ((stick_x_direction * 82), (stick_y_direction * 35))
                } else if b {
                    ((stick_x_direction * 85), (stick_y_direction * 31))
                } else {
                    ((stick_x_direction * 70), (stick_y_direction * 34))
                }
            } else if vertical {
                (0, (stick_y_direction * 60))
            } else if horizontal {
                ((stick_x_direction * 70), 0)
            } else {
                (0, 0)
            }
        } else if mod_y {
            if diagonal {
                if cstick_right {
                    ((stick_x_direction * 62), (stick_y_direction * 72))
                } else if cstick_left {
                    ((stick_x_direction * 40), (stick_y_direction * 84))
                } else if cstick_down {
                    ((stick_x_direction * 34), (stick_y_direction * 82))
                } else if cstick_up {
                    ((stick_x_direction * 55), (stick_y_direction * 77))
                } else if r_digital {
                    ((stick_x_direction * 51), (stick_y_direction * 82))
                } else if b {
                    ((stick_x_direction * 28), (stick_y_direction * 85))
                } else {
                    ((stick_x_direction * 28), (stick_y_direction * 58))
                }
            } else if vertical {
                (0, (stick_y_direction * 70))
            } else if horizontal {
                ((stick_x_direction * 35), 0)
            } else {
                (0, 0)
            }
        } else if diagonal && stick_up {
            ((stick_x_direction * 83), (stick_y_direction * 93))
        } else {
            ((stick_x_direction * 100), (stick_y_direction * 100))
        };
        let stick_x = (128 + stick_x_offset) as u8;
        let stick_y = (128 + stick_y_offset) as u8;

        // TODO: Cstick ASDI slideoff angle overrides?

        // TODO: ledgedash SOCD override?

        // Derive C stick values

        let (cstick_x_offset, cstick_y_offset) = if mod_x && cstick_horizontal {
            // Allow for angled smash attacks
            ((cstick_x_direction * 65), (stick_y_direction * 23))
        } else {
            ((cstick_x_direction * 100), (cstick_y_direction * 100))
        };
        let cstick_x = (128 + cstick_x_offset) as u8;
        let cstick_y = (128 + cstick_y_offset) as u8;

        // Derive analog trigger values

        let l_analog = 0;
        let r_analog = if r_analog { 49 } else { 0 };

        GamecubeInput {
            start,
            a,
            b,
            x,
            y,
            z,
            dpad_up,
            dpad_down: false,
            dpad_left: false,
            dpad_right: false,
            l_digital,
            r_digital,
            stick_x,
            stick_y,
            cstick_x,
            cstick_y,
            l_analog,
            r_analog,
        }
    }
}
