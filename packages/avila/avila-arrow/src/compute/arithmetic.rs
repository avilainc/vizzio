//! Arithmetic kernels: add, sub, mul, div

use crate::error::Result;

/// Add two arrays
pub fn add_i32(left: &[i32], right: &[i32]) -> Result<Vec<i32>> {
    if left.len() != right.len() {
        return Err(crate::error::ArrowError::ComputeError(
            "Arrays must have the same length".to_string()
        ));
    }
    Ok(left.iter().zip(right.iter()).map(|(l, r)| l + r).collect())
}

pub fn add_f64(left: &[f64], right: &[f64]) -> Result<Vec<f64>> {
    if left.len() != right.len() {
        return Err(crate::error::ArrowError::ComputeError(
            "Arrays must have the same length".to_string()
        ));
    }
    Ok(left.iter().zip(right.iter()).map(|(l, r)| l + r).collect())
}

/// Subtract two arrays
pub fn sub_i32(left: &[i32], right: &[i32]) -> Result<Vec<i32>> {
    if left.len() != right.len() {
        return Err(crate::error::ArrowError::ComputeError(
            "Arrays must have the same length".to_string()
        ));
    }
    Ok(left.iter().zip(right.iter()).map(|(l, r)| l - r).collect())
}

/// Multiply two arrays
pub fn mul_i32(left: &[i32], right: &[i32]) -> Result<Vec<i32>> {
    if left.len() != right.len() {
        return Err(crate::error::ArrowError::ComputeError(
            "Arrays must have the same length".to_string()
        ));
    }
    Ok(left.iter().zip(right.iter()).map(|(l, r)| l * r).collect())
}

/// Divide two arrays
pub fn div_f64(left: &[f64], right: &[f64]) -> Result<Vec<f64>> {
    if left.len() != right.len() {
        return Err(crate::error::ArrowError::ComputeError(
            "Arrays must have the same length".to_string()
        ));
    }
    Ok(left.iter().zip(right.iter()).map(|(l, r)| l / r).collect())
}
