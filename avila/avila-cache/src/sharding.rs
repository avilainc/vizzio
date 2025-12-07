//! Sharded cache for improved concurrency
//!
//! # TODO (Priority: P1)
//! - [ ] Add per-shard statistics
//! - [ ] Support dynamic resizing/rebalancing
//! - [ ] Configurable hash function
//! - [ ] Add consistent hashing option
//! - [ ] Lock-free shard selection
//! - [ ] Parallel iteration across shards
//! - [ ] Shard affinity for better cache locality
extern crate alloc;
use alloc::vec::Vec;
use crate::cache::DistributedCache;
use core::hash::{Hash, Hasher};

/// A cache divided into multiple shards for better concurrency
///
/// # TODO
/// - Support custom shard count validation
/// - Add shard_count() getter
pub struct ShardedCache<K, V> {
    shards: Vec<DistributedCache<K, V>>,
    shard_count: usize,
}

impl<K: Ord + Hash, V> ShardedCache<K, V> {
    pub fn new(shard_count: usize) -> Self {
        let mut shards = Vec::new();
        for _ in 0..shard_count {
            shards.push(DistributedCache::new());
        }

        Self {
            shards,
            shard_count,
        }
    }

    fn get_shard_index(&self, key: &K) -> usize {
        let mut hasher = SipHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.shard_count
    }

    fn get_shard(&self, key: &K) -> &DistributedCache<K, V> {
        let index = self.get_shard_index(key);
        &self.shards[index]
    }

    fn get_shard_mut(&mut self, key: &K) -> &mut DistributedCache<K, V> {
        let index = self.get_shard_index(key);
        &mut self.shards[index]
    }

    pub fn insert(&mut self, key: K, value: V) {
        let shard = self.get_shard_mut(&key);
        shard.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let shard = self.get_shard(key);
        shard.get(key)
    }

    pub fn total_len(&self) -> usize {
        self.shards.iter().map(|s| s.len()).sum()
    }
}

// Simple hasher for demonstration
struct SipHasher {
    state: u64,
}

impl SipHasher {
    fn new() -> Self {
        Self { state: 0 }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

impl Hasher for SipHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = self.state.wrapping_mul(31).wrapping_add(byte as u64);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sharded_cache() {
        let mut cache = ShardedCache::new(4);
        cache.insert(1, "a");
        cache.insert(2, "b");
        cache.insert(3, "c");

        assert_eq!(cache.get(&1), Some(&"a"));
        assert_eq!(cache.get(&2), Some(&"b"));
        assert_eq!(cache.total_len(), 3);
    }
}
