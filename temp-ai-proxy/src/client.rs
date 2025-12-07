//! Clientes para Ollama e DeepSeek

use crate::{config::ModelProvider, Error, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaResponse {
    pub message: OllamaMessage,
    #[serde(default)]
    pub prompt_eval_count: u32,
    #[serde(default)]
    pub eval_count: u32,
    #[serde(default)]
    pub total_duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaGenerateResponse {
    pub response: String,
    #[serde(default)]
    pub total_duration: u64,
}

#[derive(Clone)]
pub struct OllamaClient {
    client: Client,
    base_url: String,
}

impl OllamaClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .unwrap(),
            base_url,
        }
    }

    /// Chat completion
    pub async fn chat(
        &self,
        model: &str,
        messages: Vec<OllamaMessage>,
        temperature: f32,
        max_tokens: u32,
    ) -> Result<OllamaResponse> {
        let url = format!("{}/api/chat", self.base_url);

        let payload = json!({
            "model": model,
            "messages": messages,
            "stream": false,
            "options": {
                "temperature": temperature,
                "num_predict": max_tokens
            }
        });

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| Error::OllamaError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::OllamaError(format!(
                "HTTP {}: {}",
                response.status(),
                error_text
            )));
        }

        response
            .json()
            .await
            .map_err(|e| Error::OllamaError(e.to_string()))
    }

    /// Code generation
    pub async fn generate(
        &self,
        model: &str,
        prompt: &str,
        temperature: f32,
        max_tokens: u32,
    ) -> Result<OllamaGenerateResponse> {
        let url = format!("{}/api/generate", self.base_url);

        let payload = json!({
            "model": model,
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": temperature,
                "num_predict": max_tokens
            }
        });

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| Error::OllamaError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::OllamaError(format!(
                "HTTP {}: {}",
                response.status(),
                error_text
            )));
        }

        response
            .json()
            .await
            .map_err(|e| Error::OllamaError(e.to_string()))
    }
}
