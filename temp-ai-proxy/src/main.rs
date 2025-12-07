//! Avila AI Proxy - Main binary

use avila_ai_proxy::{
    api_key::ApiKeyManager, client::OllamaClient, config::Config, handlers::AppStateInner,
    rate_limit::RateLimiter, router::create_router,
};
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inicializar logging
    tracing_subscriber::fmt()
        .with_env_filter("avila_ai_proxy=debug,tower_http=debug")
        .init();

    // Carregar configuração
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    info!("🚀 Avila AI Proxy v0.1.0");
    info!("=" .repeat(60));
    info!("📡 Ollama: {}", config.ollama_url);
    info!("🤖 Modelos disponíveis: {}", config.models.len());

    // Inicializar clientes
    let ollama_client = OllamaClient::new(config.ollama_url.clone());

    // Inicializar gerenciadores
    let api_key_manager = ApiKeyManager::new();
    let rate_limiter = RateLimiter::default();

    // Criar admin key
    let admin_key = api_key_manager
        .initialize_admin(config.admin_api_key.clone())
        .await;

    info!("🔑 Admin API Key: {}", admin_key);
    info!("=" .repeat(60));

    // Criar state
    let state = AppStateInner {
        config: config.clone(),
        ollama_client,
        api_key_manager,
        rate_limiter,
    };

    // Criar router
    let app = create_router(state);

    // Iniciar servidor
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("🌐 Servidor rodando em: http://{}", addr);
    info!("📖 Docs: http://{}/models", addr);
    info!("=" .repeat(60));

    axum::serve(listener, app).await?;

    Ok(())
}
