use tokio;
use flexi_logger::Logger;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use log::{info, error};

const TOGETHER_API_URL: &str = "https://api.together.ai/v1/ai";

#[derive(Debug, Serialize)]
struct TogetherAIRequest {
    model: String,
    prompt: String,
    max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct TogetherAIResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    text: String,
    index: u32,
    logprobs: Option<serde_json::Value>,
    finish_reason: Option<String>,
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

    pub async fn generate_text(&self, model: &str, prompt: &str, max_tokens: Option<u32>) -> Result<String, reqwest::Error> {
        let request_body = TogetherAIRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            max_tokens,
        };

        let response = self.client.post(TOGETHER_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let resp_text = response.text().await?;
            info!("TogetherAIClient: Full response text: {}", resp_text);
            let resp_json: TogetherAIResponse = serde_json::from_str(&resp_text).unwrap();
            if let Some(choice) = resp_json.choices.first() {
                info!("TogetherAIClient: Received response text");
                Ok(choice.text.clone())
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

#[tokio::test]
async fn test_generate_text() {
    Logger::try_with_env_or_str("info").unwrap().start().unwrap();
    let api_key = "56c8eeff9971269d7a7e625ff88e8a83a34a556003a5c87c289ebe9a3d8a3d2c".to_string();
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
