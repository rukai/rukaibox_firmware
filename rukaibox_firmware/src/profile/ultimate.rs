use crate::{
    input::{ButtonInputLogical, ButtonInputResults},
    socd::{SocdState, SocdType},
};
use joybus_pio::GamecubeInput;
use rukaibox_config::{LogicalButtonToPhysicalButton, Profile};

pub struct UltimateMapping {
    pub socd_state: SocdState,
    pub socd_type: SocdType,
    pub button_mapping: LogicalButtonToPhysicalButton,
}

impl UltimateMapping {
    pub fn new(profile: &Profile) -> Self {
        let socd_type = match profile.socd {
            rukaibox_config::SocdType::SecondInputPriority => SocdType::SecondInputPriority,
            rukaibox_config::SocdType::Neutral => SocdType::Neutral,
        };

        UltimateMapping {
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

        // Up-front queries

        let horizontal = stick_left || stick_right;
        let vertical = stick_up || stick_down;
        let diagonal = horizontal && vertical;

        let cstick_horizontal = cstick_left || cstick_right;
        let cstick_vertical = cstick_up || cstick_down;

        let shield = l_digital || r_digital;

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

        // Derive main stick offsets

        let (stick_x_offset, stick_y_offset) = if mod_x {
            if diagonal {
                if b {
                    if cstick_right {
                        (stick_x_direction * 67, stick_y_direction * 35)
                    } else if cstick_up {
                        (stick_x_direction * 67, stick_y_direction * 39)
                    } else if cstick_left {
                        (stick_x_direction * 67, stick_y_direction * 49)
                    } else if cstick_down {
                        (stick_x_direction * 67, stick_y_direction * 55)
                    } else {
                        (stick_x_direction * 67, stick_y_direction * 44)
                    }
                } else if cstick_right {
                    (stick_x_direction * 53, stick_y_direction * 28)
                } else if cstick_up {
                    (stick_x_direction * 53, stick_y_direction * 31)
                } else if cstick_left {
                    (stick_x_direction * 53, stick_y_direction * 39)
                } else if cstick_down {
                    (stick_x_direction * 53, stick_y_direction * 43)
                } else if shield {
                    (stick_x_direction * 51, stick_y_direction * 30)
                } else if a {
                    (stick_x_direction * 70, stick_y_direction * 34)
                } else {
                    (stick_x_direction * 53, stick_y_direction * 35)
                }
            } else {
                let x_offset = if horizontal {
                    if b {
                        stick_x_direction * 66
                    } else if shield {
                        stick_x_direction * 51
                    } else {
                        stick_x_direction * 53
                    }
                } else {
                    0
                };
                let y_offset = if vertical {
                    if shield {
                        stick_y_direction * 51
                    } else {
                        stick_y_direction * 60
                    }
                } else {
                    0
                };
                (x_offset, y_offset)
            }
        } else if mod_y {
            if diagonal {
                if shield {
                    if stick_left {
                        (stick_x_direction * 40, stick_y_direction * 68)
                    } else {
                        (stick_x_direction * 38, stick_y_direction * 70)
                    }
                } else if a {
                    (stick_x_direction * 38, stick_y_direction * 69)
                } else if b {
                    if cstick_right {
                        (stick_x_direction * 35, stick_y_direction * 67)
                    } else if cstick_up {
                        (stick_x_direction * 39, stick_y_direction * 67)
                    } else if cstick_left {
                        (stick_x_direction * 49, stick_y_direction * 67)
                    } else if cstick_down {
                        (stick_x_direction * 55, stick_y_direction * 67)
                    } else {
                        (stick_x_direction * 44, stick_y_direction * 67)
                    }
                } else if cstick_right {
                    (stick_x_direction * 28, stick_y_direction * 53)
                } else if cstick_up {
                    (stick_x_direction * 31, stick_y_direction * 53)
                } else if cstick_left {
                    (stick_x_direction * 49, stick_y_direction * 53)
                } else if cstick_down {
                    (stick_x_direction * 43, stick_y_direction * 53)
                } else {
                    (stick_x_direction * 35, stick_y_direction * 53)
                }
            } else {
                let x_offset = if horizontal {
                    stick_x_direction * 35
                } else {
                    0
                };
                let y_offset = if vertical {
                    if a {
                        stick_y_direction * 36
                    } else {
                        stick_y_direction * 53
                    }
                } else {
                    0
                };
                (x_offset, y_offset)
            }
        } else {
            (stick_x_direction * 100, stick_y_direction * 100)
        };

        let stick_x = (128 + stick_x_offset) as u8;
        let stick_y = (128 + stick_y_offset) as u8;

        // Derive C-stick offsets
        // ASDI slideoff overrides angled fsmash, so check it first.

        let (cstick_x_offset, cstick_y_offset) = if cstick_horizontal && cstick_vertical {
            (cstick_x_direction * 42, cstick_y_direction * 68)
        } else if mod_x && cstick_horizontal {
            (cstick_x_direction * 65, stick_y_direction * 44)
        } else {
            (cstick_x_direction * 100, cstick_y_direction * 100)
        };

        let cstick_x = (128 + cstick_x_offset) as u8;
        let cstick_y = (128 + cstick_y_offset) as u8;

        // Derive analog trigger values
        // also set analog trigger on digital trigger input, since ultimate ignores digital triggers this will probably do what the user intended.

        let l_analog = if l_digital || l_analog { 140 } else { 0 };
        let r_analog = if r_digital || r_analog { 140 } else { 0 };

        // Derive dpad values

        let dpad_up = (mod_x && mod_y && cstick_up) || dpad_up;
        let dpad_down = (mod_x && mod_y && cstick_down) || dpad_down;
        let dpad_left = (mod_x && mod_y && cstick_left) || dpad_left;
        let dpad_right = (mod_x && mod_y && cstick_right) || dpad_right;

        // Disable C-stick when dpad layer active

        let cstick_x = if dpad_left || dpad_right {
            128
        } else {
            cstick_x
        };
        let cstick_y = if dpad_up || dpad_down { 128 } else { cstick_y };

        // Suppress start when mod_x or mod_y

        let start = start && !(mod_x || mod_y);

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
