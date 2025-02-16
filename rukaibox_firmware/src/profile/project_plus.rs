use crate::{
    input::ButtonInputResults,
    socd::{SocdState, SocdType},
};
use joybus_pio::GamecubeInput;
use rukaibox_config::{ArchivedProfile, ArchivedSocdType};

pub struct ProjectPlusMapping {
    pub socd_state: SocdState,
    pub socd_type: SocdType,
}

impl ProjectPlusMapping {
    pub fn new(profile: &ArchivedProfile) -> Self {
        let socd_type = match profile.socd {
            ArchivedSocdType::SecondInputPriority => SocdType::SecondInputPriority,
            ArchivedSocdType::Neutral => SocdType::Neutral,
        };

        ProjectPlusMapping {
            socd_state: Default::default(),
            socd_type,
        }
    }

    pub fn map_to_gamecube(&mut self, input: &ButtonInputResults) -> GamecubeInput {
        let start = input.start;

        let mod_x = input.left_hand_thumb_left;
        let mod_y = input.left_hand_thumb_right;

        let a = input.right_hand_thumb_middle;
        let b = input.right_hand_index;
        let x = input.right_hand_middle;
        let y = input.right_hand_middle_2;
        let z = input.right_hand_ring;
        let dpad_up_shortcut = input.right_hand_pinky_2;
        let r_analog = input.right_hand_ring_2;
        let l_digital = input.left_hand_pinky;
        let r_digital = input.right_hand_index_2;

        let stick_left = input.left_hand_ring;
        let stick_right = input.left_hand_index;
        let stick_up = input.right_hand_pinky || input.left_hand_middle_2;
        let stick_down = input.left_hand_middle;

        let cstick_left = input.right_hand_thumb_left;
        let cstick_right = input.right_hand_thumb_right;
        let cstick_up = input.right_hand_thumb_up;
        let cstick_down = input.right_hand_thumb_down;

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

        // Derive dpad values

        let dpad_up = (mod_x && mod_y && cstick_up) || dpad_up_shortcut;
        let dpad_down = mod_x && mod_y && cstick_down;
        let dpad_left = mod_x && mod_y && cstick_left;
        let dpad_right = mod_x && mod_y && cstick_right;

        // disable cstick when dpad in use
        let cstick_x = if dpad_left || dpad_right {
            128
        } else {
            cstick_x
        };
        let cstick_y = if dpad_up || dpad_down { 128 } else { cstick_y };

        GamecubeInput {
            start,
            a,
            b,
            x,
            y,
            z,
            dpad_up,
            dpad_down,
            dpad_left,
            dpad_right,
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
