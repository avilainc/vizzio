//! Middleware de autenticação

use crate::{api_key::ApiKeyManager, rate_limit::RateLimiter, Error, Result};
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub api_key_manager: Arc<ApiKeyManager>,
    pub rate_limiter: Arc<RateLimiter>,
}

/// Extrai e valida API key do header Authorization
pub async fn auth_middleware<B>(
    State(state): State<AppState>,
    mut req: Request<B>,
    next: Next<B>,
) -> std::result::Result<Response, StatusCode> {
    // Extrair header Authorization
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Extrair key (remover "Bearer " se presente)
    let api_key = if auth_header.starts_with("Bearer ") {
        &auth_header[7..]
    } else {
        auth_header
    };

    // Validar API key
    let key_info = state
        .api_key_manager
        .validate_key(api_key)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Verificar rate limit
    state
        .rate_limiter
        .check_and_record(api_key, &key_info.tier)
        .await
        .map_err(|_| StatusCode::TOO_MANY_REQUESTS)?;

    // Incrementar contador
    let _ = state.api_key_manager.increment_requests(api_key).await;

    // Adicionar informações ao request para uso nos handlers
    req.extensions_mut().insert(key_info);
    req.extensions_mut().insert(api_key.to_string());

    Ok(next.run(req).await)
}
