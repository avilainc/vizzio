//! Type casting operations

use crate::error::Result;

/// Cast i32 to f64
pub fn cast_i32_to_f64(values: &[i32]) -> Vec<f64> {
    values.iter().map(|&v| v as f64).collect()
}

/// Cast f64 to i32 (with truncation)
pub fn cast_f64_to_i32(values: &[f64]) -> Vec<i32> {
    values.iter().map(|&v| v as i32).collect()
}

/// Cast i32 to string
pub fn cast_i32_to_string(values: &[i32]) -> Vec<String> {
    values.iter().map(|v| v.to_string()).collect()
}

/// Cast f64 to string
pub fn cast_f64_to_string(values: &[f64]) -> Vec<String> {
    values.iter().map(|v| v.to_string()).collect()
}
