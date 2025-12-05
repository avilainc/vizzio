//! Cache statistics collection

#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Total number of insertions
    pub insertions: u64,
    /// Total number of evictions
    pub evictions: u64,
}

impl CacheStats {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate hit rate (0.0 to 1.0)
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// Calculate miss rate (0.0 to 1.0)
    pub fn miss_rate(&self) -> f64 {
        1.0 - self.hit_rate()
    }

    /// Record a cache hit
    pub fn record_hit(&mut self) {
        self.hits += 1;
    }

    /// Record a cache miss
    pub fn record_miss(&mut self) {
        self.misses += 1;
    }

    /// Record an insertion
    pub fn record_insertion(&mut self) {
        self.insertions += 1;
    }

    /// Record an eviction
    pub fn record_eviction(&mut self) {
        self.evictions += 1;
    }

    /// Reset all statistics
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_rate() {
        let mut stats = CacheStats::new();
        stats.record_hit();
        stats.record_hit();
        stats.record_miss();

        assert_eq!(stats.hit_rate(), 2.0 / 3.0);
        assert_eq!(stats.miss_rate(), 1.0 / 3.0);
    }

    #[test]
    fn test_reset() {
        let mut stats = CacheStats::new();
        stats.record_hit();
        stats.reset();

        assert_eq!(stats.hits, 0);
        assert_eq!(stats.hit_rate(), 0.0);
    }
}
