//! Estruturas de dados para requests e responses

use std::collections::HashMap;

/// Mensagem de chat
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Request de chat completion
#[derive(Debug, Clone)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: Option<bool>,
}

/// Response de chat completion
#[derive(Debug, Clone)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

/// Escolha de resposta
#[derive(Debug, Clone)]
pub struct Choice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: String,
}

/// Uso de tokens
#[derive(Debug, Clone)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Informação de modelo
#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
}

/// Lista de modelos
#[derive(Debug, Clone)]
pub struct ModelList {
    pub object: String,
    pub data: Vec<ModelInfo>,
}

impl ChatMessage {
    /// Converte para JSON string
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"role":"{}","content":"{}"}}"#,
            self.role.replace('"', "\\\""),
            self.content.replace('"', "\\\"")
        )
    }
}

impl ChatRequest {
    /// Parse de JSON string
    pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: Implementar parsing JSON completo com avila-codec
        // Por enquanto, retorna erro
        Err("JSON parsing not implemented yet".into())
    }
}

impl ChatResponse {
    /// Converte para JSON string
    pub fn to_json(&self) -> String {
        let choices_json: Vec<String> = self.choices
            .iter()
            .map(|c| {
                format!(
                    r#"{{"index":{},"message":{},"finish_reason":"{}"}}"#,
                    c.index,
                    c.message.to_json(),
                    c.finish_reason
                )
            })
            .collect();

        format!(
            r#"{{"id":"{}","object":"{}","created":{},"model":"{}","choices":[{}],"usage":{{"prompt_tokens":{},"completion_tokens":{},"total_tokens":{}}}}}"#,
            self.id,
            self.object,
            self.created,
            self.model,
            choices_json.join(","),
            self.usage.prompt_tokens,
            self.usage.completion_tokens,
            self.usage.total_tokens
        )
    }
}

impl ModelList {
    /// Converte para JSON string
    pub fn to_json(&self) -> String {
        let data_json: Vec<String> = self.data
            .iter()
            .map(|m| {
                format!(
                    r#"{{"id":"{}","object":"{}","created":{},"owned_by":"{}"}}"#,
                    m.id, m.object, m.created, m.owned_by
                )
            })
            .collect();

        format!(
            r#"{{"object":"{}","data":[{}]}}"#,
            self.object,
            data_json.join(",")
        )
    }
}
