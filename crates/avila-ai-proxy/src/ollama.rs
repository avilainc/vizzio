//! Cliente Ollama

use crate::models::{ChatRequest, ChatResponse, ModelInfo, ModelList, ChatMessage, Choice, Usage};
use std::time::{SystemTime, UNIX_EPOCH};

/// Cliente Ollama
#[derive(Clone)]
pub struct OllamaClient {
    base_url: String,
}

impl OllamaClient {
    /// Cria novo cliente
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    /// Lista modelos disponíveis
    pub fn list_models(&self) -> Result<ModelList, Box<dyn std::error::Error>> {
        // TODO: Fazer request HTTP usando avila-http
        // GET {base_url}/api/tags

        // Por enquanto, retorna lista hardcoded
        Ok(ModelList {
            object: "list".to_string(),
            data: vec![
                ModelInfo {
                    id: "mistral:latest".to_string(),
                    object: "model".to_string(),
                    created: SystemTime::now()
                        .duration_since(UNIX_EPOCH)?
                        .as_secs(),
                    owned_by: "ollama".to_string(),
                },
                ModelInfo {
                    id: "dolphin-mistral:latest".to_string(),
                    object: "model".to_string(),
                    created: SystemTime::now()
                        .duration_since(UNIX_EPOCH)?
                        .as_secs(),
                    owned_by: "ollama".to_string(),
                },
            ],
        })
    }

    /// Chat completion
    pub fn chat_completion(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error>> {
        // TODO: Fazer request HTTP usando avila-http
        // POST {base_url}/api/chat
        // Body: {"model": "...", "messages": [...]}

        // Por enquanto, retorna mock
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        Ok(ChatResponse {
            id: format!("chatcmpl-{}", now),
            object: "chat.completion".to_string(),
            created: now,
            model: req.model.clone(),
            choices: vec![
                Choice {
                    index: 0,
                    message: ChatMessage {
                        role: "assistant".to_string(),
                        content: "[TODO] Response do Ollama".to_string(),
                    },
                    finish_reason: "stop".to_string(),
                }
            ],
            usage: Usage {
                prompt_tokens: 10,
                completion_tokens: 20,
                total_tokens: 30,
            },
        })
    }
}
