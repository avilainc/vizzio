//! Rate limiting por API key

use crate::{api_key::Tier, Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
struct RequestLog {
    timestamps: Vec<Instant>,
}

impl RequestLog {
    fn new() -> Self {
        Self {
            timestamps: Vec::new(),
        }
    }

    fn clean_old(&mut self, window: Duration) {
        let now = Instant::now();
        self.timestamps.retain(|&t| now.duration_since(t) < window);
    }

    fn count(&self) -> usize {
        self.timestamps.len()
    }

    fn add(&mut self) {
        self.timestamps.push(Instant::now());
    }
}

#[derive(Clone)]
pub struct RateLimiter {
    logs: Arc<RwLock<HashMap<String, RequestLog>>>,
    window: Duration,
}

impl RateLimiter {
    pub fn new(window_seconds: u64) -> Self {
        Self {
            logs: Arc::new(RwLock::new(HashMap::new())),
            window: Duration::from_secs(window_seconds),
        }
    }

    /// Verifica e registra requisição
    pub async fn check_and_record(&self, key: &str, tier: &Tier) -> Result<()> {
        let mut logs = self.logs.write().await;
        let log = logs.entry(key.to_string()).or_insert_with(RequestLog::new);

        // Limpar timestamps antigos
        log.clean_old(self.window);

        let limit = tier.rate_limit() as usize;
        let current = log.count();

        if current >= limit {
            return Err(Error::RateLimitExceeded(format!(
                "Limite: {}/min, Tier: {:?}",
                limit, tier
            )));
        }

        log.add();
        Ok(())
    }

    /// Retorna informações de uso
    pub async fn get_usage(&self, key: &str, tier: &Tier) -> (usize, usize) {
        let mut logs = self.logs.write().await;
        let log = logs.entry(key.to_string()).or_insert_with(RequestLog::new);
        log.clean_old(self.window);

        let current = log.count();
        let limit = tier.rate_limit() as usize;

        (current, limit)
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(60) // Janela de 60 segundos (1 minuto)
    }
}
