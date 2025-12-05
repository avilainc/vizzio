// Threat intel cache
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Cache for threat intelligence queries
pub struct ThreatIntelCache {
    cache: HashMap<String, CacheEntry>,
    ttl: Duration,
}

struct CacheEntry {
    data: Vec<u8>,
    timestamp: Instant,
}

impl ThreatIntelCache {
    pub fn new(ttl_hours: u64) -> Self {
        Self {
            cache: HashMap::new(),
            ttl: Duration::from_secs(ttl_hours * 3600),
        }
    }

    /// Get from cache
    pub fn get(&self, key: &str) -> Option<&[u8]> {
        if let Some(entry) = self.cache.get(key) {
            if entry.timestamp.elapsed() < self.ttl {
                return Some(&entry.data);
            }
        }
        None
    }

    /// Put in cache
    pub fn put(&mut self, key: String, data: Vec<u8>) {
        self.cache.insert(key, CacheEntry {
            data,
            timestamp: Instant::now(),
        });
    }

    /// Clear expired entries
    pub fn cleanup(&mut self) {
        self.cache.retain(|_, entry| entry.timestamp.elapsed() < self.ttl);
    }
}
