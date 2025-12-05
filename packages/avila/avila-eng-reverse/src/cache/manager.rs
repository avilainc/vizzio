// Cache manager
use super::storage::CacheStorage;
use std::error::Error;
use std::time::Duration;

/// Cache manager for intelligent caching
pub struct CacheManager {
    storage: Box<dyn CacheStorage>,
    default_ttl: Duration,
}

impl CacheManager {
    pub fn new(storage: Box<dyn CacheStorage>, ttl_seconds: u64) -> Self {
        Self {
            storage,
            default_ttl: Duration::from_secs(ttl_seconds),
        }
    }

    /// Get value from cache
    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        self.storage.get(key)
    }

    /// Set value in cache
    pub fn set(&mut self, key: &str, value: Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.storage.set(key, value, self.default_ttl)
    }

    /// Set value with custom TTL
    pub fn set_with_ttl(&mut self, key: &str, value: Vec<u8>, ttl: Duration) -> Result<(), Box<dyn Error>> {
        self.storage.set(key, value, ttl)
    }

    /// Delete value from cache
    pub fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        self.storage.delete(key)
    }

    /// Clear all cache
    pub fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        self.storage.clear()
    }

    /// Check if key exists
    pub fn exists(&self, key: &str) -> Result<bool, Box<dyn Error>> {
        self.storage.exists(key)
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            hits: 0,
            misses: 0,
            size: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub size: usize,
}
