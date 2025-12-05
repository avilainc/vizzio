//! Utility traits and implementations
//!
//! Provides core traits for cache components and advanced metrics collection.

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Trait for types that can be used as cache keys
pub trait CacheKey: Ord + Clone {}

// Blanket implementation for all types that meet requirements
impl<T: Ord + Clone> CacheKey for T {}

/// Trait for types that can be used as cache values
pub trait CacheValue: Clone {}

// Blanket implementation
impl<T: Clone> CacheValue for T {}

/// Trait for metrics collection with labels support
pub trait Metrics {
    /// Record an operation occurrence
    fn record_operation(&mut self, operation: &str);

    /// Get count for a specific operation
    fn get_count(&self, operation: &str) -> u64;

    /// Record operation with labels
    fn record_labeled(&mut self, operation: &str, labels: &[(&str, &str)]);

    /// Record a value (for histograms)
    fn record_value(&mut self, metric: &str, value: f64);

    /// Get all metric names
    fn metric_names(&self) -> Vec<String>;

    /// Export metrics in Prometheus format
    fn export_prometheus(&self) -> String;
}

/// Advanced metrics with histogram support
#[derive(Debug, Clone)]
pub struct AdvancedMetrics {
    counters: BTreeMap<String, u64>,
    histograms: BTreeMap<String, Histogram>,
    labeled_counters: BTreeMap<String, BTreeMap<String, u64>>,
}

impl AdvancedMetrics {
    pub fn new() -> Self {
        Self {
            counters: BTreeMap::new(),
            histograms: BTreeMap::new(),
            labeled_counters: BTreeMap::new(),
        }
    }

    /// Get histogram for a metric
    pub fn histogram(&self, name: &str) -> Option<&Histogram> {
        self.histograms.get(name)
    }
}

impl Default for AdvancedMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics for AdvancedMetrics {
    fn record_operation(&mut self, operation: &str) {
        *self.counters.entry(operation.into()).or_insert(0) += 1;
    }

    fn get_count(&self, operation: &str) -> u64 {
        self.counters.get(operation).copied().unwrap_or(0)
    }

    fn record_labeled(&mut self, operation: &str, labels: &[(&str, &str)]) {
        let label_key = labels.iter()
            .map(|(k, v)| alloc::format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(",");
        let full_key = alloc::format!("{}:{}", operation, label_key);
        *self.labeled_counters.entry(full_key).or_insert_with(BTreeMap::new)
            .entry(operation.into()).or_insert(0) += 1;
    }

    fn record_value(&mut self, metric: &str, value: f64) {
        self.histograms.entry(metric.into())
            .or_insert_with(Histogram::new)
            .record(value);
    }

    fn metric_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        names.extend(self.counters.keys().cloned());
        names.extend(self.histograms.keys().cloned());
        names
    }

    fn export_prometheus(&self) -> String {
        let mut output = String::new();
        for (name, count) in &self.counters {
            output.push_str(&alloc::format!("{}{{}} {}\n", name, count));
        }
        for (name, hist) in &self.histograms {
            output.push_str(&alloc::format!(
                "{}_sum {{}} {}\n{}_count {{}} {}\n",
                name, hist.sum, name, hist.count
            ));
        }
        output
    }
}

/// Simple histogram for latency tracking
#[derive(Debug, Clone)]
pub struct Histogram {
    pub count: u64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    values: Vec<f64>,
}

impl Histogram {
    pub fn new() -> Self {
        Self {
            count: 0,
            sum: 0.0,
            min: f64::MAX,
            max: f64::MIN,
            values: Vec::new(),
        }
    }

    pub fn record(&mut self, value: f64) {
        self.count += 1;
        self.sum += value;
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.values.push(value);
    }

    pub fn mean(&self) -> f64 {
        if self.count > 0 {
            self.sum / self.count as f64
        } else {
            0.0
        }
    }

    pub fn percentile(&mut self, p: f64) -> f64 {
        if self.values.is_empty() {
            return 0.0;
        }
        self.values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let idx = ((p / 100.0) * self.values.len() as f64) as usize;
        self.values[idx.min(self.values.len() - 1)]
    }
}

impl Default for Histogram {
    fn default() -> Self {
        Self::new()
    }
}

// Implement Metrics for CacheStats
impl Metrics for crate::stats::CacheStats {
    fn record_operation(&mut self, operation: &str) {
        match operation {
            "hit" => self.record_hit(),
            "miss" => self.record_miss(),
            "insert" => self.record_insertion(),
            "evict" => self.record_eviction(),
            _ => {}
        }
    }

    fn get_count(&self, operation: &str) -> u64 {
        match operation {
            "hit" => self.hits,
            "miss" => self.misses,
            "insert" => self.insertions,
            "evict" => self.evictions,
            _ => 0,
        }
    }

    fn record_labeled(&mut self, operation: &str, _labels: &[(&str, &str)]) {
        self.record_operation(operation);
    }

    fn record_value(&mut self, _metric: &str, _value: f64) {
        // CacheStats doesn't support histograms yet
    }

    fn metric_names(&self) -> Vec<String> {
        vec!["hit".into(), "miss".into(), "insert".into(), "evict".into()]
    }

    fn export_prometheus(&self) -> String {
        alloc::format!(
            "cache_hits{{}} {}\ncache_misses{{}} {}\ncache_insertions{{}} {}\ncache_evictions{{}} {}\n",
            self.hits, self.misses, self.insertions, self.evictions
        )
    }
}#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_key_trait() {
        fn accepts_key<K: CacheKey>(_key: K) {}

        accepts_key(1);
        accepts_key("string");
    }

    #[test]
    fn test_cache_value_trait() {
        fn accepts_value<V: CacheValue>(_value: V) {}

        accepts_value(42);
        accepts_value("value");
    }
}
