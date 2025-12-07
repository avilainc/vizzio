//! Iterator implementations for cache
use crate::cache::DistributedCache;

impl<K: Ord, V> DistributedCache<K, V> {
    /// Returns an iterator over the keys
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.data.keys()
    }

    /// Returns an iterator over the values
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.data.values()
    }

    /// Returns an iterator over key-value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.data.iter()
    }

    /// Returns a mutable iterator over key-value pairs
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.data.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::cache::DistributedCache;

    #[test]
    fn test_keys_iter() {
        let mut cache = DistributedCache::new();
        cache.insert(1, "a");
        cache.insert(2, "b");

        let keys: Vec<_> = cache.keys().copied().collect();
        assert_eq!(keys, vec![1, 2]);
    }

    #[test]
    fn test_values_iter() {
        let mut cache = DistributedCache::new();
        cache.insert(1, "a");
        cache.insert(2, "b");

        let values: Vec<_> = cache.values().copied().collect();
        assert_eq!(values, vec!["a", "b"]);
    }

    #[test]
    fn test_iter() {
        let mut cache = DistributedCache::new();
        cache.insert(1, "a");
        cache.insert(2, "b");

        let count = cache.iter().count();
        assert_eq!(count, 2);
    }
}
