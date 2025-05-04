use log::info;
use std::collections::HashMap;

pub struct IoanaAI {
    quest_templates_path: String,
    difficulty_balancing_enabled: bool,
    active_quests: HashMap<String, String>, // player_id -> quest
}

impl IoanaAI {
    pub fn new(quest_templates_path: String, difficulty_balancing_enabled: bool) -> Self {
        Self {
            quest_templates_path,
            difficulty_balancing_enabled,
            active_quests: HashMap::new(),
        }
    }

    pub fn assign_quest(&mut self, player_id: &str, quest: &str) {
        self.active_quests.insert(player_id.to_string(), quest.to_string());
        info!("IoanaAI: Assigned quest '{}' to player '{}'", quest, player_id);
    }

    pub fn balance_difficulty(&self, player_performance: f64) -> f64 {
        if self.difficulty_balancing_enabled {
            // Simple example: scale difficulty inversely with performance
            let difficulty = 1.0 / (1.0 + player_performance);
            info!("IoanaAI: Balanced difficulty to {}", difficulty);
            difficulty
        } else {
            1.0
        }
    }
}
