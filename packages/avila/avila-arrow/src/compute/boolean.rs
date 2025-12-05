//! Boolean kernels: and, or, not

use crate::error::Result;

/// Boolean AND
pub fn and(left: &[bool], right: &[bool]) -> Result<Vec<bool>> {
    if left.len() != right.len() {
        return Err(crate::error::ArrowError::ComputeError(
            "Arrays must have the same length".to_string()
        ));
    }
    Ok(left.iter().zip(right.iter()).map(|(l, r)| *l && *r).collect())
}

/// Boolean OR
pub fn or(left: &[bool], right: &[bool]) -> Result<Vec<bool>> {
    if left.len() != right.len() {
        return Err(crate::error::ArrowError::ComputeError(
            "Arrays must have the same length".to_string()
        ));
    }
    Ok(left.iter().zip(right.iter()).map(|(l, r)| *l || *r).collect())
}

/// Boolean NOT
pub fn not(values: &[bool]) -> Vec<bool> {
    values.iter().map(|v| !v).collect()
}
