use reqwest::Client;
use serde::{Deserialize, Serialize};
use log::{info, error};
use anyhow::{Result, Context};

const TOGETHER_API_URL: &str = "https://api.together.xyz/v1/chat/completions";
const TOGETHER_MODELS_URL: &str = "https://api.together.xyz/v1/models";

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: Option<u32>,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    index: u32,
    message: MessageContent,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MessageContent {
    role: String,
    content: String,
}

pub struct TogetherAIClient {
    api_key: String,
    client: Client,
}

impl TogetherAIClient {
    pub fn new(api_key: String) -> Self {
        let client = Client::new();
        Self { api_key, client }
    }

    pub async fn generate_text(&self, model: &str, prompt: &str, max_tokens: Option<u32>) -> Result<String> {
        let request_body = ChatCompletionRequest {
            model: model.to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            max_tokens,
        };

        let response = self.client.post(TOGETHER_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await
            .context("Failed to send request")?;

        if response.status().is_success() {
            let resp_text = response.text().await.context("Failed to read response text")?;
            info!("TogetherAIClient: Full response text: {}", resp_text);
            let resp_json: ChatCompletionResponse = serde_json::from_str(&resp_text).context("Failed to parse response JSON")?;
            if let Some(choice) = resp_json.choices.first() {
                info!("TogetherAIClient: Received response text");
                Ok(choice.message.content.clone())
            } else {
                error!("TogetherAIClient: No choices in response. Full response: {}", resp_text);
                Ok(String::new())
            }
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            error!("TogetherAIClient: API call failed with status {}: {}", status, text);
            Ok(String::new())
        }
    }
}

#[derive(Debug, Deserialize)]
struct ModelInfo {
    id: String,
    object: String,
    created: u64,
    owned_by: String,
    permission: Vec<serde_json::Value>,
    root: String,
    parent: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ModelsResponse {
    object: String,
    data: Vec<ModelInfo>,
}

impl TogetherAIClient {
    pub async fn get_models(&self) -> Result<Vec<ModelInfo>> {
        let response = self.client.get(TOGETHER_MODELS_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to send request to get models")?;

        if response.status().is_success() {
            let resp_text = response.text().await.context("Failed to read response text for models")?;
            info!("TogetherAIClient: Full models response text: {}", resp_text);
            let resp_json: ModelsResponse = serde_json::from_str(&resp_text).context("Failed to parse models response JSON")?;
            Ok(resp_json.data)
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            error!("TogetherAIClient: Get models API call failed with status {}: {}", status, text);
            Ok(vec![])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use crate::config::Config;

    #[tokio::test]
    async fn test_generate_text() {
        let config = Config::load("src/config.toml").expect("Failed to load config");
        let api_key = config.together_ai.expect("Missing together_ai config").api_key;
        let client = TogetherAIClient::new(api_key);
        let model = "deepseek-ai/DeepSeek-R1";
        let prompt = "Hello, Together AI!";
        let result = client.generate_text(model, prompt, Some(50)).await;
        match result {
            Ok(text) => {
                println!("Generated text: {}", text);
                assert!(!text.is_empty(), "Generated text should not be empty");
            }
            Err(e) => {
                panic!("API call failed: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_models() {
        let config = Config::load("src/config.toml").expect("Failed to load config");
        let api_key = config.together_ai.expect("Missing together_ai config").api_key;
        let client = TogetherAIClient::new(api_key);
        let result = client.get_models().await;
        match result {
            Ok(models) => {
                println!("Available models:");
                for model in models {
                    println!("Model ID: {}", model.id);
                }
                assert!(!models.is_empty(), "Models list should not be empty");
            }
            Err(e) => {
                panic!("Get models API call failed: {:?}", e);
            }
        }
    }
}
