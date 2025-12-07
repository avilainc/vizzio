//! Concurrent-safe wrappers using atomic reference counting
//!
//! This module provides thread-safe cache wrappers without requiring std::sync
//!
//! # TODO (Priority: P0)
//! - [ ] Replace RefCell with Mutex for true thread-safety
//! - [ ] Add feature flag: "std" for std::sync::Mutex
//! - [ ] Add feature flag: "spin" for spin::Mutex (no_std)
//! - [ ] Use RwLock for read-heavy workloads
//! - [ ] Add lock-free operations where possible
//! - [ ] Benchmark contention scenarios
//! - [ ] Add concurrent stress tests

extern crate alloc;
use alloc::sync::Arc;
use core::cell::RefCell;
use crate::cache::DistributedCache;

/// A simple shared cache using Arc and RefCell
///
/// **⚠️ WARNING**: Not thread-safe! RefCell panics on concurrent access.
///
/// Note: RefCell provides runtime borrow checking, not thread-safety.
/// For true concurrency, would need Mutex from std or spin crate.
///
/// # TODO
/// Replace with `Arc<Mutex<DistributedCache<K, V>>>` for production use
pub struct SharedCache<K, V> {
    inner: Arc<RwLock<DistributedCache<K, V>>>,
}

#[cfg(any(feature = "std", feature = "parking_lot", feature = "spin"))]
impl<K: Ord, V> SharedCache<K, V> {
    /// Create a new thread-safe shared cache
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(DistributedCache::new())),
        }
    }

    /// Insert a key-value pair (write lock)
    pub fn insert(&self, key: K, value: V) {
        #[cfg(any(feature = "parking_lot", feature = "spin"))]
        {
            self.inner.write().insert(key, value);
        }
        #[cfg(all(feature = "std", not(feature = "parking_lot"), not(feature = "spin")))]
        {
            self.inner.write().unwrap().insert(key, value);
        }
    }

    /// Get a value by key (read lock)
    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone
    {
        #[cfg(any(feature = "parking_lot", feature = "spin"))]
        {
            self.inner.read().get(key).cloned()
        }
        #[cfg(all(feature = "std", not(feature = "parking_lot"), not(feature = "spin")))]
        {
            self.inner.read().unwrap().get(key).cloned()
        }
    }

    /// Remove a key (write lock)
    pub fn remove(&self, key: &K) -> Option<V>
    where
        V: Clone
    {
        #[cfg(any(feature = "parking_lot", feature = "spin"))]
        {
            self.inner.write().remove(key).cloned()
        }
        #[cfg(all(feature = "std", not(feature = "parking_lot"), not(feature = "spin")))]
        {
            self.inner.write().unwrap().remove(key).cloned()
        }
    }

    /// Check if key exists (read lock)
    pub fn contains_key(&self, key: &K) -> bool {
        #[cfg(any(feature = "parking_lot", feature = "spin"))]
        {
            self.inner.read().contains_key(key)
        }
        #[cfg(all(feature = "std", not(feature = "parking_lot"), not(feature = "spin")))]
        {
            self.inner.read().unwrap().contains_key(key)
        }
    }

    /// Batch get operation (single read lock)
    pub fn get_batch(&self, keys: &[K]) -> alloc::vec::Vec<Option<V>>
    where
        V: Clone,
        K: Clone,
    {
        #[cfg(any(feature = "parking_lot", feature = "spin"))]
        {
            let cache = self.inner.read();
            keys.iter().map(|k| cache.get(k).cloned()).collect()
        }
        #[cfg(all(feature = "std", not(feature = "parking_lot"), not(feature = "spin")))]
        {
            let cache = self.inner.read().unwrap();
            keys.iter().map(|k| cache.get(k).cloned()).collect()
        }
    }

    pub fn len(&self) -> usize {
        self.inner.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.borrow().is_empty()
    }

    pub fn clone_handle(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<K, V> Clone for SharedCache<K, V> {
    fn clone(&self) -> Self {
        self.clone_handle()
    }
}

impl<K: Ord, V> Default for SharedCache<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_cache() {
        let cache = SharedCache::new();
        cache.insert(1, "value");

        assert_eq!(cache.get(&1), Some("value"));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_shared_cache_clone() {
        let cache1 = SharedCache::new();
        cache1.insert(1, "value");

        let cache2 = cache1.clone_handle();
        assert_eq!(cache2.get(&1), Some("value"));

        // Both references point to same data
        cache2.insert(2, "another");
        assert_eq!(cache1.get(&2), Some("another"));
    }
}
