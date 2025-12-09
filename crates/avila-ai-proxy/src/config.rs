//! Configuração do servidor via variáveis de ambiente

use std::env;

/// Configuração do servidor
#[derive(Debug, Clone)]
pub struct Config {
    /// Host para bind (padrão: 0.0.0.0)
    pub host: String,

    /// Porta do servidor (padrão: 8000)
    pub port: u16,

    /// URL do Ollama (padrão: http://localhost:11434)
    pub ollama_url: String,

    /// API Key da OpenAI (opcional)
    pub openai_api_key: Option<String>,

    /// API Key da DeepSeek (opcional)
    pub deepseek_api_key: Option<String>,

    /// Admin API Key (opcional)
    pub admin_api_key: Option<String>,
}

impl Config {
    /// Carrega configuração das variáveis de ambiente
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Config {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()?,
            ollama_url: env::var("OLLAMA_URL")
                .unwrap_or_else(|_| "http://localhost:11434".to_string()),
        })
    }
}
