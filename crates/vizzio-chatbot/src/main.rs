/// VIZZIO ChatBot - Rust Native
/// WhatsApp-style notifications com Avila HTTP + WebSocket
/// YOLO MODE: Sem testes, pura produção

mod handlers;
mod models;
mod db;

use avila_http::{Server, Router, Request, Response};
use avila_websocket::WebSocket;
use avila_db::Database as AvilaDb;
use avila_async::runtime;
use avila_log::{info, error};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use models::{ChatMessage, ChatState};
use handlers::{chat_handler, notify_build, get_messages, get_unread};
use db::Database;

#[runtime::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    avila_log::init();

    let mongo_uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let db = AvilaDb::connect(&mongo_uri, "vizzio-chat").await?;
    let db = Database::new(db);
    db.init().await?;

    let db = Arc::new(RwLock::new(db));
    let state = Arc::new(ChatState { db: db.clone() });

    let port = std::env::var("CHATBOT_PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse::<u16>()
        .unwrap_or(3001);

    info!("💬 VIZZIO ChatBot Rust");
    info!("🚀 http://localhost:{}/chat", port);
    info!("📡 WebSocket: ws://localhost:{}/ws", port);

    let mut router = Router::new();
    router.get("/chat", serve_chat_html);
    router.ws("/ws", move |ws| chat_handler(ws, state.clone()));
    router.post("/api/chat/notify", move |req| notify_build(req, state.clone()));
    router.get("/api/chat/messages", move |req| get_messages(req, state.clone()));
    router.get("/api/chat/unread", move |req| get_unread(req, state.clone()));

    let server = Server::bind(("0.0.0.0", port))
        .router(router)
        .run()
        .await?;

    Ok(())
}

async fn serve_chat_html(_req: Request) -> Response {
    let path = std::env::var("CHAT_HTML_PATH")
        .unwrap_or_else(|_| "./src/chat.html".to_string());

    match std::fs::read_to_string(&path) {
        Ok(content) => Response::html(content),
        Err(_) => Response::not_found("Chat HTML not found"),
    }
}
