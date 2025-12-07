//! Avila AI Proxy - Servidor de IA sem censura
//!
//! Proxy unificado para múltiplos modelos de IA:
//! - Ollama (modelos locais sem censura)
//! - DeepSeek (API remota)
//! - Integração com Avila Copilot, MCP, CLI

pub mod api_key;
pub mod client;
pub mod config;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod rate_limit;
pub mod router;

pub use config::Config;
pub use error::{Error, Result};
pub use router::create_router;

/// Re-exports principais
pub mod prelude {
    pub use crate::api_key::ApiKeyManager;
    pub use crate::client::{OllamaClient, ModelProvider};
    pub use crate::config::Config;
    pub use crate::error::{Error, Result};
    pub use crate::handlers::*;
    pub use crate::models::*;
    pub use crate::rate_limit::RateLimiter;
    pub use crate::router::create_router;
}
