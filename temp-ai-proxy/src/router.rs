//! Configuração das rotas

use crate::{
    handlers::{self, AppState, AppStateInner},
    middleware::auth_middleware,
};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub fn create_router(state: AppStateInner) -> Router {
    let app_state = Arc::new(state);

    // Rotas públicas (sem auth)
    let public_routes = Router::new()
        .route("/", get(handlers::health_check))
        .route("/health", get(handlers::health_check));

    // Rotas protegidas (com auth)
    let protected_routes = Router::new()
        .route("/models", get(handlers::list_models))
        .route("/v1/chat/completions", post(handlers::chat_completion))
        .route("/v1/code/completions", post(handlers::code_completion))
        .route("/v1/keys", post(handlers::create_api_key))
        .route("/v1/usage", get(handlers::get_usage))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    // WebSocket (sem middleware auth - autentica internamente)
    let ws_routes = Router::new()
        .route("/ws", get(handlers::websocket_handler));

    // Combinar rotas
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(ws_routes)
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}
