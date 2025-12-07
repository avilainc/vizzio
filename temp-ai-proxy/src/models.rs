//! Modelos de dados para API

use crate::api_key::Tier;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ==================== Chat ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default)]
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: Usage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

// ==================== Code Completion ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeCompletionRequest {
    pub code: String,
    pub language: String,
    #[serde(default)]
    pub cursor_position: usize,
    pub model: String,
    #[serde(default = "default_code_max_tokens")]
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeCompletionResponse {
    pub completion: String,
    pub language: String,
    pub model: String,
    pub latency_ms: u64,
}

// ==================== API Keys ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    #[serde(default = "default_tier")]
    pub tier: String,
    #[serde(default = "default_expires_days")]
    pub expires_days: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyResponse {
    pub key: String,
    pub name: String,
    pub tier: Tier,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

// ==================== Usage ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageResponse {
    pub key_info: KeyInfo,
    pub usage: UsageStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    pub name: String,
    pub tier: Tier,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    pub requests_last_minute: usize,
    pub requests_remaining: usize,
    pub requests_total: u64,
    pub limit_per_minute: u32,
}

// ==================== Models ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsResponse {
    pub models: Vec<ModelInfo>,
    pub default_model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub provider: String,
    pub max_tokens: u32,
}

// ==================== Defaults ====================

fn default_temperature() -> f32 {
    0.7
}

fn default_max_tokens() -> u32 {
    2000
}

fn default_code_max_tokens() -> u32 {
    500
}

fn default_tier() -> String {
    "free".to_string()
}

fn default_expires_days() -> i64 {
    365
}
