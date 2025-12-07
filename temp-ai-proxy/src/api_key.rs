//! Gerenciamento de API Keys

use crate::{Error, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    Free,
    Paid,
    Admin,
}

impl Tier {
    pub fn rate_limit(&self) -> u32 {
        match self {
            Tier::Free => 10,
            Tier::Paid => 100,
            Tier::Admin => 1000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub name: String,
    pub tier: Tier,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub requests_total: u64,
}

#[derive(Clone)]
pub struct ApiKeyManager {
    keys: Arc<RwLock<HashMap<String, ApiKey>>>,
}

impl ApiKeyManager {
    pub fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Gera uma nova API key segura
    pub fn generate_key() -> String {
        let random_bytes: [u8; 32] = rand::random();
        let hash = Sha256::digest(random_bytes);
        format!("avila_{}", base64::encode(&hash[..24]))
    }

    /// Cria uma nova API key
    pub async fn create_key(
        &self,
        name: String,
        tier: Tier,
        expires_days: i64,
    ) -> Result<(String, ApiKey)> {
        let key = Self::generate_key();
        let api_key = ApiKey {
            name,
            tier,
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::days(expires_days),
            requests_total: 0,
        };

        let mut keys = self.keys.write().await;
        keys.insert(key.clone(), api_key.clone());

        Ok((key, api_key))
    }

    /// Valida uma API key
    pub async fn validate_key(&self, key: &str) -> Result<ApiKey> {
        let keys = self.keys.read().await;
        let api_key = keys.get(key).ok_or(Error::InvalidApiKey)?;

        // Verificar expiração
        if Utc::now() > api_key.expires_at {
            return Err(Error::InvalidApiKey);
        }

        Ok(api_key.clone())
    }

    /// Incrementa contador de requisições
    pub async fn increment_requests(&self, key: &str) -> Result<()> {
        let mut keys = self.keys.write().await;
        if let Some(api_key) = keys.get_mut(key) {
            api_key.requests_total += 1;
        }
        Ok(())
    }

    /// Retorna estatísticas da API key
    pub async fn get_stats(&self, key: &str) -> Result<ApiKey> {
        let keys = self.keys.read().await;
        keys.get(key).cloned().ok_or(Error::InvalidApiKey)
    }

    /// Lista todas as keys (admin only)
    pub async fn list_keys(&self) -> Vec<(String, ApiKey)> {
        let keys = self.keys.read().await;
        keys.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Remove uma API key
    pub async fn revoke_key(&self, key: &str) -> Result<()> {
        let mut keys = self.keys.write().await;
        keys.remove(key).ok_or(Error::InvalidApiKey)?;
        Ok(())
    }

    /// Inicializa com admin key
    pub async fn initialize_admin(&self, admin_key: Option<String>) -> String {
        let key = admin_key.unwrap_or_else(Self::generate_key);
        let api_key = ApiKey {
            name: "Admin".to_string(),
            tier: Tier::Admin,
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::days(3650), // 10 anos
            requests_total: 0,
        };

        let mut keys = self.keys.write().await;
        keys.insert(key.clone(), api_key);

        key
    }
}

impl Default for ApiKeyManager {
    fn default() -> Self {
        Self::new()
    }
}
