use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerMetadata {
    pub quest_logs: u32,
    pub character_level: u32,
    pub gear_system: bool,
    pub resource_management: bool,
    pub army_building: bool,
    pub territory_control: bool,
    pub crafting_system: bool,
    pub hunger_thirst_mechanics: bool,
    pub permadeath_features: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameTypeResponse {
    pub game_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenreAgentRequest {
    pub player_wealth: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenreAgentResponse {
    pub drop_rate_adjustment: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BehaviorData {
    pub feature_usage_counts: Vec<(String, u32)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BehaviorProfile {
    pub profile_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestDesignRequest {
    pub player_stats: PlayerMetadata,
    pub behavior_profile: BehaviorProfile,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestDesignResponse {
    pub quest_template: String,
}

pub struct BladeAIClient {
    client: Client,
    base_url: String,
}

impl BladeAIClient {
    pub fn new(base_url: &str) -> Self {
        BladeAIClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn classify_game_type(&self, metadata: &PlayerMetadata) -> Result<GameTypeResponse> {
        let url = format!("{}/blade_ai/classify_game_type/", self.base_url);
        let resp = self.client.post(&url)
            .json(metadata)
            .send()
            .await?
            .json::<GameTypeResponse>()
            .await?;
        Ok(resp)
    }
}

pub struct GenreAgentClient {
    client: Client,
    base_url: String,
}

impl GenreAgentClient {
    pub fn new(base_url: &str) -> Self {
        GenreAgentClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn adjust_drop_rate(&self, request: &GenreAgentRequest) -> Result<GenreAgentResponse> {
        let url = format!("{}/genre_agents/adjust_drop_rate/", self.base_url);
        let resp = self.client.post(&url)
            .json(request)
            .send()
            .await?
            .json::<GenreAgentResponse>()
            .await?;
        Ok(resp)
    }
}

pub struct MaoAIClient {
    client: Client,
    base_url: String,
}

impl MaoAIClient {
    pub fn new(base_url: &str) -> Self {
        MaoAIClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn analyze_behavior(&self, data: &BehaviorData) -> Result<BehaviorProfile> {
        let url = format!("{}/mao_ai/analyze_behavior/", self.base_url);
        let resp = self.client.post(&url)
            .json(data)
            .send()
            .await?
            .json::<BehaviorProfile>()
            .await?;
        Ok(resp)
    }
}

pub struct IoanaAIClient {
    client: Client,
    base_url: String,
}

impl IoanaAIClient {
    pub fn new(base_url: &str) -> Self {
        IoanaAIClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn design_quest(&self, request: &QuestDesignRequest) -> Result<QuestDesignResponse> {
        let url = format!("{}/ioana_ai/design_quest/", self.base_url);
        let resp = self.client.post(&url)
            .json(request)
            .send()
            .await?
            .json::<QuestDesignResponse>()
            .await?;
        Ok(resp)
    }
}
