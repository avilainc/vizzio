// Cache storage trait
use std::error::Error;
use std::time::Duration;

/// Trait for cache storage backends
pub trait CacheStorage: Send + Sync {
    /// Get value by key
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Box<dyn Error>>;

    /// Set value with TTL
    fn set(&mut self, key: &str, value: Vec<u8>, ttl: Duration) -> Result<(), Box<dyn Error>>;

    /// Delete value by key
    fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>>;

    /// Clear all entries
    fn clear(&mut self) -> Result<(), Box<dyn Error>>;

    /// Check if key exists
    fn exists(&self, key: &str) -> Result<bool, Box<dyn Error>>;
}
