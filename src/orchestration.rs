use anyhow::Result;
use crate::agents::blade_ai::{BladeAI, GameType, PlayerMetadata as BladePlayerMetadata};
use crate::communication::{GenreAgentClient, MaoAIClient, IoanaAIClient, PlayerMetadata, GenreAgentRequest, BehaviorData, QuestDesignRequest};
use log::{info, error};

use std::env;

pub struct Orchestrator {
    blade_ai: BladeAI,
    claire_client: GenreAgentClient,
    earnest_client: GenreAgentClient,
    sophie_client: GenreAgentClient,
    mao_ai_client: MaoAIClient,
    ioana_ai_client: IoanaAIClient,
}

impl Orchestrator {
    pub fn new() -> Self {
        let genre_agents_url = env::var("GENRE_AGENTS_URL").unwrap_or_else(|_| "http://genre-agents:8000".to_string());
        let earnest_agents_url = env::var("EARNEST_AGENTS_URL").unwrap_or_else(|_| "http://earnest-agents:8000".to_string());
        let sophie_agents_url = env::var("SOPHIE_AGENTS_URL").unwrap_or_else(|_| "http://sophie-agents:8000".to_string());
        let mao_ai_url = env::var("MAO_AI_URL").unwrap_or_else(|_| "http://mao-ai:8000".to_string());
        let ioana_ai_url = env::var("IOANA_AI_URL").unwrap_or_else(|_| "http://ioana-ai:8000".to_string());

        Orchestrator {
            blade_ai: BladeAI::new(10, 5, true),
            claire_client: GenreAgentClient::new(&genre_agents_url),
            earnest_client: GenreAgentClient::new(&earnest_agents_url),
            sophie_client: GenreAgentClient::new(&sophie_agents_url),
            mao_ai_client: MaoAIClient::new(&mao_ai_url),
            ioana_ai_client: IoanaAIClient::new(&ioana_ai_url),
        }
    }

    pub async fn run(&self, metadata: PlayerMetadata, player_wealth: f64, behavior_data: BehaviorData) -> Result<()> {
        // Convert PlayerMetadata to BladePlayerMetadata for classification
        let blade_metadata = BladePlayerMetadata {
            quest_logs: Some(metadata.quest_logs),
            character_level: Some(metadata.character_level),
            gear_system: Some(metadata.gear_system),
            resource_management: Some(metadata.resource_management),
            army_building: Some(metadata.army_building),
            territory_control: Some(metadata.territory_control),
            crafting_system: Some(metadata.crafting_system),
            hunger_thirst_mechanics: Some(metadata.hunger_thirst_mechanics),
            permadeath_features: Some(metadata.permadeath_features),
        };

        // Step 1: Classify game type using blade_ai module directly
        let game_type = self.blade_ai.classify_game_type(&blade_metadata);
        info!("Game type classified as: {:?}", game_type);

        // Step 2: Adjust drop rates based on player wealth for genre agents
        let genre_request = GenreAgentRequest { player_wealth };
        let claire_resp = self.claire_client.adjust_drop_rate(&genre_request).await?;
        let earnest_resp = self.earnest_client.adjust_drop_rate(&genre_request).await?;
        let sophie_resp = self.sophie_client.adjust_drop_rate(&genre_request).await?;
        info!("Claire drop rate adjustment: {}", claire_resp.drop_rate_adjustment);
        info!("Earnest drop rate adjustment: {}", earnest_resp.drop_rate_adjustment);
        info!("Sophie drop rate adjustment: {}", sophie_resp.drop_rate_adjustment);

        // Step 3: Analyze player behavior
        let behavior_profile = self.mao_ai_client.analyze_behavior(&behavior_data).await?;
        info!("Behavior profile: {}", behavior_profile.profile_name);

        // Step 4: Design quest based on stats and behavior
        let quest_request = QuestDesignRequest {
            player_stats: metadata,
            behavior_profile,
        };
        let quest_response = self.ioana_ai_client.design_quest(&quest_request).await?;
        info!("Quest designed: {}", quest_response.quest_template);

        Ok(())
    }
}
