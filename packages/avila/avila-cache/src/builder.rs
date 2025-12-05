//! Builder patterns for convenient cache creation
extern crate alloc;
use crate::cache::ManagedCache;
use crate::eviction::{EvictionPolicy, LruPolicy, LfuPolicy, FifoPolicy, NoEviction};
use crate::error::{CacheError, CacheResult};

/// Builder for creating caches with different eviction policies
pub struct CacheBuilder<K, V> {
    max_capacity: Option<usize>,
    _phantom: core::marker::PhantomData<(K, V)>,
}

impl<K, V> CacheBuilder<K, V> {
    pub fn new() -> Self {
        Self {
            max_capacity: None,
            _phantom: core::marker::PhantomData,
        }
    }

    pub fn max_capacity(mut self, capacity: usize) -> Self {
        self.max_capacity = Some(capacity);
        self
    }
}

impl<K: Ord + Clone, V> CacheBuilder<K, V> {
    /// Build a cache with LRU eviction policy
    pub fn with_lru(self) -> CacheResult<ManagedCache<K, V, LruPolicy<K>>> {
        let capacity = self.max_capacity.ok_or(CacheError::InvalidConfig)?;
        if capacity == 0 {
            return Err(CacheError::InvalidConfig);
        }
        Ok(ManagedCache::new(LruPolicy::new(), capacity))
    }

    /// Build a cache with LFU eviction policy
    pub fn with_lfu(self) -> CacheResult<ManagedCache<K, V, LfuPolicy<K>>> {
        let capacity = self.max_capacity.ok_or(CacheError::InvalidConfig)?;
        if capacity == 0 {
            return Err(CacheError::InvalidConfig);
        }
        Ok(ManagedCache::new(LfuPolicy::new(), capacity))
    }

    /// Build a cache with FIFO eviction policy
    pub fn with_fifo(self) -> CacheResult<ManagedCache<K, V, FifoPolicy<K>>> {
        let capacity = self.max_capacity.ok_or(CacheError::InvalidConfig)?;
        if capacity == 0 {
            return Err(CacheError::InvalidConfig);
        }
        Ok(ManagedCache::new(FifoPolicy::new(), capacity))
    }

    /// Build a cache with no eviction policy
    pub fn with_no_eviction(self) -> CacheResult<ManagedCache<K, V, NoEviction>> {
        let capacity = self.max_capacity.unwrap_or(usize::MAX);
        Ok(ManagedCache::new(NoEviction, capacity))
    }
}

impl<K, V> Default for CacheBuilder<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_lru() {
        let mut cache = CacheBuilder::new()
            .max_capacity(2)
            .with_lru()
            .unwrap();

        cache.insert(1, "a");
        cache.insert(2, "b");
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_builder_validation() {
        let result = CacheBuilder::<i32, &str>::new()
            .max_capacity(0)
            .with_lru();

        assert!(result.is_err());
    }

    #[test]
    fn test_builder_no_capacity() {
        let result = CacheBuilder::<i32, &str>::new()
            .with_lru();

        assert!(result.is_err());
    }
}
