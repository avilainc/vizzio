//! Handlers HTTP

use crate::config::Config;
use crate::models::{ChatRequest, ModelList};
use crate::ollama::OllamaClient;

/// Contexto compartilhado entre handlers
#[derive(Clone)]
pub struct AppContext {
    pub config: Config,
    pub ollama: OllamaClient,
}

impl AppContext {
    /// Cria novo contexto
    pub fn new(config: Config) -> Self {
        let ollama = OllamaClient::new(config.ollama_url.clone());

        let openai = config.openai_api_key.clone()
            .map(|key| OpenAIClient::new(key));

        let deepseek = config.deepseek_api_key.clone()
            .map(|key| DeepSeekClient::new(key));

        Self {
            config,
            ollama,
            openai,
            deepseek,
        }
    }
}

/// Handler para health check
pub fn handle_health() -> Result<String, Box<dyn std::error::Error>> {
    Ok(r#"{"status":"ok"}"#.to_string())
}

/// Handler para listar modelos
pub fn handle_models(ctx: &AppContext) -> Result<String, Box<dyn std::error::Error>> {
    let models = ctx.ollama.list_models()?;
    Ok(models.to_json())
}

/// Handler para chat completion
pub fn handle_chat_completion(
    ctx: &AppContext,
    body: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Parse request
    let req = ChatRequest::from_json(body)?;
    
    // Sempre usa Ollama (100% local)
    let response = ctx.ollama.chat_completion(req)?;
    
    Ok(response.to_json())
}/// Handler para completion (legacy)
pub fn handle_completion(
    ctx: &AppContext,
    body: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: Implementar handler de completion
    Err("Legacy completions not implemented yet".into())
}
