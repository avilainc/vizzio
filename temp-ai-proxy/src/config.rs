//! Configuração do servidor

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Endereço do servidor Ollama
    pub ollama_url: String,

    /// API key do DeepSeek (opcional)
    pub deepseek_api_key: Option<String>,

    /// API key do admin (gerada se não fornecida)
    pub admin_api_key: Option<String>,

    /// Porta do servidor
    pub port: u16,

    /// Host do servidor
    pub host: String,

    /// Rate limits por tier
    pub rate_limits: RateLimitConfig,

    /// Modelos disponíveis
    pub models: HashMap<String, ModelConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub free: u32,
    pub paid: u32,
    pub admin: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub description: String,
    pub provider: ModelProvider,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ModelProvider {
    Ollama,
    DeepSeek,
}

impl Default for Config {
    fn default() -> Self {
        let mut models = HashMap::new();

        // Modelos sem censura Ollama
        models.insert(
            "gpt-oss".to_string(),
            ModelConfig {
                name: "gpt-oss:120b-cloud".to_string(),
                description: "GPT-OSS 120B - Modelo gigante local sem censura".to_string(),
                provider: ModelProvider::Ollama,
                max_tokens: 32000,
            },
        );

        models.insert(
            "dolphin-mistral".to_string(),
            ModelConfig {
                name: "dolphin-mistral:latest".to_string(),
                description: "Dolphin Mistral - Modelo sem censura e alinhamento".to_string(),
                provider: ModelProvider::Ollama,
                max_tokens: 32000,
            },
        );

        models.insert(
            "wizard-vicuna".to_string(),
            ModelConfig {
                name: "wizard-vicuna-uncensored:latest".to_string(),
                description: "Wizard Vicuna Uncensored - Sem filtros ou restrições".to_string(),
                provider: ModelProvider::Ollama,
                max_tokens: 16000,
            },
        );

        models.insert(
            "neural-chat".to_string(),
            ModelConfig {
                name: "neural-chat:latest".to_string(),
                description: "Neural Chat - Modelo conversacional sem censura".to_string(),
                provider: ModelProvider::Ollama,
                max_tokens: 8000,
            },
        );

        models.insert(
            "deepseek-coder".to_string(),
            ModelConfig {
                name: "deepseek-coder".to_string(),
                description: "DeepSeek Coder - Especializado em código".to_string(),
                provider: ModelProvider::DeepSeek,
                max_tokens: 32000,
            },
        );

        Self {
            ollama_url: std::env::var("OLLAMA_URL")
                .unwrap_or_else(|_| "http://localhost:11434".to_string()),
            deepseek_api_key: std::env::var("DEEPSEEK_API_KEY").ok(),
            admin_api_key: std::env::var("ADMIN_API_KEY").ok(),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8000),
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            rate_limits: RateLimitConfig {
                free: 10,
                paid: 100,
                admin: 1000,
            },
            models,
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        Self::default()
    }

    pub fn get_model(&self, model_id: &str) -> Option<&ModelConfig> {
        self.models.get(model_id)
    }

    pub fn default_model(&self) -> &str {
        "gpt-oss"
    }
}
