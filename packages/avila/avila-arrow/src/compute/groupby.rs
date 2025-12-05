//! Group by operations for aggregation
//!
//! Pure Rust implementation of group by with aggregations.

use crate::error::{Result, ArrowError};
use std::collections::HashMap;

/// Group by result for i32 values
pub struct GroupByResult<T> {
    pub keys: Vec<T>,
    pub aggregates: Vec<f64>,
}

/// Group by with sum aggregation for i32
pub fn group_by_sum_i32(keys: &[i32], values: &[i32]) -> Result<GroupByResult<i32>> {
    if keys.len() != values.len() {
        return Err(ArrowError::InvalidArgument(
            "Keys and values must have same length".to_string()
        ));
    }

    let mut groups: HashMap<i32, i64> = HashMap::new();
    
    for (key, value) in keys.iter().zip(values.iter()) {
        *groups.entry(*key).or_insert(0) += *value as i64;
    }

    let mut result_keys: Vec<i32> = groups.keys().copied().collect();
    result_keys.sort_unstable();

    let aggregates: Vec<f64> = result_keys
        .iter()
        .map(|k| groups[k] as f64)
        .collect();

    Ok(GroupByResult {
        keys: result_keys,
        aggregates,
    })
}

/// Group by with mean aggregation for i32
pub fn group_by_mean_i32(keys: &[i32], values: &[i32]) -> Result<GroupByResult<i32>> {
    if keys.len() != values.len() {
        return Err(ArrowError::InvalidArgument(
            "Keys and values must have same length".to_string()
        ));
    }

    let mut groups: HashMap<i32, (i64, usize)> = HashMap::new();
    
    for (key, value) in keys.iter().zip(values.iter()) {
        let entry = groups.entry(*key).or_insert((0, 0));
        entry.0 += *value as i64;
        entry.1 += 1;
    }

    let mut result_keys: Vec<i32> = groups.keys().copied().collect();
    result_keys.sort_unstable();

    let aggregates: Vec<f64> = result_keys
        .iter()
        .map(|k| {
            let (sum, count) = groups[k];
            sum as f64 / count as f64
        })
        .collect();

    Ok(GroupByResult {
        keys: result_keys,
        aggregates,
    })
}

/// Group by with count aggregation
pub fn group_by_count_i32(keys: &[i32]) -> GroupByResult<i32> {
    let mut groups: HashMap<i32, usize> = HashMap::new();
    
    for key in keys {
        *groups.entry(*key).or_insert(0) += 1;
    }

    let mut result_keys: Vec<i32> = groups.keys().copied().collect();
    result_keys.sort_unstable();

    let aggregates: Vec<f64> = result_keys
        .iter()
        .map(|k| groups[k] as f64)
        .collect();

    GroupByResult {
        keys: result_keys,
        aggregates,
    }
}

/// Group by with min aggregation for i32
pub fn group_by_min_i32(keys: &[i32], values: &[i32]) -> Result<GroupByResult<i32>> {
    if keys.len() != values.len() {
        return Err(ArrowError::InvalidArgument(
            "Keys and values must have same length".to_string()
        ));
    }

    let mut groups: HashMap<i32, i32> = HashMap::new();
    
    for (key, value) in keys.iter().zip(values.iter()) {
        groups.entry(*key)
            .and_modify(|min| *min = (*min).min(*value))
            .or_insert(*value);
    }

    let mut result_keys: Vec<i32> = groups.keys().copied().collect();
    result_keys.sort_unstable();

    let aggregates: Vec<f64> = result_keys
        .iter()
        .map(|k| groups[k] as f64)
        .collect();

    Ok(GroupByResult {
        keys: result_keys,
        aggregates,
    })
}

/// Group by with max aggregation for i32
pub fn group_by_max_i32(keys: &[i32], values: &[i32]) -> Result<GroupByResult<i32>> {
    if keys.len() != values.len() {
        return Err(ArrowError::InvalidArgument(
            "Keys and values must have same length".to_string()
        ));
    }

    let mut groups: HashMap<i32, i32> = HashMap::new();
    
    for (key, value) in keys.iter().zip(values.iter()) {
        groups.entry(*key)
            .and_modify(|max| *max = (*max).max(*value))
            .or_insert(*value);
    }

    let mut result_keys: Vec<i32> = groups.keys().copied().collect();
    result_keys.sort_unstable();

    let aggregates: Vec<f64> = result_keys
        .iter()
        .map(|k| groups[k] as f64)
        .collect();

    Ok(GroupByResult {
        keys: result_keys,
        aggregates,
    })
}

/// Group by with sum for f64 values
pub fn group_by_sum_f64(keys: &[i32], values: &[f64]) -> Result<GroupByResult<i32>> {
    if keys.len() != values.len() {
        return Err(ArrowError::InvalidArgument(
            "Keys and values must have same length".to_string()
        ));
    }

    let mut groups: HashMap<i32, f64> = HashMap::new();
    
    for (key, value) in keys.iter().zip(values.iter()) {
        *groups.entry(*key).or_insert(0.0) += value;
    }

    let mut result_keys: Vec<i32> = groups.keys().copied().collect();
    result_keys.sort_unstable();

    let aggregates: Vec<f64> = result_keys
        .iter()
        .map(|k| groups[k])
        .collect();

    Ok(GroupByResult {
        keys: result_keys,
        aggregates,
    })
}

/// Group by with mean for f64 values
pub fn group_by_mean_f64(keys: &[i32], values: &[f64]) -> Result<GroupByResult<i32>> {
    if keys.len() != values.len() {
        return Err(ArrowError::InvalidArgument(
            "Keys and values must have same length".to_string()
        ));
    }

    let mut groups: HashMap<i32, (f64, usize)> = HashMap::new();
    
    for (key, value) in keys.iter().zip(values.iter()) {
        let entry = groups.entry(*key).or_insert((0.0, 0));
        entry.0 += value;
        entry.1 += 1;
    }

    let mut result_keys: Vec<i32> = groups.keys().copied().collect();
    result_keys.sort_unstable();

    let aggregates: Vec<f64> = result_keys
        .iter()
        .map(|k| {
            let (sum, count) = groups[k];
            sum / count as f64
        })
        .collect();

    Ok(GroupByResult {
        keys: result_keys,
        aggregates,
    })
}

/// Group by with multiple aggregations
pub struct MultiAggResult<T> {
    pub keys: Vec<T>,
    pub sum: Vec<f64>,
    pub mean: Vec<f64>,
    pub min: Vec<f64>,
    pub max: Vec<f64>,
    pub count: Vec<usize>,
}

/// Group by with multiple aggregations at once
pub fn group_by_multi_i32(keys: &[i32], values: &[i32]) -> Result<MultiAggResult<i32>> {
    if keys.len() != values.len() {
        return Err(ArrowError::InvalidArgument(
            "Keys and values must have same length".to_string()
        ));
    }

    #[derive(Default)]
    struct Stats {
        sum: i64,
        count: usize,
        min: i32,
        max: i32,
    }

    let mut groups: HashMap<i32, Stats> = HashMap::new();
    
    for (key, value) in keys.iter().zip(values.iter()) {
        groups.entry(*key)
            .and_modify(|stats| {
                stats.sum += *value as i64;
                stats.count += 1;
                stats.min = stats.min.min(*value);
                stats.max = stats.max.max(*value);
            })
            .or_insert(Stats {
                sum: *value as i64,
                count: 1,
                min: *value,
                max: *value,
            });
    }

    let mut result_keys: Vec<i32> = groups.keys().copied().collect();
    result_keys.sort_unstable();

    let sum: Vec<f64> = result_keys.iter().map(|k| groups[k].sum as f64).collect();
    let mean: Vec<f64> = result_keys.iter().map(|k| {
        let stats = &groups[k];
        stats.sum as f64 / stats.count as f64
    }).collect();
    let min: Vec<f64> = result_keys.iter().map(|k| groups[k].min as f64).collect();
    let max: Vec<f64> = result_keys.iter().map(|k| groups[k].max as f64).collect();
    let count: Vec<usize> = result_keys.iter().map(|k| groups[k].count).collect();

    Ok(MultiAggResult {
        keys: result_keys,
        sum,
        mean,
        min,
        max,
        count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_by_sum() {
        let keys = vec![1, 2, 1, 2, 1];
        let values = vec![10, 20, 15, 25, 5];
        
        let result = group_by_sum_i32(&keys, &values).unwrap();
        assert_eq!(result.keys, vec![1, 2]);
        assert_eq!(result.aggregates, vec![30.0, 45.0]);
    }

    #[test]
    fn test_group_by_mean() {
        let keys = vec![1, 2, 1, 2, 1];
        let values = vec![10, 20, 20, 30, 30];
        
        let result = group_by_mean_i32(&keys, &values).unwrap();
        assert_eq!(result.keys, vec![1, 2]);
        assert_eq!(result.aggregates, vec![20.0, 25.0]);
    }

    #[test]
    fn test_group_by_count() {
        let keys = vec![1, 2, 1, 3, 2, 1];
        
        let result = group_by_count_i32(&keys);
        assert_eq!(result.keys, vec![1, 2, 3]);
        assert_eq!(result.aggregates, vec![3.0, 2.0, 1.0]);
    }

    #[test]
    fn test_group_by_min_max() {
        let keys = vec![1, 2, 1, 2];
        let values = vec![10, 30, 5, 40];
        
        let min_result = group_by_min_i32(&keys, &values).unwrap();
        assert_eq!(min_result.keys, vec![1, 2]);
        assert_eq!(min_result.aggregates, vec![5.0, 30.0]);
        
        let max_result = group_by_max_i32(&keys, &values).unwrap();
        assert_eq!(max_result.keys, vec![1, 2]);
        assert_eq!(max_result.aggregates, vec![10.0, 40.0]);
    }

    #[test]
    fn test_group_by_multi() {
        let keys = vec![1, 2, 1, 2, 1];
        let values = vec![10, 20, 30, 40, 50];
        
        let result = group_by_multi_i32(&keys, &values).unwrap();
        assert_eq!(result.keys, vec![1, 2]);
        assert_eq!(result.sum, vec![90.0, 60.0]);
        assert_eq!(result.mean, vec![30.0, 30.0]);
        assert_eq!(result.min, vec![10.0, 20.0]);
        assert_eq!(result.max, vec![50.0, 40.0]);
        assert_eq!(result.count, vec![3, 2]);
    }

    #[test]
    fn test_group_by_f64() {
        let keys = vec![1, 2, 1, 2];
        let values = vec![1.5, 2.5, 3.5, 4.5];
        
        let result = group_by_sum_f64(&keys, &values).unwrap();
        assert_eq!(result.keys, vec![1, 2]);
        assert_eq!(result.aggregates, vec![5.0, 7.0]);
    }
}
