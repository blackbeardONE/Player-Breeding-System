use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct BladeAIConfig {
    pub quest_log_threshold: Option<u32>,
    pub character_level_threshold: Option<u32>,
    pub gear_system_enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct GenreAgentConfig {
    pub wealth_threshold: Option<f64>,
    pub drop_rate_increase: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct MaoAIConfig {
    pub behavior_tracking_enabled: Option<bool>,
    pub clustering_algorithm: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IoanaAIConfig {
    pub quest_templates_path: Option<String>,
    pub difficulty_balancing_enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TogetherAIConfig {
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub blade_ai: Option<BladeAIConfig>,
    pub claire_agent: Option<GenreAgentConfig>,
    pub earnest_agent: Option<GenreAgentConfig>,
    pub sophie_agent: Option<GenreAgentConfig>,
    pub mao_ai: Option<MaoAIConfig>,
    pub ioana_ai: Option<IoanaAIConfig>,
    pub together_ai: Option<TogetherAIConfig>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, anyhow::Error> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
