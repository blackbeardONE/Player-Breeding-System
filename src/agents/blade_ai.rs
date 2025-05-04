use log::info;

#[derive(Debug)]
pub struct PlayerMetadata {
    pub quest_logs: Option<u32>,
    pub character_level: Option<u32>,
    pub gear_system: Option<bool>,
    pub resource_management: Option<bool>,
    pub army_building: Option<bool>,
    pub territory_control: Option<bool>,
    pub crafting_system: Option<bool>,
    pub hunger_thirst_mechanics: Option<bool>,
    pub permadeath_features: Option<bool>,
}

#[derive(Debug, PartialEq)]
pub enum GameType {
    MMORPG,
    MMORTS,
    MMO_Survival,
    Unknown,
}

pub struct BladeAI {
    quest_log_threshold: u32,
    character_level_threshold: u32,
    gear_system_enabled: bool,
}

impl BladeAI {
    pub fn new(quest_log_threshold: u32, character_level_threshold: u32, gear_system_enabled: bool) -> Self {
        Self {
            quest_log_threshold,
            character_level_threshold,
            gear_system_enabled,
        }
    }

    pub fn classify_game_type(&self, metadata: &PlayerMetadata) -> GameType {
        // Check MMORPG criteria
        if let (Some(quest_logs), Some(level), Some(gear)) = (metadata.quest_logs, metadata.character_level, metadata.gear_system) {
            if quest_logs >= self.quest_log_threshold &&
               level >= self.character_level_threshold &&
               gear == self.gear_system_enabled {
                info!("BladeAI: Classified as MMORPG");
                return GameType::MMORPG;
            }
        }

        // Check MMORTS criteria
        if let (Some(resource), Some(army), Some(territory)) = (metadata.resource_management, metadata.army_building, metadata.territory_control) {
            if resource && army && territory {
                info!("BladeAI: Classified as MMORTS");
                return GameType::MMORTS;
            }
        }

        // Check MMO Survival criteria
        if let (Some(crafting), Some(hunger), Some(permadeath)) = (metadata.crafting_system, metadata.hunger_thirst_mechanics, metadata.permadeath_features) {
            if crafting && hunger && permadeath {
                info!("BladeAI: Classified as MMO Survival");
                return GameType::MMO_Survival;
            }
        }

        info!("BladeAI: Game type unknown");
        GameType::Unknown
    }
}
