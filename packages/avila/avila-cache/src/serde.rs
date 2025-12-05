//! Serialization support for cache
//!
//! This module provides comprehensive serialization/deserialization support
//! for cache structures using serde. All functionality is behind feature flags
//! to maintain no_std compatibility.
//!
//! # Features
//! - `serde` - Enables basic serialization support
//! - `serde_json` - JSON format support
//! - `bincode` - Binary format support
//!
//! # Examples
//!
//! ```rust,ignore
//! #[cfg(feature = "serde")]
//! use avila_cache::DistributedCache;
//!
//! let mut cache = DistributedCache::new();
//! cache.insert("key", "value");
//!
//! // Snapshot to bytes
//! let snapshot = cache.snapshot_bincode().unwrap();
//!
//! // Restore from bytes
//! let restored = DistributedCache::restore_bincode(&snapshot).unwrap();
//! ```

extern crate alloc;
use alloc::vec::Vec;
use crate::cache::DistributedCache;
use crate::error::{CacheError, CacheResult};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Base serialization support
impl<K, V> DistributedCache<K, V> {
    /// Get the number of entries (useful for serialization sizing)
    pub fn capacity_hint(&self) -> usize {
        self.data.len()
    }
}

// Serde implementation
#[cfg(feature = "serde")]
impl<K, V> Serialize for DistributedCache<K, V>
where
    K: Serialize + Ord,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.data.len()))?;
        for (k, v) in self.data.iter() {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, K, V> Deserialize<'de> for DistributedCache<K, V>
where
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use alloc::collections::BTreeMap;
        let data = BTreeMap::<K, V>::deserialize(deserializer)?;
        Ok(DistributedCache {
            data,
            config: None,
            stats: None,
        })
    }
}

// JSON format support
#[cfg(feature = "serde_json")]
impl<K, V> DistributedCache<K, V>
where
    K: Serialize + for<'de> Deserialize<'de> + Ord,
    V: Serialize + for<'de> Deserialize<'de>,
{
    /// Create a JSON snapshot of the cache
    pub fn snapshot_json(&self) -> CacheResult<Vec<u8>> {
        serde_json::to_vec(&self)
            .map_err(|_| CacheError::SerializationError)
    }

    /// Create a pretty-printed JSON snapshot
    pub fn snapshot_json_pretty(&self) -> CacheResult<Vec<u8>> {
        serde_json::to_vec_pretty(&self)
            .map_err(|_| CacheError::SerializationError)
    }

    /// Restore cache from JSON snapshot
    pub fn restore_json(data: &[u8]) -> CacheResult<Self> {
        serde_json::from_slice(data)
            .map_err(|_| CacheError::SerializationError)
    }
}

// Bincode format support
#[cfg(feature = "bincode")]
impl<K, V> DistributedCache<K, V>
where
    K: Serialize + for<'de> Deserialize<'de> + Ord,
    V: Serialize + for<'de> Deserialize<'de>,
{
    /// Create a binary snapshot using bincode (compact and fast)
    pub fn snapshot_bincode(&self) -> CacheResult<Vec<u8>> {
        bincode::serialize(&self)
            .map_err(|_| CacheError::SerializationError)
    }

    /// Restore cache from bincode snapshot
    pub fn restore_bincode(data: &[u8]) -> CacheResult<Self> {
        bincode::deserialize(data)
            .map_err(|_| CacheError::SerializationError)
    }
}

// Generic snapshot/restore API
#[cfg(feature = "serde")]
impl<K, V> DistributedCache<K, V>
where
    K: Serialize + for<'de> Deserialize<'de> + Ord + Clone,
    V: Serialize + for<'de> Deserialize<'de> + Clone,
{
    /// Export cache entries as a vector of key-value pairs
    pub fn export_entries(&self) -> Vec<(K, V)> {
        self.data
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Import entries from a vector, replacing existing data
    pub fn import_entries(&mut self, entries: Vec<(K, V)>) {
        self.data.clear();
        for (k, v) in entries {
            self.data.insert(k, v);
        }
    }

    /// Merge entries from another cache
    pub fn merge(&mut self, other: &Self) {
        for (k, v) in other.data.iter() {
            self.data.insert(k.clone(), v.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capacity_hint() {
        let mut cache = DistributedCache::new();
        cache.insert(1, "a");
        cache.insert(2, "b");
        assert_eq!(cache.capacity_hint(), 2);
    }

    #[test]
    fn test_export_import() {
        let mut cache = DistributedCache::new();
        cache.insert(1, "a");
        cache.insert(2, "b");

        let entries = cache.export_entries();
        assert_eq!(entries.len(), 2);

        let mut cache2 = DistributedCache::new();
        cache2.import_entries(entries);
        assert_eq!(cache2.get(&1), Some(&"a"));
        assert_eq!(cache2.get(&2), Some(&"b"));
    }

    #[test]
    fn test_merge() {
        let mut cache1 = DistributedCache::new();
        cache1.insert(1, "a");

        let mut cache2 = DistributedCache::new();
        cache2.insert(2, "b");

        cache1.merge(&cache2);
        assert_eq!(cache1.len(), 2);
        assert_eq!(cache1.get(&2), Some(&"b"));
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn test_json_roundtrip() {
        let mut cache = DistributedCache::new();
        cache.insert("key1", 100);
        cache.insert("key2", 200);

        let snapshot = cache.snapshot_json().unwrap();
        let restored = DistributedCache::restore_json(&snapshot).unwrap();

        assert_eq!(restored.get(&"key1"), Some(&100));
        assert_eq!(restored.get(&"key2"), Some(&200));
    }

    #[cfg(feature = "bincode")]
    #[test]
    fn test_bincode_roundtrip() {
        let mut cache = DistributedCache::new();
        cache.insert(1u64, "value1");
        cache.insert(2u64, "value2");

        let snapshot = cache.snapshot_bincode().unwrap();
        let restored = DistributedCache::restore_bincode(&snapshot).unwrap();

        assert_eq!(restored.get(&1), Some(&"value1"));
        assert_eq!(restored.get(&2), Some(&"value2"));
        assert_eq!(restored.len(), 2);
    }
}
