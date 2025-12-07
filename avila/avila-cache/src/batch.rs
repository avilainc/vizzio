//! Batch operations for efficient multi-key access
extern crate alloc;
use alloc::vec::Vec;
use crate::cache::DistributedCache;

/// Batch operation results
pub struct BatchResult<K, V> {
    pub found: Vec<(K, V)>,
    pub missing: Vec<K>,
}

impl<K: Ord + Clone, V: Clone> DistributedCache<K, V> {
    /// Get multiple keys at once
    pub fn get_batch(&self, keys: &[K]) -> BatchResult<K, V> {
        let mut found = Vec::new();
        let mut missing = Vec::new();

        for key in keys {
            if let Some(value) = self.get(key) {
                found.push((key.clone(), value.clone()));
            } else {
                missing.push(key.clone());
            }
        }

        BatchResult { found, missing }
    }

    /// Insert multiple key-value pairs
    pub fn insert_batch(&mut self, pairs: Vec<(K, V)>) {
        for (key, value) in pairs {
            self.insert(key, value);
        }
    }

    /// Remove multiple keys
    pub fn remove_batch(&mut self, keys: &[K]) -> Vec<(K, V)> {
        let mut removed = Vec::new();

        for key in keys {
            if let Some(value) = self.remove(key) {
                removed.push((key.clone(), value));
            }
        }

        removed
    }

    /// Check which keys exist
    pub fn exists_batch(&self, keys: &[K]) -> Vec<bool> {
        keys.iter()
            .map(|key| self.contains_key(key))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_batch() {
        let mut cache = DistributedCache::new();
        cache.insert(1, "a");
        cache.insert(2, "b");
        cache.insert(3, "c");

        let result = cache.get_batch(&[1, 2, 4]);

        assert_eq!(result.found.len(), 2);
        assert_eq!(result.missing.len(), 1);
        assert_eq!(result.missing[0], 4);
    }

    #[test]
    fn test_insert_batch() {
        let mut cache = DistributedCache::new();

        let pairs = vec![
            (1, "a"),
            (2, "b"),
            (3, "c"),
        ];

        cache.insert_batch(pairs);

        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get(&2), Some(&"b"));
    }

    #[test]
    fn test_remove_batch() {
        let mut cache = DistributedCache::new();
        cache.insert(1, "a");
        cache.insert(2, "b");
        cache.insert(3, "c");

        let removed = cache.remove_batch(&[1, 3]);

        assert_eq!(removed.len(), 2);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_exists_batch() {
        let mut cache = DistributedCache::new();
        cache.insert(1, "a");
        cache.insert(3, "c");

        let exists = cache.exists_batch(&[1, 2, 3]);

        assert_eq!(exists, vec![true, false, true]);
    }
}
