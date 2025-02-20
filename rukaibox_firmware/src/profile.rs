mod project_plus;
mod rivals2;

use crate::input::ButtonInputResults;
use joybus_pio::GamecubeInput;
use project_plus::ProjectPlusMapping;
use rivals2::Rivals2Mapping;
use rukaibox_config::{BaseLogic, Config, Profile};

pub enum MapProfile {
    ProjectPlus(ProjectPlusMapping),
    // TODO: rivals mapping
    Rivals2(Rivals2Mapping),
}

impl MapProfile {
    pub fn new(config: &Profile) -> Self {
        match config.logic {
            BaseLogic::ProjectPlus => MapProfile::ProjectPlus(ProjectPlusMapping::new(config)),
            BaseLogic::Rivals2 => MapProfile::Rivals2(Rivals2Mapping::new(config)),
        }
    }

    pub fn map_to_gamecube(&mut self, input: &ButtonInputResults) -> GamecubeInput {
        match self {
            MapProfile::ProjectPlus(x) => x.map_to_gamecube(input),
            MapProfile::Rivals2(x) => x.map_to_gamecube(input),
        }
    }

    pub fn change_profile(&mut self, input: &ButtonInputResults, config: &Config) {
        'next_profile: for profile in config.profiles.iter() {
            for check in profile.activation_combination.iter() {
                if !input.get_button_value(*check) {
                    continue 'next_profile;
                }
            }

            *self = Self::new(profile);
            // immediately return to avoid triggering any other changes.
            return;
        }
    }
}
