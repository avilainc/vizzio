//! Core cache implementation
extern crate alloc;
use alloc::collections::BTreeMap;
use crate::config::CacheConfig;
use crate::stats::CacheStats;
use crate::eviction::EvictionPolicy;
use crate::error::CacheResult;

pub struct DistributedCache<K, V> {
    pub data: BTreeMap<K, V>,
    config: Option<CacheConfig>,
    stats: Option<CacheStats>,
}

impl<K: Ord, V> DistributedCache<K, V> {
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
            config: None,
            stats: None,
        }
    }

    pub fn with_config(config: CacheConfig) -> CacheResult<Self> {
        config.validate()?;
        let stats = if config.enable_stats {
            Some(CacheStats::new())
        } else {
            None
        };

        Ok(Self {
            data: BTreeMap::new(),
            config: Some(config),
            stats,
        })
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.data.insert(key, value);
        if let Some(stats) = &mut self.stats {
            stats.record_insertion();
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let result = self.data.get(key);
        if let Some(stats) = &self.stats {
            // Note: Can't mutate through &self, would need RefCell or different design
            // This is a limitation we'll document
        }
        result
    }

    pub fn get_stats(&self) -> Option<&CacheStats> {
        self.stats.as_ref()
    }

    pub fn config(&self) -> Option<&CacheConfig> {
        self.config.as_ref()
    }
}

/// Advanced cache with eviction policy
pub struct ManagedCache<K, V, P: EvictionPolicy<K>> {
    data: BTreeMap<K, V>,
    policy: P,
    max_capacity: usize,
    stats: CacheStats,
}

impl<K: Ord + Clone, V, P: EvictionPolicy<K>> ManagedCache<K, V, P> {
    pub fn new(policy: P, max_capacity: usize) -> Self {
        Self {
            data: BTreeMap::new(),
            policy,
            max_capacity,
            stats: CacheStats::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        // Evict if at capacity
        if self.data.len() >= self.max_capacity {
            if let Some(victim_key) = self.policy.select_victim() {
                let victim = victim_key.clone();
                self.data.remove(&victim);
                self.policy.on_remove(&victim);
                self.stats.record_eviction();
            }
        }

        self.data.insert(key.clone(), value);
        self.policy.on_insert(key);
        self.stats.record_insertion();
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let result = self.data.get(key);

        if result.is_some() {
            self.policy.on_access(key);
            self.stats.record_hit();
        } else {
            self.stats.record_miss();
        }

        result
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let result = self.data.remove(key);
        if result.is_some() {
            self.policy.on_remove(key);
        }
        result
    }

    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.policy.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eviction::LruPolicy;

    #[test]
    fn test_cache() {
        let mut cache = DistributedCache::new();
        cache.insert(1u64, 100u64);
        assert_eq!(cache.get(&1), Some(&100));
    }

    #[test]
    fn test_cache_with_config() {
        let config = CacheConfig::new()
            .with_capacity(100)
            .with_stats(true);

        let cache = DistributedCache::<i32, String>::with_config(config);
        assert!(cache.is_ok());

        let cache = cache.unwrap();
        assert!(cache.config().is_some());
        assert!(cache.get_stats().is_some());
    }

    #[test]
    fn test_managed_cache_eviction() {
        let policy = LruPolicy::new();
        let mut cache = ManagedCache::new(policy, 2);

        cache.insert(1, "a");
        cache.insert(2, "b");
        assert_eq!(cache.len(), 2);

        // This should evict key 1 (LRU)
        cache.insert(3, "c");
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&"b"));
        assert_eq!(cache.get(&3), Some(&"c"));
    }

    #[test]
    fn test_managed_cache_stats() {
        let policy = LruPolicy::new();
        let mut cache = ManagedCache::new(policy, 10);

        cache.insert(1, "a");
        cache.get(&1);
        cache.get(&2);

        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.insertions, 1);
    }
}
