//! Extended cache operations
use crate::cache::DistributedCache;

impl<K: Ord, V> DistributedCache<K, V> {
    /// Remove a key from the cache
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    /// Check if a key exists in the cache
    pub fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    /// Clear all entries from the cache
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get the number of entries in the cache
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get a mutable reference to a value
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data.get_mut(key)
    }
}

#[cfg(test)]
mod tests {
    use crate::cache::DistributedCache;

    #[test]
    fn test_remove() {
        let mut cache = DistributedCache::new();
        cache.insert(1, "value");
        assert_eq!(cache.remove(&1), Some("value"));
        assert_eq!(cache.get(&1), None);
    }

    #[test]
    fn test_contains_key() {
        let mut cache = DistributedCache::new();
        cache.insert(1, "value");
        assert!(cache.contains_key(&1));
        assert!(!cache.contains_key(&2));
    }

    #[test]
    fn test_clear() {
        let mut cache = DistributedCache::new();
        cache.insert(1, "a");
        cache.insert(2, "b");
        cache.clear();
        assert!(cache.is_empty());
    }
}
