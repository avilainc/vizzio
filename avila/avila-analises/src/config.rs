//! Configuration module

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub websocket: WebSocketConfig,
    pub streaming: StreamingConfig,
    pub ml: MLConfig,
    pub analytics: AnalyticsConfig,
    pub logging: LoggingConfig,
    pub monitoring: MonitoringConfig,
    pub security: SecurityConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub backend: String,
    pub connection_string: String,
    pub max_connections: u32,
    pub connection_timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub ttl_seconds: u64,
    pub max_size_mb: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConfig {
    pub enabled: bool,
    pub max_connections: usize,
    pub ping_interval_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingConfig {
    pub backend: String,
    pub brokers: Vec<String>,
    pub topic: String,
    pub group_id: String,
    pub buffer_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLConfig {
    pub framework: String,
    pub model_dir: String,
    pub cache_models: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfig {
    pub realtime_enabled: bool,
    pub batch_size: usize,
    pub flush_interval_ms: u64,
    pub retention_days: u32,
    pub archive_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub metrics_port: u16,
    pub health_check_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub auth_enabled: bool,
    pub jwt_secret: String,
    pub token_expiry_hours: u64,
    pub rate_limit_enabled: bool,
    pub requests_per_minute: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub thread_pool_size: usize,
    pub max_blocking_threads: usize,
    pub max_memory_mb: usize,
    pub gc_interval_seconds: u64,
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        toml::from_str(&contents)
            .map_err(|e| format!("Failed to parse config: {}", e))
    }

    /// Load configuration based on environment
    pub fn load() -> Result<Self, String> {
        let env = std::env::var("AVILA_ENV").unwrap_or_else(|_| "development".to_string());

        let config_file = match env.as_str() {
            "production" => "config.prod.toml",
            "development" => "config.dev.toml",
            _ => "config.toml",
        };

        Self::from_file(config_file)
    }

    /// Get default development configuration
    pub fn default_dev() -> Self {
        // TODO: Implement default development config
        unimplemented!()
    }
}
