//! Streaming aggregation operations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AggregationType {
    Sum,
    Count,
    Average,
    Min,
    Max,
    Percentile(f64),
}

pub struct StreamAggregator<K, V> {
    aggregations: HashMap<K, AggregationState<V>>,
}

#[derive(Debug, Clone)]
pub struct AggregationState<V> {
    pub count: usize,
    pub sum: V,
    pub min: V,
    pub max: V,
}

impl<K, V> StreamAggregator<K, V>
where
    K: Eq + std::hash::Hash,
    V: Default + Clone,
{
    pub fn new() -> Self {
        Self {
            aggregations: HashMap::new(),
        }
    }

    pub fn update(&mut self, key: K, value: V) {
        // TODO: Update aggregation state
    }

    pub fn get(&self, key: &K) -> Option<&AggregationState<V>> {
        self.aggregations.get(key)
    }

    pub fn reset(&mut self) {
        self.aggregations.clear();
    }
}

impl<K, V> Default for StreamAggregator<K, V>
where
    K: Eq + std::hash::Hash,
    V: Default + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Moving average calculator
pub struct MovingAverage {
    window_size: usize,
    values: Vec<f64>,
}

impl MovingAverage {
    pub fn new(window_size: usize) -> Self {
        Self {
            window_size,
            values: Vec::new(),
        }
    }

    pub fn add(&mut self, value: f64) -> f64 {
        self.values.push(value);
        if self.values.len() > self.window_size {
            self.values.remove(0);
        }
        self.calculate()
    }

    pub fn calculate(&self) -> f64 {
        if self.values.is_empty() {
            0.0
        } else {
            self.values.iter().sum::<f64>() / self.values.len() as f64
        }
    }
}
