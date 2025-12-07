//! Cache eviction policies
//!
//! # TODO (Priority: P1)
//! - [ ] Add TtlLruPolicy (TTL + LRU hybrid)
//! - [ ] Add TtlLfuPolicy (TTL + LFU hybrid)
//! - [ ] Add AdaptivePolicy (switches between LRU/LFU)
//! - [ ] Add SizeBasedEviction (evict by entry size)
//! - [ ] Add RandomEviction for testing
//! - [ ] Add ARC (Adaptive Replacement Cache)
//! - [ ] Benchmark all policies
extern crate alloc;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;

/// Trait for implementing cache eviction strategies
///
/// # Custom Implementations
/// Implement this trait to create custom eviction policies.
///
/// # TODO
/// - Add async eviction support
/// - Add batch eviction
pub trait EvictionPolicy<K> {
    /// Called when a key is accessed
    fn on_access(&mut self, key: &K);

    /// Called when a key is inserted
    fn on_insert(&mut self, key: K);

    /// Called when a key is removed
    fn on_remove(&mut self, key: &K);

    /// Select a key to evict (returns None if no eviction needed)
    fn select_victim(&self) -> Option<&K>;

    /// Clear all tracking data
    fn clear(&mut self);
}

/// No eviction policy (cache can grow unbounded)
pub struct NoEviction;

impl<K> EvictionPolicy<K> for NoEviction {
    fn on_access(&mut self, _key: &K) {}
    fn on_insert(&mut self, _key: K) {}
    fn on_remove(&mut self, _key: &K) {}
    fn select_victim(&self) -> Option<&K> {
        None
    }
    fn clear(&mut self) {}
}

/// FIFO (First In First Out) eviction policy
pub struct FifoPolicy<K> {
    queue: VecDeque<K>,
}

impl<K> FifoPolicy<K> {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

impl<K: Ord + Clone> EvictionPolicy<K> for FifoPolicy<K> {
    fn on_access(&mut self, _key: &K) {
        // FIFO doesn't care about access
    }

    fn on_insert(&mut self, key: K) {
        self.queue.push_back(key);
    }

    fn on_remove(&mut self, key: &K) {
        self.queue.retain(|k| k != key);
    }

    fn select_victim(&self) -> Option<&K> {
        self.queue.front()
    }

    fn clear(&mut self) {
        self.queue.clear();
    }
}

/// LRU (Least Recently Used) eviction policy
pub struct LruPolicy<K> {
    access_order: VecDeque<K>,
}

impl<K> LruPolicy<K> {
    pub fn new() -> Self {
        Self {
            access_order: VecDeque::new(),
        }
    }
}

impl<K: Ord + Clone> EvictionPolicy<K> for LruPolicy<K> {
    fn on_access(&mut self, key: &K) {
        // Move to back (most recently used)
        self.access_order.retain(|k| k != key);
        self.access_order.push_back(key.clone());
    }

    fn on_insert(&mut self, key: K) {
        self.access_order.push_back(key);
    }

    fn on_remove(&mut self, key: &K) {
        self.access_order.retain(|k| k != key);
    }

    fn select_victim(&self) -> Option<&K> {
        // Front is least recently used
        self.access_order.front()
    }

    fn clear(&mut self) {
        self.access_order.clear();
    }
}

/// LFU (Least Frequently Used) eviction policy
pub struct LfuPolicy<K> {
    frequency: BTreeMap<K, u64>,
}

impl<K> LfuPolicy<K> {
    pub fn new() -> Self {
        Self {
            frequency: BTreeMap::new(),
        }
    }
}

impl<K: Ord + Clone> EvictionPolicy<K> for LfuPolicy<K> {
    fn on_access(&mut self, key: &K) {
        *self.frequency.entry(key.clone()).or_insert(0) += 1;
    }

    fn on_insert(&mut self, key: K) {
        self.frequency.insert(key, 1);
    }

    fn on_remove(&mut self, key: &K) {
        self.frequency.remove(key);
    }

    fn select_victim(&self) -> Option<&K> {
        // Find key with lowest frequency
        self.frequency
            .iter()
            .min_by_key(|(_, &freq)| freq)
            .map(|(k, _)| k)
    }

    fn clear(&mut self) {
        self.frequency.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_eviction() {
        let mut policy = NoEviction;
        policy.on_insert(1);
        policy.on_access(&1);
        assert_eq!(policy.select_victim(), None);
    }

    #[test]
    fn test_fifo_policy() {
        let mut policy = FifoPolicy::new();
        policy.on_insert(1);
        policy.on_insert(2);
        policy.on_insert(3);

        assert_eq!(policy.select_victim(), Some(&1));
        policy.on_remove(&1);
        assert_eq!(policy.select_victim(), Some(&2));
    }

    #[test]
    fn test_lru_policy() {
        let mut policy = LruPolicy::new();
        policy.on_insert(1);
        policy.on_insert(2);
        policy.on_insert(3);

        // Access 1, making it most recent
        policy.on_access(&1);

        // 2 is now least recently used
        assert_eq!(policy.select_victim(), Some(&2));
    }

    #[test]
    fn test_lfu_policy() {
        let mut policy = LfuPolicy::new();
        policy.on_insert(1);
        policy.on_insert(2);
        policy.on_insert(3);

        // Access 1 and 3 multiple times
        policy.on_access(&1);
        policy.on_access(&1);
        policy.on_access(&3);

        // 2 has frequency 1, should be victim
        assert_eq!(policy.select_victim(), Some(&2));
    }
}
