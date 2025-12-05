//! Sorting operations for arrays

use crate::error::Result;

/// Sort an i32 array in ascending order
pub fn sort_i32(values: &[i32]) -> Vec<i32> {
    let mut result = values.to_vec();
    result.sort_unstable();
    result
}

/// Sort an i32 array in descending order
pub fn sort_i32_desc(values: &[i32]) -> Vec<i32> {
    let mut result = values.to_vec();
    result.sort_unstable_by(|a, b| b.cmp(a));
    result
}

/// Sort an i64 array in ascending order
pub fn sort_i64(values: &[i64]) -> Vec<i64> {
    let mut result = values.to_vec();
    result.sort_unstable();
    result
}

/// Sort an i64 array in descending order
pub fn sort_i64_desc(values: &[i64]) -> Vec<i64> {
    let mut result = values.to_vec();
    result.sort_unstable_by(|a, b| b.cmp(a));
    result
}

/// Sort an f32 array in ascending order
pub fn sort_f32(values: &[f32]) -> Vec<f32> {
    let mut result = values.to_vec();
    result.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    result
}

/// Sort an f64 array in ascending order
pub fn sort_f64(values: &[f64]) -> Vec<f64> {
    let mut result = values.to_vec();
    result.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    result
}

/// Sort an f64 array in descending order
pub fn sort_f64_desc(values: &[f64]) -> Vec<f64> {
    let mut result = values.to_vec();
    result.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    result
}

/// Get the indices that would sort an i32 array
pub fn argsort_i32(values: &[i32]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..values.len()).collect();
    indices.sort_unstable_by_key(|&i| values[i]);
    indices
}

/// Get the indices that would sort an f64 array
pub fn argsort_f64(values: &[f64]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..values.len()).collect();
    indices.sort_by(|&a, &b| {
        values[a]
            .partial_cmp(&values[b])
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    indices
}

/// Partition an i32 array around a pivot (quicksort partition)
pub fn partition_i32(values: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = values[high];
    let mut i = low;

    for j in low..high {
        if values[j] <= pivot {
            values.swap(i, j);
            i += 1;
        }
    }
    values.swap(i, high);
    i
}

/// Find the k-th smallest element (quickselect algorithm)
pub fn quickselect_i32(values: &mut [i32], k: usize) -> i32 {
    let mut low = 0;
    let mut high = values.len() - 1;

    loop {
        if low == high {
            return values[low];
        }

        let pivot_index = partition_i32(values, low, high);

        if k == pivot_index {
            return values[k];
        } else if k < pivot_index {
            high = pivot_index - 1;
        } else {
            low = pivot_index + 1;
        }
    }
}

/// Find median of an i32 array
pub fn median_i32(values: &[i32]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let len = sorted.len();

    if len % 2 == 0 {
        (sorted[len / 2 - 1] as f64 + sorted[len / 2] as f64) / 2.0
    } else {
        sorted[len / 2] as f64
    }
}

/// Find median of an f64 array
pub fn median_f64(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let len = sorted.len();

    if len % 2 == 0 {
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
    } else {
        sorted[len / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_i32() {
        let values = vec![5, 2, 8, 1, 9];
        let sorted = sort_i32(&values);
        assert_eq!(sorted, vec![1, 2, 5, 8, 9]);
    }

    #[test]
    fn test_sort_i32_desc() {
        let values = vec![5, 2, 8, 1, 9];
        let sorted = sort_i32_desc(&values);
        assert_eq!(sorted, vec![9, 8, 5, 2, 1]);
    }

    #[test]
    fn test_argsort_i32() {
        let values = vec![5, 2, 8, 1, 9];
        let indices = argsort_i32(&values);
        assert_eq!(indices, vec![3, 1, 0, 2, 4]);
    }

    #[test]
    fn test_median_i32() {
        let values = vec![1, 2, 3, 4, 5];
        assert_eq!(median_i32(&values), 3.0);

        let values = vec![1, 2, 3, 4];
        assert_eq!(median_i32(&values), 2.5);
    }

    #[test]
    fn test_median_f64() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(median_f64(&values), 3.0);
    }
}
