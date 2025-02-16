mod project_plus;

use crate::input::ButtonInputResults;
use joybus_pio::GamecubeInput;
use project_plus::ProjectPlusMapping;
use rukaibox_config::{ArchivedBaseLogic, ArchivedConfig, ArchivedProfile};

pub enum MapProfile {
    ProjectPlus(ProjectPlusMapping),
    // TODO: rivals mapping
    Rivals2(ProjectPlusMapping),
}

impl MapProfile {
    pub fn new(config: &ArchivedProfile) -> Self {
        match config.logic {
            ArchivedBaseLogic::ProjectPlus => {
                MapProfile::ProjectPlus(ProjectPlusMapping::new(config))
            }
            ArchivedBaseLogic::Rivals2 => MapProfile::ProjectPlus(ProjectPlusMapping::new(config)),
        }
    }

    pub fn map_to_gamecube(&mut self, input: &ButtonInputResults) -> GamecubeInput {
        match self {
            MapProfile::ProjectPlus(x) => x.map_to_gamecube(input),
            MapProfile::Rivals2(x) => x.map_to_gamecube(input),
        }
    }

    pub fn change_profile(&mut self, input: &ButtonInputResults, config: &ArchivedConfig) {
        for profile in config.profiles.iter() {
            if !profile.activation_combination.is_empty() {
                // TODO: implement actual logic here!!!!!
                if input.left_hand_pinky && input.start {
                    *self = MapProfile::Rivals2(ProjectPlusMapping::new(profile))
                }
            }
        }
    }
}
