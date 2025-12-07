//! Aggregation kernels: sum, min, max, count

use crate::error::Result;

/// Sum aggregation
pub fn sum_i32(values: &[i32]) -> i32 {
    values.iter().sum()
}

pub fn sum_f64(values: &[f64]) -> f64 {
    values.iter().sum()
}

/// Min aggregation
pub fn min_i32(values: &[i32]) -> Option<i32> {
    values.iter().copied().min()
}

pub fn min_f64(values: &[f64]) -> Option<f64> {
    values.iter().copied().reduce(f64::min)
}

/// Max aggregation
pub fn max_i32(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

pub fn max_f64(values: &[f64]) -> Option<f64> {
    values.iter().copied().reduce(f64::max)
}

/// Count aggregation
pub fn count<T>(values: &[T]) -> usize {
    values.len()
}
