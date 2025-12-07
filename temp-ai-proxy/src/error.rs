//! Error types

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("API key inválida ou expirada")]
    InvalidApiKey,

    #[error("Rate limit excedido: {0}")]
    RateLimitExceeded(String),

    #[error("Modelo não encontrado: {0}")]
    ModelNotFound(String),

    #[error("Erro ao comunicar com Ollama: {0}")]
    OllamaError(String),

    #[error("Erro ao comunicar com DeepSeek: {0}")]
    DeepSeekError(String),

    #[error("Erro de serialização: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Erro de requisição HTTP: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Apenas admins podem executar esta ação")]
    Forbidden,

    #[error("Parâmetro inválido: {0}")]
    InvalidParameter(String),

    #[error("Erro interno: {0}")]
    Internal(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::InvalidApiKey => (StatusCode::UNAUTHORIZED, self.to_string()),
            Error::RateLimitExceeded(_) => (StatusCode::TOO_MANY_REQUESTS, self.to_string()),
            Error::ModelNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            Error::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
            Error::InvalidParameter(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Error::OllamaError(_) | Error::DeepSeekError(_) => {
                (StatusCode::BAD_GATEWAY, self.to_string())
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "type": "api_error",
                "code": status.as_u16()
            }
        }));

        (status, body).into_response()
    }
}
