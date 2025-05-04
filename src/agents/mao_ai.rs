use log::info;
use std::collections::HashMap;

pub struct MaoAI {
    behavior_tracking_enabled: bool,
    clustering_algorithm: String,
    feature_usage_counts: HashMap<String, u32>,
}

impl MaoAI {
    pub fn new(behavior_tracking_enabled: bool, clustering_algorithm: String) -> Self {
        Self {
            behavior_tracking_enabled,
            clustering_algorithm,
            feature_usage_counts: HashMap::new(),
        }
    }

    pub fn track_feature_usage(&mut self, feature_name: &str) {
        if self.behavior_tracking_enabled {
            let count = self.feature_usage_counts.entry(feature_name.to_string()).or_insert(0);
            *count += 1;
            info!("MaoAI: Tracked usage of feature '{}', count now {}", feature_name, count);
        }
    }

    pub fn get_player_profile(&self) -> String {
        // Placeholder for clustering logic
        // For MVP, return a simple profile based on most used feature
        if let Some((feature, _)) = self.feature_usage_counts.iter().max_by_key(|entry| entry.1) {
            info!("MaoAI: Player profile based on feature '{}'", feature);
            feature.clone()
        } else {
            "Unknown".to_string()
        }
    }
}
