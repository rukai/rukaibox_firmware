use crate::{
    input::{ButtonInputLogical, ButtonInputResults},
    socd::{SocdState, SocdType},
};
use joybus_pio::GamecubeInput;
use rukaibox_config::{LogicalButtonToPhysicalButton, Profile};

pub struct Rivals2Mapping {
    pub socd_state: SocdState,
    pub socd_type: SocdType,
    pub button_mapping: LogicalButtonToPhysicalButton,
}

impl Rivals2Mapping {
    pub fn new(profile: &Profile) -> Self {
        let socd_type = match profile.socd {
            rukaibox_config::SocdType::SecondInputPriority => SocdType::SecondInputPriority,
            rukaibox_config::SocdType::Neutral => SocdType::Neutral,
        };

        Rivals2Mapping {
            button_mapping: profile.buttons.clone(),
            socd_state: Default::default(),
            socd_type,
        }
    }

    pub fn map_to_gamecube(&mut self, input: &ButtonInputResults) -> GamecubeInput {
        let ButtonInputLogical {
            mod_x,
            mod_y,
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
            l_analog,
            r_analog,
            stick_left,
            stick_right,
            stick_up,
            stick_down,
            cstick_left,
            cstick_right,
            cstick_up,
            cstick_down,
        } = input.to_gc(&self.button_mapping);

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

        let shield = l_digital || r_digital;

        // Derive stick values, applying modifiers

        // Values taken from: https://github.com/JonnyHaystack/HayBox/blob/52188f41209a18c03e0c1d151679c32025a48962/src/modes/Rivals2.cpp#L109
        // cstick modifiers on up b left out since they seem zetterburn specific and should probably go in a character specific profile.
        let (stick_x_offset, stick_y_offset) = if mod_x {
            if diagonal && !shield {
                if a {
                    // angled tilts
                    ((stick_x_direction * 69), (stick_y_direction * 53))
                } else if z {
                    // shortest up B
                    // (x, y), (53, 68), (~0.31, ~0.188) [coords, code_values, in-game values]
                    ((stick_x_direction * 53), (stick_y_direction * 42))
                } else if b {
                    // 100% up B (just hold B)
                    // (x, y), (123, 51), (1.14~, 0.29~) [coords, code_values, in-game values]
                    ((stick_x_direction * 123), (stick_y_direction * 51))
                } else {
                    // 60% up B (just release B)
                    // (x, y), (68, 42), (~0.49, ~0.188) [coords, code_values, in-game values]
                    ((stick_x_direction * 68), (stick_y_direction * 42))
                }
            } else if diagonal && shield {
                //for max-length diagonal wavedash while holding ModX
                ((stick_x_direction * 76), (stick_y_direction * 42))
            } else if vertical {
                // 48 (0.31~ in-game), 0.3 allows tilts and shield drop
                (0, (stick_y_direction * 53))
            } else if horizontal {
                //76 gives 0.58~ in-game for a medium speed walk. will also do tilts
                ((stick_x_direction * 76), 0)
            } else {
                (0, 0)
            }
        } else if mod_y {
            if diagonal && !shield {
                if z {
                    // shortest up B
                    // (x, y), (42, 53), (~0.188, ~0.31) [coords, code_values, in-game values]
                    ((stick_x_direction * 42), (stick_y_direction * 53))
                } else if b {
                    // 100% up B (just hold B)
                    // (x, y), (51, 123), (~0.29, ~1.14) [coords, code_values, in-game values]
                    ((stick_x_direction * 51), (stick_y_direction * 123))
                } else {
                    // 60% up B (just release B)
                    // (x, y), (42, 68), (~0.188, ~0.49) [coords, code_values, in-game values]
                    ((stick_x_direction * 42), (stick_y_direction * 68))
                }
            } else if vertical {
                // 0.75~ in-game. will shield drop and tap jump; will not fast fall
                (0, (stick_y_direction * 90))
            } else if horizontal {
                //53 equates to 0.318~ in-game. 0.3 is min to achieve a walk
                ((stick_x_direction * 53), 0)
            } else {
                (0, 0)
            }
        } else if diagonal && shield {
            // (0.77~, 0.77~) to prevent spot dodging when pressing diagonal on the ground
            ((stick_x_direction * 92), (stick_y_direction * 92))
        } else if diagonal && !shield {
            //added this conditional to give joystick accurate diagonals rather than (+/- 1.2, 1.2) should be (0.87~, 0.87~)
            (
                // (0.78 in-game), reduced below 0.8 to allow crouch tilts/crouch turn-around tilts
                (stick_x_direction * 92),
                // 0.83 in-game >0.8 allows fast fall
                (stick_y_direction * 96),
            )
        } else {
            // when only stick inputs, set to maximum
            ((stick_x_direction * 127), (stick_y_direction * 127))
        };
        let stick_x = (128 + stick_x_offset) as u8;
        let stick_y = (128 + stick_y_offset) as u8;

        // Derive C stick values

        let (cstick_x_offset, cstick_y_offset) = if mod_x && cstick_horizontal {
            // Allow for angled smash attacks
            ((cstick_x_direction * 65), (stick_y_direction * 23))
        } else {
            ((cstick_x_direction * 127), (cstick_y_direction * 127))
        };
        let cstick_x = (128 + cstick_x_offset) as u8;
        let cstick_y = (128 + cstick_y_offset) as u8;

        // Derive analog trigger values

        let l_analog = if l_analog { 49 } else { 0 };
        let r_analog = if r_analog { 49 } else { 0 };

        // Derive dpad values

        let dpad_up = (mod_x && mod_y && cstick_up) || dpad_up;
        let dpad_down = mod_x && mod_y && cstick_down || dpad_down;
        let dpad_left = mod_x && mod_y && cstick_left || dpad_left;
        let dpad_right = mod_x && mod_y && cstick_right || dpad_right;

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
