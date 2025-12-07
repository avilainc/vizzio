//! HTTP handlers

use crate::{
    api_key::{ApiKey, ApiKeyManager, Tier},
    client::{OllamaClient, OllamaMessage},
    config::Config,
    models::*,
    rate_limit::RateLimiter,
    Error, Result,
};
use axum::{
    extract::{Extension, Json, State, ws::{WebSocket, WebSocketUpgrade, Message}},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use uuid::Uuid;

pub type AppState = Arc<AppStateInner>;

pub struct AppStateInner {
    pub config: Config,
    pub ollama_client: OllamaClient,
    pub api_key_manager: ApiKeyManager,
    pub rate_limiter: RateLimiter,
}

/// Health check
pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "service": "Avila AI Proxy",
        "status": "online",
        "timestamp": Utc::now()
    }))
}

/// Lista modelos disponíveis
pub async fn list_models(State(state): State<AppState>) -> Result<Json<ModelsResponse>> {
    let models = state
        .config
        .models
        .iter()
        .map(|(id, config)| ModelInfo {
            id: id.clone(),
            name: config.name.clone(),
            description: config.description.clone(),
            provider: format!("{:?}", config.provider),
            max_tokens: config.max_tokens,
        })
        .collect();

    Ok(Json(ModelsResponse {
        models,
        default_model: state.config.default_model().to_string(),
    }))
}

/// Chat completion (OpenAI-compatible)
pub async fn chat_completion(
    State(state): State<AppState>,
    Extension(api_key): Extension<String>,
    Json(req): Json<ChatCompletionRequest>,
) -> Result<Json<ChatCompletionResponse>> {
    // Validar modelo
    let model_config = state
        .config
        .get_model(&req.model)
        .ok_or_else(|| Error::ModelNotFound(req.model.clone()))?;

    // Converter mensagens
    let messages: Vec<OllamaMessage> = req
        .messages
        .iter()
        .map(|m| OllamaMessage {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();

    // Chamar Ollama
    let response = state
        .ollama_client
        .chat(
            &model_config.name,
            messages,
            req.temperature,
            req.max_tokens,
        )
        .await?;

    // Formatar resposta no padrão OpenAI
    Ok(Json(ChatCompletionResponse {
        id: format!("chatcmpl-{}", Uuid::new_v4()),
        object: "chat.completion".to_string(),
        created: Utc::now().timestamp(),
        model: req.model,
        choices: vec![ChatChoice {
            index: 0,
            message: ChatMessage {
                role: "assistant".to_string(),
                content: response.message.content,
            },
            finish_reason: "stop".to_string(),
        }],
        usage: Usage {
            prompt_tokens: response.prompt_eval_count,
            completion_tokens: response.eval_count,
            total_tokens: response.prompt_eval_count + response.eval_count,
        },
    }))
}

/// Code completion para IDEs
pub async fn code_completion(
    State(state): State<AppState>,
    Extension(api_key): Extension<String>,
    Json(req): Json<CodeCompletionRequest>,
) -> Result<Json<CodeCompletionResponse>> {
    // Validar modelo
    let model_config = state
        .config
        .get_model(&req.model)
        .ok_or_else(|| Error::ModelNotFound(req.model.clone()))?;

    // Construir prompt otimizado para código
    let prompt = format!(
        "Complete o código {} abaixo. Retorne APENAS o código completado, sem explicações.\n\n```{}\n{}\n```\n\nCompletação:",
        req.language, req.language, req.code
    );

    let start = std::time::Instant::now();

    // Gerar código
    let response = state
        .ollama_client
        .generate(&model_config.name, &prompt, 0.2, req.max_tokens)
        .await?;

    let latency_ms = start.elapsed().as_millis() as u64;

    Ok(Json(CodeCompletionResponse {
        completion: response.response,
        language: req.language,
        model: req.model,
        latency_ms,
    }))
}

/// Cria nova API key (admin only)
pub async fn create_api_key(
    State(state): State<AppState>,
    Extension(current_key_info): Extension<ApiKey>,
    Json(req): Json<CreateApiKeyRequest>,
) -> Result<Json<ApiKeyResponse>> {
    // Verificar se é admin
    if current_key_info.tier != Tier::Admin {
        return Err(Error::Forbidden);
    }

    // Converter tier string para enum
    let tier = match req.tier.to_lowercase().as_str() {
        "free" => Tier::Free,
        "paid" => Tier::Paid,
        "admin" => Tier::Admin,
        _ => return Err(Error::InvalidParameter("tier inválido".to_string())),
    };

    // Criar key
    let (key, api_key) = state
        .api_key_manager
        .create_key(req.name, tier, req.expires_days)
        .await?;

    Ok(Json(ApiKeyResponse {
        key,
        name: api_key.name,
        tier: api_key.tier,
        created_at: api_key.created_at,
        expires_at: api_key.expires_at,
    }))
}

/// Retorna estatísticas de uso
pub async fn get_usage(
    State(state): State<AppState>,
    Extension(api_key): Extension<String>,
    Extension(key_info): Extension<ApiKey>,
) -> Result<Json<UsageResponse>> {
    let (current, limit) = state
        .rate_limiter
        .get_usage(&api_key, &key_info.tier)
        .await;

    Ok(Json(UsageResponse {
        key_info: KeyInfo {
            name: key_info.name,
            tier: key_info.tier.clone(),
            created_at: key_info.created_at,
            expires_at: key_info.expires_at,
        },
        usage: UsageStats {
            requests_last_minute: current,
            requests_remaining: limit.saturating_sub(current),
            requests_total: key_info.requests_total,
            limit_per_minute: key_info.tier.rate_limit(),
        },
    }))
}

/// WebSocket handler para streaming
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

async fn handle_websocket(mut socket: WebSocket, state: AppState) {
    // Aguardar API key
    let api_key = match socket.recv().await {
        Some(Ok(Message::Text(key))) => key,
        _ => {
            let _ = socket.send(Message::Text(
                serde_json::to_string(&serde_json::json!({
                    "error": "API key não fornecida"
                })).unwrap()
            )).await;
            return;
        }
    };

    // Validar API key
    let key_info = match state.api_key_manager.validate_key(&api_key).await {
        Ok(info) => info,
        Err(_) => {
            let _ = socket.send(Message::Text(
                serde_json::to_string(&serde_json::json!({
                    "error": "API key inválida"
                })).unwrap()
            )).await;
            return;
        }
    };

    // Verificar rate limit
    if let Err(_) = state.rate_limiter.check_and_record(&api_key, &key_info.tier).await {
        let _ = socket.send(Message::Text(
            serde_json::to_string(&serde_json::json!({
                "error": "Rate limit excedido"
            })).unwrap()
        )).await;
        return;
    }

    // Loop de mensagens
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                // Parse request
                let request: ChatCompletionRequest = match serde_json::from_str(&text) {
                    Ok(req) => req,
                    Err(_) => {
                        let _ = socket.send(Message::Text(
                            serde_json::to_string(&serde_json::json!({
                                "error": "JSON inválido"
                            })).unwrap()
                        )).await;
                        continue;
                    }
                };

                // Validar modelo
                let model_config = match state.config.get_model(&request.model) {
                    Some(cfg) => cfg,
                    None => {
                        let _ = socket.send(Message::Text(
                            serde_json::to_string(&serde_json::json!({
                                "error": format!("Modelo '{}' não encontrado", request.model)
                            })).unwrap()
                        )).await;
                        continue;
                    }
                };

                // Converter mensagens
                let messages: Vec<OllamaMessage> = request
                    .messages
                    .iter()
                    .map(|m| OllamaMessage {
                        role: m.role.clone(),
                        content: m.content.clone(),
                    })
                    .collect();

                // Chamar Ollama
                match state.ollama_client.chat(
                    &model_config.name,
                    messages,
                    request.temperature,
                    request.max_tokens,
                ).await {
                    Ok(response) => {
                        let _ = socket.send(Message::Text(
                            serde_json::to_string(&serde_json::json!({
                                "type": "completion",
                                "content": response.message.content,
                                "done": true
                            })).unwrap()
                        )).await;
                    }
                    Err(e) => {
                        let _ = socket.send(Message::Text(
                            serde_json::to_string(&serde_json::json!({
                                "error": format!("Erro Ollama: {}", e)
                            })).unwrap()
                        )).await;
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            _ => {}
        }
    }
}
