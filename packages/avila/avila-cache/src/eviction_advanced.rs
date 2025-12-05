//! Advanced eviction policies with hybrid strategies
//!
//! This module extends the basic eviction policies with production-ready
//! hybrid algorithms that combine multiple strategies.

extern crate alloc;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;
use crate::eviction::EvictionPolicy;
use crate::ttl::Timestamp;
use core::time::Duration;

/// TTL + LRU hybrid policy
///
/// Evicts expired entries first, then falls back to LRU for capacity management.
/// Ideal for session caches or time-sensitive data with access patterns.
///
/// # Example
/// ```rust,ignore
/// let policy = TtlLruPolicy::new(Duration::from_secs(300));
/// let mut cache = ManagedCache::new(policy, 100);
/// ```
pub struct TtlLruPolicy<K> {
    access_order: VecDeque<K>,
    expiration: BTreeMap<K, Timestamp>,
    default_ttl: Duration,
}

impl<K> TtlLruPolicy<K> {
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            access_order: VecDeque::new(),
            expiration: BTreeMap::new(),
            default_ttl,
        }
    }

    pub fn with_ttl(&mut self, key: &K, ttl: Duration)
    where K: Clone + Ord
    {
        let expires_at = Timestamp::now().add_duration(ttl);
        self.expiration.insert(key.clone(), expires_at);
    }

    fn find_expired(&self) -> Option<&K> {
        let now = Timestamp::now();
        self.expiration.iter()
            .find(|(_, &expires)| expires <= now)
            .map(|(k, _)| k)
    }
}

impl<K: Ord + Clone> EvictionPolicy<K> for TtlLruPolicy<K> {
    fn on_access(&mut self, key: &K) {
        self.access_order.retain(|k| k != key);
        self.access_order.push_back(key.clone());
    }

    fn on_insert(&mut self, key: K) {
        let expires_at = Timestamp::now().add_duration(self.default_ttl);
        self.expiration.insert(key.clone(), expires_at);
        self.access_order.push_back(key);
    }

    fn on_remove(&mut self, key: &K) {
        self.access_order.retain(|k| k != key);
        self.expiration.remove(key);
    }

    fn select_victim(&self) -> Option<&K> {
        // First try to find expired entry
        if let Some(expired) = self.find_expired() {
            return Some(expired);
        }
        // Fall back to LRU
        self.access_order.front()
    }

    fn clear(&mut self) {
        self.access_order.clear();
        self.expiration.clear();
    }
}

/// TTL + LFU hybrid policy
///
/// Combines time-based expiration with frequency-based eviction.
pub struct TtlLfuPolicy<K> {
    frequency: BTreeMap<K, u64>,
    expiration: BTreeMap<K, Timestamp>,
    default_ttl: Duration,
}

impl<K> TtlLfuPolicy<K> {
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            frequency: BTreeMap::new(),
            expiration: BTreeMap::new(),
            default_ttl,
        }
    }
}

impl<K: Ord + Clone> EvictionPolicy<K> for TtlLfuPolicy<K> {
    fn on_access(&mut self, key: &K) {
        *self.frequency.entry(key.clone()).or_insert(0) += 1;
    }

    fn on_insert(&mut self, key: K) {
        let expires_at = Timestamp::now().add_duration(self.default_ttl);
        self.expiration.insert(key.clone(), expires_at);
        self.frequency.insert(key, 1);
    }

    fn on_remove(&mut self, key: &K) {
        self.frequency.remove(key);
        self.expiration.remove(key);
    }

    fn select_victim(&self) -> Option<&K> {
        let now = Timestamp::now();

        // First check for expired
        if let Some((k, _)) = self.expiration.iter().find(|(_, &exp)| exp <= now) {
            return Some(k);
        }

        // Fall back to least frequent
        self.frequency.iter()
            .min_by_key(|(_, &freq)| freq)
            .map(|(k, _)| k)
    }

    fn clear(&mut self) {
        self.frequency.clear();
        self.expiration.clear();
    }
}

/// Adaptive policy that switches between LRU and LFU
///
/// Monitors hit rates and adapts eviction strategy based on workload patterns.
pub struct AdaptivePolicy<K> {
    lru_order: VecDeque<K>,
    frequency: BTreeMap<K, u64>,
    use_lru: bool,
    sample_size: usize,
    lru_hits: u64,
    lfu_hits: u64,
}

impl<K> AdaptivePolicy<K> {
    pub fn new(sample_size: usize) -> Self {
        Self {
            lru_order: VecDeque::new(),
            frequency: BTreeMap::new(),
            use_lru: true,
            sample_size,
            lru_hits: 0,
            lfu_hits: 0,
        }
    }

    fn adapt(&mut self) {
        if (self.lru_hits + self.lfu_hits) as usize >= self.sample_size {
            self.use_lru = self.lru_hits >= self.lfu_hits;
            self.lru_hits = 0;
            self.lfu_hits = 0;
        }
    }
}

impl<K: Ord + Clone> EvictionPolicy<K> for AdaptivePolicy<K> {
    fn on_access(&mut self, key: &K) {
        // Update both structures
        self.lru_order.retain(|k| k != key);
        self.lru_order.push_back(key.clone());
        *self.frequency.entry(key.clone()).or_insert(0) += 1;
    }

    fn on_insert(&mut self, key: K) {
        self.lru_order.push_back(key.clone());
        self.frequency.insert(key, 1);
        self.adapt();
    }

    fn on_remove(&mut self, key: &K) {
        self.lru_order.retain(|k| k != key);
        self.frequency.remove(key);
    }

    fn select_victim(&self) -> Option<&K> {
        if self.use_lru {
            self.lru_order.front()
        } else {
            self.frequency.iter()
                .min_by_key(|(_, &freq)| freq)
                .map(|(k, _)| k)
        }
    }

    fn clear(&mut self) {
        self.lru_order.clear();
        self.frequency.clear();
        self.lru_hits = 0;
        self.lfu_hits = 0;
    }
}

/// Size-based eviction policy
///
/// Evicts largest entries first when capacity is reached.
/// Useful for caches with variable-sized values.
pub struct SizeBasedPolicy<K> {
    sizes: BTreeMap<K, usize>,
}

impl<K> SizeBasedPolicy<K> {
    pub fn new() -> Self {
        Self {
            sizes: BTreeMap::new(),
        }
    }

    pub fn set_size(&mut self, key: K, size: usize)
    where K: Ord
    {
        self.sizes.insert(key, size);
    }
}

impl<K: Ord + Clone> EvictionPolicy<K> for SizeBasedPolicy<K> {
    fn on_access(&mut self, _key: &K) {}

    fn on_insert(&mut self, key: K) {
        self.sizes.insert(key, 1);
    }

    fn on_remove(&mut self, key: &K) {
        self.sizes.remove(key);
    }

    fn select_victim(&self) -> Option<&K> {
        self.sizes.iter()
            .max_by_key(|(_, &size)| size)
            .map(|(k, _)| k)
    }

    fn clear(&mut self) {
        self.sizes.clear();
    }
}

/// Random eviction policy (for testing/baseline)
pub struct RandomPolicy<K> {
    keys: Vec<K>,
    counter: usize,
}

impl<K> RandomPolicy<K> {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            counter: 0,
        }
    }
}

impl<K: Ord + Clone> EvictionPolicy<K> for RandomPolicy<K> {
    fn on_access(&mut self, _key: &K) {}

    fn on_insert(&mut self, key: K) {
        self.keys.push(key);
    }

    fn on_remove(&mut self, key: &K) {
        self.keys.retain(|k| k != key);
    }

    fn select_victim(&self) -> Option<&K> {
        if self.keys.is_empty() {
            None
        } else {
            // Simple pseudo-random using counter
            let idx = self.counter % self.keys.len();
            Some(&self.keys[idx])
        }
    }

    fn clear(&mut self) {
        self.keys.clear();
        self.counter = 0;
    }
}

impl<K> Default for RandomPolicy<K> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ttl_lru_policy() {
        let mut policy = TtlLruPolicy::new(Duration::from_secs(60));

        policy.on_insert(1);
        policy.on_insert(2);
        policy.on_access(&1);

        assert_eq!(policy.select_victim(), Some(&2));
    }

    #[test]
    fn test_adaptive_policy() {
        let mut policy = AdaptivePolicy::new(10);

        policy.on_insert(1);
        policy.on_insert(2);
        policy.on_insert(3);

        assert!(policy.select_victim().is_some());
    }

    #[test]
    fn test_size_based_policy() {
        let mut policy = SizeBasedPolicy::new();

        policy.on_insert(1);
        policy.set_size(1, 100);
        policy.on_insert(2);
        policy.set_size(2, 200);

        assert_eq!(policy.select_victim(), Some(&2));
    }

    #[test]
    fn test_random_policy() {
        let mut policy = RandomPolicy::new();

        policy.on_insert(1);
        policy.on_insert(2);

        assert!(policy.select_victim().is_some());
    }
}
