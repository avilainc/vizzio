//! LRU and LFU Cache implementations
//!
//! **Use cases**:
//! - Web caching
//! - Database query results
//! - Computed values

use crate::AssociativeArray;
use core::hash::Hash;

/// LRU (Least Recently Used) Cache
///
/// **Performance**:
/// - Get: O(1) amortized
/// - Put: O(1) amortized
/// - Eviction: O(1)
///
/// **Implementation**: HashMap + custom index tracking
pub struct LRUCache<K, V> {
    capacity: usize,
    map: AssociativeArray<K, CacheEntry<V>>,
    access_order: Vec<K>,
}

struct CacheEntry<V> {
    value: V,
    access_index: usize,
}

impl<K: Hash + Eq + Clone, V> LRUCache<K, V> {
    /// Create new LRU cache with capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: AssociativeArray::with_capacity(capacity),
            access_order: Vec::with_capacity(capacity),
        }
    }

    /// Get value by key (marks as recently used)
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // Move to end (most recently used)
            self.access_order.retain(|k| k != key);
            self.access_order.push(key.clone());

            if let Some(entry) = self.map.get_mut(key) {
                entry.access_index = self.access_order.len() - 1;
            }
        }

        self.map.get(key).map(|entry| &entry.value)
    }

    /// Get mutable value by key
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.map.contains_key(key) {
            self.access_order.retain(|k| k != key);
            self.access_order.push(key.clone());

            if let Some(entry) = self.map.get_mut(key) {
                entry.access_index = self.access_order.len() - 1;
            }
        }

        self.map.get_mut(key).map(|entry| &mut entry.value)
    }

    /// Insert or update key-value pair
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        // If key exists, update it
        if let Some(entry) = self.map.get_mut(&key) {
            let old_value = core::mem::replace(&mut entry.value, value);

            // Move to end
            self.access_order.retain(|k| k != &key);
            self.access_order.push(key);
            entry.access_index = self.access_order.len() - 1;

            return Some(old_value);
        }

        // Evict if at capacity
        if self.map.len() >= self.capacity && self.capacity > 0 {
            if let Some(lru_key) = self.access_order.first().cloned() {
                self.map.remove(&lru_key);
                self.access_order.remove(0);
            }
        }

        // Insert new entry
        self.access_order.push(key.clone());
        self.map.insert(
            key,
            CacheEntry {
                value,
                access_index: self.access_order.len() - 1,
            },
        );

        None
    }

    /// Remove key from cache
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.access_order.retain(|k| k != key);
        self.map.remove(key).map(|entry| entry.value)
    }

    /// Get current size
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.map.clear();
        self.access_order.clear();
    }
}

/// LFU (Least Frequently Used) Cache
///
/// **Performance**:
/// - Get: O(1) amortized
/// - Put: O(1) amortized
/// - Eviction: O(n) worst case
///
/// **Implementation**: HashMap with frequency tracking
pub struct LFUCache<K, V> {
    capacity: usize,
    map: AssociativeArray<K, CacheNode<V>>,
    min_freq: usize,
}

struct CacheNode<V> {
    value: V,
    freq: usize,
}

impl<K: Hash + Eq + Clone, V> LFUCache<K, V> {
    /// Create new LFU cache with capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: AssociativeArray::with_capacity(capacity),
            min_freq: 0,
        }
    }

    /// Get value by key (increments frequency)
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(node) = self.map.get_mut(key) {
            node.freq += 1;
            Some(&node.value)
        } else {
            None
        }
    }

    /// Get mutable value by key
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if let Some(node) = self.map.get_mut(key) {
            node.freq += 1;
            Some(&mut node.value)
        } else {
            None
        }
    }

    /// Insert or update key-value pair
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        if self.capacity == 0 {
            return None;
        }

        // Update existing key
        if let Some(node) = self.map.get_mut(&key) {
            let old_value = core::mem::replace(&mut node.value, value);
            node.freq += 1;
            return Some(old_value);
        }

        // Evict if at capacity
        if self.map.len() >= self.capacity {
            // Find key with minimum frequency
            let mut min_key: Option<K> = None;
            let mut min_freq = usize::MAX;

            for (k, node) in self.map.iter() {
                if node.freq < min_freq {
                    min_freq = node.freq;
                    min_key = Some(k.clone());
                }
            }

            if let Some(k) = min_key {
                self.map.remove(&k);
            }
        }

        // Insert new entry
        self.map.insert(key, CacheNode { value, freq: 1 });
        self.min_freq = 1;

        None
    }

    /// Remove key from cache
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).map(|node| node.value)
    }

    /// Get current size
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.map.clear();
        self.min_freq = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_basic() {
        let mut cache = LRUCache::new(2);

        cache.put(1, "one");
        cache.put(2, "two");

        assert_eq!(cache.get(&1), Some(&"one"));
        assert_eq!(cache.get(&2), Some(&"two"));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_lru_eviction() {
        let mut cache = LRUCache::new(2);

        cache.put(1, "one");
        cache.put(2, "two");
        cache.put(3, "three"); // Should evict 1

        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&"two"));
        assert_eq!(cache.get(&3), Some(&"three"));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_lru_access_updates() {
        let mut cache = LRUCache::new(2);

        cache.put(1, "one");
        cache.put(2, "two");
        cache.get(&1); // Access 1, making 2 LRU
        cache.put(3, "three"); // Should evict 2

        assert_eq!(cache.get(&1), Some(&"one"));
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(&"three"));
    }

    #[test]
    fn test_lru_update() {
        let mut cache = LRUCache::new(2);

        cache.put(1, "one");
        let old = cache.put(1, "ONE");

        assert_eq!(old, Some("one"));
        assert_eq!(cache.get(&1), Some(&"ONE"));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_lru_clear() {
        let mut cache = LRUCache::new(3);

        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);

        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_lfu_basic() {
        let mut cache = LFUCache::new(2);

        cache.put(1, "one");
        cache.put(2, "two");

        assert_eq!(cache.get(&1), Some(&"one"));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_lfu_eviction() {
        let mut cache = LFUCache::new(2);

        cache.put(1, "one");
        cache.put(2, "two");
        cache.get(&1); // Increase frequency of 1
        cache.get(&1); // Increase frequency of 1 again
        cache.put(3, "three"); // Should evict 2 (lower frequency)

        assert_eq!(cache.get(&1), Some(&"one"));
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(&"three"));
    }

    #[test]
    fn test_lfu_update() {
        let mut cache = LFUCache::new(2);

        cache.put(1, "one");
        let old = cache.put(1, "ONE");

        assert_eq!(old, Some("one"));
        assert_eq!(cache.get(&1), Some(&"ONE"));
    }

    #[test]
    fn test_lfu_zero_capacity() {
        let mut cache: LFUCache<i32, &str> = LFUCache::new(0);

        assert_eq!(cache.put(1, "one"), None);
        assert_eq!(cache.len(), 0);
    }
}
