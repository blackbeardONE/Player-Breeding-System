#[cfg(test)]
mod tests {
    use super::*;
    use player_breeding_system::communication::{BladeAIClient, PlayerMetadata};
    use player_breeding_system::orchestration::Orchestrator;
    use tokio;

    #[tokio::test]
    async fn test_blade_ai_classification() {
        let client = BladeAIClient::new("http://localhost:8000");
        let metadata = PlayerMetadata {
            quest_logs: 10,
            character_level: 5,
            gear_system: true,
            resource_management: false,
            army_building: false,
            territory_control: false,
            crafting_system: false,
            hunger_thirst_mechanics: false,
            permadeath_features: false,
        };

        let result = client.classify_game_type(&metadata).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.game_type.is_empty());
    }

    #[tokio::test]
    async fn test_orchestration_run() {
        let orchestrator = Orchestrator::new("http://localhost:8000");
        let metadata = PlayerMetadata {
            quest_logs: 12,
            character_level: 6,
            gear_system: true,
            resource_management: false,
            army_building: false,
            territory_control: false,
            crafting_system: false,
            hunger_thirst_mechanics: false,
            permadeath_features: false,
        };
        let player_wealth = 500.0;
        let behavior_data = player_breeding_system::communication::BehaviorData {
            feature_usage_counts: vec![
                ("Crafting Table".to_string(), 15),
                ("PvP Arena".to_string(), 5),
            ],
        };

        let result = orchestrator.run(metadata, player_wealth, behavior_data).await;
        assert!(result.is_ok());
    }
}
