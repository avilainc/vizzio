//! Comparison kernels: eq, lt, gt, etc.

use crate::error::Result;

/// Equal comparison
pub fn eq_i32(left: &[i32], right: &[i32]) -> Result<Vec<bool>> {
    if left.len() != right.len() {
        return Err(crate::error::ArrowError::ComputeError(
            "Arrays must have the same length".to_string()
        ));
    }
    Ok(left.iter().zip(right.iter()).map(|(l, r)| l == r).collect())
}

/// Less than comparison
pub fn lt_i32(left: &[i32], right: &[i32]) -> Result<Vec<bool>> {
    if left.len() != right.len() {
        return Err(crate::error::ArrowError::ComputeError(
            "Arrays must have the same length".to_string()
        ));
    }
    Ok(left.iter().zip(right.iter()).map(|(l, r)| l < r).collect())
}

/// Greater than comparison
pub fn gt_i32(left: &[i32], right: &[i32]) -> Result<Vec<bool>> {
    if left.len() != right.len() {
        return Err(crate::error::ArrowError::ComputeError(
            "Arrays must have the same length".to_string()
        ));
    }
    Ok(left.iter().zip(right.iter()).map(|(l, r)| l > r).collect())
}

/// Less than or equal comparison
pub fn lte_i32(left: &[i32], right: &[i32]) -> Result<Vec<bool>> {
    if left.len() != right.len() {
        return Err(crate::error::ArrowError::ComputeError(
            "Arrays must have the same length".to_string()
        ));
    }
    Ok(left.iter().zip(right.iter()).map(|(l, r)| l <= r).collect())
}

/// Greater than or equal comparison
pub fn gte_i32(left: &[i32], right: &[i32]) -> Result<Vec<bool>> {
    if left.len() != right.len() {
        return Err(crate::error::ArrowError::ComputeError(
            "Arrays must have the same length".to_string()
        ));
    }
    Ok(left.iter().zip(right.iter()).map(|(l, r)| l >= r).collect())
}
