//! Hash functions for arrays - pure Rust implementation

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Custom hasher for floating point values
pub struct FloatHasher {
    hash: u64,
}

impl FloatHasher {
    pub fn new() -> Self {
        Self { hash: 0 }
    }
}

impl Default for FloatHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for FloatHasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.hash = self.hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
    }
}

/// Hash an i32 array
pub fn hash_i32(values: &[i32]) -> Vec<u64> {
    values
        .iter()
        .map(|&v| {
            let mut hasher = FloatHasher::new();
            v.hash(&mut hasher);
            hasher.finish()
        })
        .collect()
}

/// Hash an i64 array
pub fn hash_i64(values: &[i64]) -> Vec<u64> {
    values
        .iter()
        .map(|&v| {
            let mut hasher = FloatHasher::new();
            v.hash(&mut hasher);
            hasher.finish()
        })
        .collect()
}

/// Hash an f64 array (treating NaN as equal)
pub fn hash_f64(values: &[f64]) -> Vec<u64> {
    values
        .iter()
        .map(|&v| {
            let mut hasher = FloatHasher::new();
            if v.is_nan() {
                0_u64.hash(&mut hasher);
            } else {
                v.to_bits().hash(&mut hasher);
            }
            hasher.finish()
        })
        .collect()
}

/// Hash a string array
pub fn hash_string(values: &[String]) -> Vec<u64> {
    values
        .iter()
        .map(|v| {
            let mut hasher = FloatHasher::new();
            v.hash(&mut hasher);
            hasher.finish()
        })
        .collect()
}

/// Count unique values in an i32 array
pub fn unique_count_i32(values: &[i32]) -> usize {
    let mut seen = HashMap::new();
    for &v in values {
        *seen.entry(v).or_insert(0) += 1;
    }
    seen.len()
}

/// Get unique values from an i32 array
pub fn unique_i32(values: &[i32]) -> Vec<i32> {
    let mut seen = HashMap::new();
    let mut result = Vec::new();

    for &v in values {
        if seen.insert(v, ()).is_none() {
            result.push(v);
        }
    }
    result
}

/// Get unique values from an i64 array
pub fn unique_i64(values: &[i64]) -> Vec<i64> {
    let mut seen = HashMap::new();
    let mut result = Vec::new();

    for &v in values {
        if seen.insert(v, ()).is_none() {
            result.push(v);
        }
    }
    result
}

/// Get value counts (histogram) for an i32 array
pub fn value_counts_i32(values: &[i32]) -> HashMap<i32, usize> {
    let mut counts = HashMap::new();
    for &v in values {
        *counts.entry(v).or_insert(0) += 1;
    }
    counts
}

/// Get value counts for a string array
pub fn value_counts_string(values: &[String]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for v in values {
        *counts.entry(v.clone()).or_insert(0) += 1;
    }
    counts
}

/// Find mode (most common value) in an i32 array
pub fn mode_i32(values: &[i32]) -> Option<i32> {
    let counts = value_counts_i32(values);
    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_i32() {
        let values = vec![1, 2, 2, 3, 3, 3, 4];
        let unique = unique_i32(&values);
        assert_eq!(unique, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_unique_count_i32() {
        let values = vec![1, 2, 2, 3, 3, 3, 4];
        assert_eq!(unique_count_i32(&values), 4);
    }

    #[test]
    fn test_value_counts_i32() {
        let values = vec![1, 2, 2, 3, 3, 3];
        let counts = value_counts_i32(&values);
        assert_eq!(counts.get(&1), Some(&1));
        assert_eq!(counts.get(&2), Some(&2));
        assert_eq!(counts.get(&3), Some(&3));
    }

    #[test]
    fn test_mode_i32() {
        let values = vec![1, 2, 2, 3, 3, 3, 4];
        assert_eq!(mode_i32(&values), Some(3));
    }

    #[test]
    fn test_hash_i32() {
        let values = vec![1, 2, 3, 4, 5];
        let hashes = hash_i32(&values);
        assert_eq!(hashes.len(), 5);
        // Same values should have same hashes
        assert_eq!(hash_i32(&[1])[0], hash_i32(&[1])[0]);
    }
}
