//! Filter operations for arrays

use crate::error::Result;

/// Filter an i32 array by a boolean mask
pub fn filter_i32(values: &[i32], mask: &[bool]) -> Result<Vec<i32>> {
    if values.len() != mask.len() {
        return Err(crate::error::ArrowError::ArrayLengthMismatch {
            expected: values.len(),
            actual: mask.len(),
        });
    }

    Ok(values
        .iter()
        .zip(mask.iter())
        .filter_map(|(&v, &m)| if m { Some(v) } else { None })
        .collect())
}

/// Filter an i64 array by a boolean mask
pub fn filter_i64(values: &[i64], mask: &[bool]) -> Result<Vec<i64>> {
    if values.len() != mask.len() {
        return Err(crate::error::ArrowError::ArrayLengthMismatch {
            expected: values.len(),
            actual: mask.len(),
        });
    }

    Ok(values
        .iter()
        .zip(mask.iter())
        .filter_map(|(&v, &m)| if m { Some(v) } else { None })
        .collect())
}

/// Filter an f32 array by a boolean mask
pub fn filter_f32(values: &[f32], mask: &[bool]) -> Result<Vec<f32>> {
    if values.len() != mask.len() {
        return Err(crate::error::ArrowError::ArrayLengthMismatch {
            expected: values.len(),
            actual: mask.len(),
        });
    }

    Ok(values
        .iter()
        .zip(mask.iter())
        .filter_map(|(&v, &m)| if m { Some(v) } else { None })
        .collect())
}

/// Filter an f64 array by a boolean mask
pub fn filter_f64(values: &[f64], mask: &[bool]) -> Result<Vec<f64>> {
    if values.len() != mask.len() {
        return Err(crate::error::ArrowError::ArrayLengthMismatch {
            expected: values.len(),
            actual: mask.len(),
        });
    }

    Ok(values
        .iter()
        .zip(mask.iter())
        .filter_map(|(&v, &m)| if m { Some(v) } else { None })
        .collect())
}

/// Filter a string array by a boolean mask
pub fn filter_string(values: &[String], mask: &[bool]) -> Result<Vec<String>> {
    if values.len() != mask.len() {
        return Err(crate::error::ArrowError::ArrayLengthMismatch {
            expected: values.len(),
            actual: mask.len(),
        });
    }

    Ok(values
        .iter()
        .zip(mask.iter())
        .filter_map(|(v, &m)| if m { Some(v.clone()) } else { None })
        .collect())
}

/// Take values at specified indices from an i32 array
pub fn take_i32(values: &[i32], indices: &[usize]) -> Result<Vec<i32>> {
    let mut result = Vec::with_capacity(indices.len());
    for &index in indices {
        if index >= values.len() {
            return Err(crate::error::ArrowError::OutOfBounds {
                index,
                length: values.len(),
            });
        }
        result.push(values[index]);
    }
    Ok(result)
}

/// Take values at specified indices from an f64 array
pub fn take_f64(values: &[f64], indices: &[usize]) -> Result<Vec<f64>> {
    let mut result = Vec::with_capacity(indices.len());
    for &index in indices {
        if index >= values.len() {
            return Err(crate::error::ArrowError::OutOfBounds {
                index,
                length: values.len(),
            });
        }
        result.push(values[index]);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_i32() {
        let values = vec![1, 2, 3, 4, 5];
        let mask = vec![true, false, true, false, true];
        let result = filter_i32(&values, &mask).unwrap();
        assert_eq!(result, vec![1, 3, 5]);
    }

    #[test]
    fn test_take_i32() {
        let values = vec![10, 20, 30, 40, 50];
        let indices = vec![0, 2, 4];
        let result = take_i32(&values, &indices).unwrap();
        assert_eq!(result, vec![10, 30, 50]);
    }

    #[test]
    fn test_filter_f64() {
        let values = vec![1.1, 2.2, 3.3, 4.4];
        let mask = vec![true, true, false, false];
        let result = filter_f64(&values, &mask).unwrap();
        assert_eq!(result, vec![1.1, 2.2]);
    }
}
