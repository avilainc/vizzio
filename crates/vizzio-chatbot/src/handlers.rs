use avila_http::{Request, Response, Json};
use avila_websocket::{WebSocket, Message as WsMessage};
use avila_serde::json;
use avila_log::{info, error};
use std::sync::Arc;

use crate::models::{ChatMessage, ChatState, Sender, MessageType, BuildNotification};

// WebSocket Handler
pub async fn chat_handler(mut ws: WebSocket, state: Arc<ChatState>) {
    info!("💬 WebSocket connection started");

    while let Some(msg) = ws.recv().await {
        match msg {
            WsMessage::Text(text) => {
                if let Ok(data) = avila_serde::from_str::<serde_json::Value>(&text) {
                    if let Some("send-message") = data.get("type").and_then(|v| v.as_str()) {
                        if let Some(content) = data.get("content").and_then(|v| v.as_str()) {
                            let db = state.db.clone();
                            let content = content.to_string();

                            tokio::spawn(async move {
                                let message = ChatMessage::new(
                                    Sender::User,
                                    content.clone(),
                                    MessageType::Text,
                                );

                                let db_guard = db.read().await;
                                let _ = db_guard.save_message(&message).await;
                                drop(db_guard);

                                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                                handle_bot_response(&db, &content).await;
                            });
                        }
                    }
                }
            }
            WsMessage::Close => {
                info!("💬 WebSocket connection closed");
                break;
            }
            _ => {}
        }
    }
}

// Bot Responses
async fn handle_bot_response(
    db: &std::sync::Arc<tokio::sync::RwLock<crate::db::Database>>,
    user_message: &str,
) {
    let message_lower = user_message.to_lowercase();

    let response = if message_lower.contains("build") || message_lower.contains("status") {
        json!({
            "content": "📊 **Status de Builds**\n\n✅ CI/CD Pipeline (main): SUCCESS\n⏱️ Duração: 2m 15s\n🧪 Testes: 50/50 passed\n📈 Coverage: 85%\n\n✅ Release (v1.0.0): SUCCESS\n⏱️ Publicado no crates.io\n\n⚠️ Deploy (develop): RUNNING...\n⏳ Tempo decorrido: 1m 30s",
            "msg_type": "notification"
        })
    } else if message_lower.contains("erro") || message_lower.contains("fail") {
        json!({
            "content": "❌ **Erros Detectados**\n\n🔴 Deploy (feature/new-api): FAILED\n\nErro:\n```\nTest failed: authentication_test\nTimeout after 5000ms\n```\n\nArquivo: src/auth.rs:145\n\nSolução sugerida:\nAjuste o timeout ou revise a lógica de autenticação.",
            "msg_type": "alert"
        })
    } else if message_lower.contains("sucesso") || message_lower.contains("success") {
        json!({
            "content": "✅ **Todos os Builds Passaram!**\n\n🏆 Taxa de Sucesso: 96.67%\n📈 Tendência: +2.3% desde ontem\n\n🚀 Últimas releases:\n• v2.1.0 - Released 2h ago\n• v2.0.9 - Released 1d ago\n• v2.0.8 - Released 2d ago\n\nParabéns! 🎉",
            "msg_type": "success"
        })
    } else {
        json!({
            "content": "👋 Oi! Sou o VIZZIO Bot. Posso ajudar com:\n\n🔍 **build status** - Ver status dos builds\n⚠️ **erros** - Listar erros recentes\n✅ **success** - Mostrar builds bem-sucedidos\n📊 **metrics** - Métricas gerais\n🚀 **deploy** - Status de deployments",
            "msg_type": "text"
        })
    };

    let msg_type = response.get("msg_type")
        .and_then(|v| v.as_str())
        .and_then(|s| match s {
            "notification" => Some(MessageType::Notification),
            "alert" => Some(MessageType::Alert),
            "success" => Some(MessageType::Success),
            _ => Some(MessageType::Text),
        })
        .unwrap_or(MessageType::Text);

    let content = response.get("content")
        .and_then(|v| v.as_str())
        .unwrap_or("Resposta não entendida")
        .to_string();

    let mut message = ChatMessage::new(Sender::Bot, content, msg_type);
    message.read = false;

    let db_guard = db.read().await;
    let _ = db_guard.save_message(&message).await;
}

// API Routes
pub async fn notify_build(req: Request, state: Arc<ChatState>) -> Response {
    let notification: BuildNotification = match req.json().await {
        Ok(n) => n,
        Err(_) => return Response::bad_request("Invalid JSON"),
    };

    let emoji = match notification.status.as_str() {
        "success" => "✅",
        "failure" => "❌",
        "running" => "🔄",
        _ => "📢",
    };

    let content = format!(
        "{} **{}** - {}\n\n📋 Build: {}\n⏱️ Testes: {}/{}\n📈 Coverage: {}%",
        emoji,
        notification.workflow,
        notification.status.to_uppercase(),
        notification.buildId,
        notification.details.get("testsPassed").map(|v| v.to_string()).unwrap_or_default(),
        notification.details.get("testsRun").map(|v| v.to_string()).unwrap_or_default(),
        notification.details.get("coverage").map(|v| v.to_string()).unwrap_or_default(),
    );

    let msg_type = match notification.status.as_str() {
        "failure" => MessageType::Alert,
        "success" => MessageType::Success,
        _ => MessageType::Notification,
    };

    let message = ChatMessage::new(Sender::Bot, content, msg_type)
        .with_metadata(json!({
            "buildId": notification.buildId,
            "workflow": notification.workflow,
        }));

    let db = state.db.clone();
    tokio::spawn(async move {
        let db_guard = db.read().await;
        let _ = db_guard.save_message(&message).await;
    });

    Response::json(json!({"success": true}))
}

pub async fn get_messages(req: Request, state: Arc<ChatState>) -> Response {
    let limit = req.query("limit")
        .and_then(|l| l.parse::<usize>().ok())
        .unwrap_or(50);

    let db = state.db.read().await;
    match db.get_messages(limit).await {
        Ok(messages) => Response::json(messages),
        Err(e) => {
            error!("Error getting messages: {}", e);
            Response::internal_error(format!("Error: {}", e))
        }
    }
}

pub async fn get_unread(_req: Request, state: Arc<ChatState>) -> Response {
    let db = state.db.read().await;
    match db.get_unread_count().await {
        Ok(count) => Response::json(json!({"unread": count})),
        Err(e) => {
            error!("Error getting unread count: {}", e);
            Response::internal_error(format!("Error: {}", e))
        }
    }
}
