//! Time-to-Live (TTL) support for cache entries
extern crate alloc;
use alloc::collections::BTreeMap;
use core::time::Duration;

/// Configurable time source for TTL functionality
///
/// This trait allows different time implementations for different environments:
/// - `SystemTimeSource` for std environments (wall clock time)
/// - `MonotonicTimeSource` for no_std (atomic counter)
/// - `MockTimeSource` for deterministic testing
pub trait TimeSource: Clone + Send + Sync {
    /// Get current timestamp
    fn now() -> Timestamp;
}

/// Timestamp representation
///
/// Represents a point in time as seconds since epoch (std) or
/// a monotonic counter value (no_std).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Timestamp(pub u64);

impl Timestamp {
    /// Create timestamp from raw value
    pub const fn from_secs(secs: u64) -> Self {
        Self(secs)
    }

    /// Get raw seconds value
    pub const fn as_secs(&self) -> u64 {
        self.0
    }

    /// Create current timestamp using default time source
    #[cfg(feature = "std")]
    pub fn now() -> Self {
        SystemTimeSource::now()
    }

    #[cfg(not(feature = "std"))]
    pub fn now() -> Self {
        MonotonicTimeSource::now()
    }

    /// Calculate elapsed time since another timestamp
    pub fn elapsed_since(&self, other: Timestamp) -> Duration {
        let diff = self.0.saturating_sub(other.0);
        Duration::from_secs(diff)
    }

    /// Add a duration to this timestamp
    pub fn add_duration(&self, duration: Duration) -> Self {
        Self(self.0.saturating_add(duration.as_secs()))
    }

    /// Check if this timestamp is after another
    pub fn is_after(&self, other: Timestamp) -> bool {
        self.0 > other.0
    }

    /// Check if this timestamp is before another
    pub fn is_before(&self, other: Timestamp) -> bool {
        self.0 < other.0
    }
}

// Standard library time source
#[cfg(feature = "std")]
#[derive(Debug, Clone, Copy)]
pub struct SystemTimeSource;

#[cfg(feature = "std")]
impl TimeSource for SystemTimeSource {
    fn now() -> Timestamp {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0));
        Timestamp(duration.as_secs())
    }
}

// No-std monotonic counter using atomic
#[cfg(not(feature = "std"))]
use core::sync::atomic::{AtomicU64, Ordering};

#[cfg(not(feature = "std"))]
static MONOTONIC_COUNTER: AtomicU64 = AtomicU64::new(0);

#[cfg(not(feature = "std"))]
#[derive(Debug, Clone, Copy)]
pub struct MonotonicTimeSource;

#[cfg(not(feature = "std"))]
impl TimeSource for MonotonicTimeSource {
    fn now() -> Timestamp {
        Timestamp(MONOTONIC_COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

#[cfg(not(feature = "std"))]
impl MonotonicTimeSource {
    /// Reset the monotonic counter (useful for testing)
    pub fn reset() {
        MONOTONIC_COUNTER.store(0, Ordering::SeqCst);
    }

    /// Set the counter to a specific value
    pub fn set(value: u64) {
        MONOTONIC_COUNTER.store(value, Ordering::SeqCst);
    }
}

// Mock time source for testing
#[derive(Debug, Clone, Copy)]
pub struct MockTimeSource {
    value: u64,
}

impl MockTimeSource {
    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn advance(&mut self, secs: u64) {
        self.value += secs;
    }
}

impl TimeSource for MockTimeSource {
    fn now() -> Timestamp {
        // Note: In real usage, would need thread-local or other mechanism
        // This is simplified for the trait demonstration
        Timestamp(0)
    }
}

/// Entry with expiration time
#[derive(Debug, Clone)]
pub struct TtlEntry<V> {
    pub value: V,
    pub expires_at: Option<Timestamp>,
}

impl<V> TtlEntry<V> {
    pub fn new(value: V) -> Self {
        Self {
            value,
            expires_at: None,
        }
    }

    pub fn with_ttl(value: V, ttl: Duration) -> Self {
        let expires_at = Timestamp::now().add_duration(ttl);
        Self {
            value,
            expires_at: Some(expires_at),
        }
    }

    pub fn is_expired(&self, now: Timestamp) -> bool {
        if let Some(expires_at) = self.expires_at {
            now >= expires_at
        } else {
            false
        }
    }
}

/// Cache with TTL support
pub struct TtlCache<K, V> {
    data: BTreeMap<K, TtlEntry<V>>,
    default_ttl: Option<Duration>,
}

impl<K: Ord, V> TtlCache<K, V> {
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
            default_ttl: None,
        }
    }

    pub fn with_default_ttl(ttl: Duration) -> Self {
        Self {
            data: BTreeMap::new(),
            default_ttl: Some(ttl),
        }
    }

    /// Insert with specific TTL
    pub fn insert_with_ttl(&mut self, key: K, value: V, ttl: Duration) {
        let entry = TtlEntry::with_ttl(value, ttl);
        self.data.insert(key, entry);
    }

    /// Insert with default TTL
    pub fn insert(&mut self, key: K, value: V) {
        let entry = if let Some(ttl) = self.default_ttl {
            TtlEntry::with_ttl(value, ttl)
        } else {
            TtlEntry::new(value)
        };
        self.data.insert(key, entry);
    }

    /// Get value if not expired
    pub fn get(&self, key: &K) -> Option<&V> {
        let now = Timestamp::now();
        self.data.get(key).and_then(|entry| {
            if entry.is_expired(now) {
                None
            } else {
                Some(&entry.value)
            }
        })
    }

    /// Remove expired entries
    pub fn cleanup_expired(&mut self) -> usize {
        let now = Timestamp::now();
        let expired_keys: alloc::vec::Vec<K> = self.data
            .iter()
            .filter(|(_, entry)| entry.is_expired(now))
            .map(|(k, _)| k.clone())
            .collect();

        let count = expired_keys.len();
        for key in expired_keys {
            self.data.remove(&key);
        }
        count
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl<K: Ord, V> Default for TtlCache<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ttl_entry() {
        let entry = TtlEntry::new("value");
        assert!(!entry.is_expired(Timestamp::now()));

        let entry_with_ttl = TtlEntry::with_ttl("value", Duration::from_secs(60));
        assert!(!entry_with_ttl.is_expired(Timestamp::now()));
    }

    #[test]
    fn test_ttl_cache() {
        let mut cache = TtlCache::new();
        cache.insert(1, "value");

        assert_eq!(cache.get(&1), Some(&"value"));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_ttl_cache_with_default() {
        let mut cache = TtlCache::with_default_ttl(Duration::from_secs(300));
        cache.insert(1, "value");

        assert_eq!(cache.get(&1), Some(&"value"));
    }

    #[test]
    fn test_insert_with_specific_ttl() {
        let mut cache = TtlCache::new();
        cache.insert_with_ttl(1, "short", Duration::from_secs(10));
        cache.insert_with_ttl(2, "long", Duration::from_secs(3600));

        assert_eq!(cache.len(), 2);
    }
}
