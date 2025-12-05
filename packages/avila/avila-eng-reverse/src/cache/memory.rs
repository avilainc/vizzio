// In-memory cache implementation
use super::storage::CacheStorage;
use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, Instant};

struct CacheEntry {
    data: Vec<u8>,
    expires_at: Instant,
}

/// In-memory cache storage
pub struct MemoryCache {
    entries: HashMap<String, CacheEntry>,
    max_size: usize,
}

impl MemoryCache {
    pub fn new(max_size_mb: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_size: max_size_mb * 1024 * 1024,
        }
    }

    fn evict_expired(&mut self) {
        let now = Instant::now();
        self.entries.retain(|_, entry| entry.expires_at > now);
    }

    fn current_size(&self) -> usize {
        self.entries.values().map(|e| e.data.len()).sum()
    }
}

impl CacheStorage for MemoryCache {
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        if let Some(entry) = self.entries.get(key) {
            if entry.expires_at > Instant::now() {
                return Ok(Some(entry.data.clone()));
            }
        }
        Ok(None)
    }

    fn set(&mut self, key: &str, value: Vec<u8>, ttl: Duration) -> Result<(), Box<dyn Error>> {
        self.evict_expired();

        // Simple LRU eviction if needed
        while self.current_size() + value.len() > self.max_size && !self.entries.is_empty() {
            if let Some(key) = self.entries.keys().next().cloned() {
                self.entries.remove(&key);
            }
        }

        let entry = CacheEntry {
            data: value,
            expires_at: Instant::now() + ttl,
        };
        self.entries.insert(key.to_string(), entry);
        Ok(())
    }

    fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        self.entries.remove(key);
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        self.entries.clear();
        Ok(())
    }

    fn exists(&self, key: &str) -> Result<bool, Box<dyn Error>> {
        if let Some(entry) = self.entries.get(key) {
            Ok(entry.expires_at > Instant::now())
        } else {
            Ok(false)
        }
    }
}
