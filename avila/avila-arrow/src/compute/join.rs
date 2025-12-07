//! Join operations for combining arrays
//!
//! Pure Rust implementation of SQL-like join operations.

use crate::error::{Result, ArrowError};
use std::collections::HashMap;

/// Result of a join operation
pub struct JoinResult {
    pub left_indices: Vec<usize>,
    pub right_indices: Vec<usize>,
}

/// Inner join on i32 keys
pub fn inner_join_i32(left_keys: &[i32], right_keys: &[i32]) -> JoinResult {
    // Build hash map of right keys
    let mut right_map: HashMap<i32, Vec<usize>> = HashMap::new();
    for (idx, key) in right_keys.iter().enumerate() {
        right_map.entry(*key).or_insert_with(Vec::new).push(idx);
    }

    let mut left_indices = Vec::new();
    let mut right_indices = Vec::new();

    // Probe with left keys
    for (left_idx, key) in left_keys.iter().enumerate() {
        if let Some(right_idxs) = right_map.get(key) {
            for &right_idx in right_idxs {
                left_indices.push(left_idx);
                right_indices.push(right_idx);
            }
        }
    }

    JoinResult {
        left_indices,
        right_indices,
    }
}

/// Left join on i32 keys
pub fn left_join_i32(left_keys: &[i32], right_keys: &[i32]) -> JoinResult {
    // Build hash map of right keys
    let mut right_map: HashMap<i32, Vec<usize>> = HashMap::new();
    for (idx, key) in right_keys.iter().enumerate() {
        right_map.entry(*key).or_insert_with(Vec::new).push(idx);
    }

    let mut left_indices = Vec::new();
    let mut right_indices = Vec::new();

    // Probe with left keys
    for (left_idx, key) in left_keys.iter().enumerate() {
        if let Some(right_idxs) = right_map.get(key) {
            for &right_idx in right_idxs {
                left_indices.push(left_idx);
                right_indices.push(right_idx);
            }
        } else {
            // No match in right, add null marker
            left_indices.push(left_idx);
            right_indices.push(usize::MAX); // Use MAX as null marker
        }
    }

    JoinResult {
        left_indices,
        right_indices,
    }
}

/// Right join on i32 keys
pub fn right_join_i32(left_keys: &[i32], right_keys: &[i32]) -> JoinResult {
    // Build hash map of left keys
    let mut left_map: HashMap<i32, Vec<usize>> = HashMap::new();
    for (idx, key) in left_keys.iter().enumerate() {
        left_map.entry(*key).or_insert_with(Vec::new).push(idx);
    }

    let mut left_indices = Vec::new();
    let mut right_indices = Vec::new();

    // Probe with right keys
    for (right_idx, key) in right_keys.iter().enumerate() {
        if let Some(left_idxs) = left_map.get(key) {
            for &left_idx in left_idxs {
                left_indices.push(left_idx);
                right_indices.push(right_idx);
            }
        } else {
            // No match in left, add null marker
            left_indices.push(usize::MAX); // Use MAX as null marker
            right_indices.push(right_idx);
        }
    }

    JoinResult {
        left_indices,
        right_indices,
    }
}

/// Semi join - returns left rows that have a match in right
pub fn semi_join_i32(left_keys: &[i32], right_keys: &[i32]) -> Vec<usize> {
    let right_set: std::collections::HashSet<i32> = right_keys.iter().copied().collect();

    left_keys
        .iter()
        .enumerate()
        .filter_map(|(idx, key)| {
            if right_set.contains(key) {
                Some(idx)
            } else {
                None
            }
        })
        .collect()
}

/// Anti join - returns left rows that don't have a match in right
pub fn anti_join_i32(left_keys: &[i32], right_keys: &[i32]) -> Vec<usize> {
    let right_set: std::collections::HashSet<i32> = right_keys.iter().copied().collect();

    left_keys
        .iter()
        .enumerate()
        .filter_map(|(idx, key)| {
            if !right_set.contains(key) {
                Some(idx)
            } else {
                None
            }
        })
        .collect()
}

/// Join on string keys
pub fn inner_join_string(left_keys: &[String], right_keys: &[String]) -> JoinResult {
    let mut right_map: HashMap<&str, Vec<usize>> = HashMap::new();
    for (idx, key) in right_keys.iter().enumerate() {
        right_map.entry(key.as_str()).or_insert_with(Vec::new).push(idx);
    }

    let mut left_indices = Vec::new();
    let mut right_indices = Vec::new();

    for (left_idx, key) in left_keys.iter().enumerate() {
        if let Some(right_idxs) = right_map.get(key.as_str()) {
            for &right_idx in right_idxs {
                left_indices.push(left_idx);
                right_indices.push(right_idx);
            }
        }
    }

    JoinResult {
        left_indices,
        right_indices,
    }
}

/// Multi-column join on i32 keys
pub fn inner_join_multi_i32(
    left_keys: &[Vec<i32>],
    right_keys: &[Vec<i32>],
) -> Result<JoinResult> {
    if left_keys.is_empty() || right_keys.is_empty() {
        return Ok(JoinResult {
            left_indices: Vec::new(),
            right_indices: Vec::new(),
        });
    }

    // Build hash map of right key tuples
    let mut right_map: HashMap<Vec<i32>, Vec<usize>> = HashMap::new();
    for (idx, key) in right_keys.iter().enumerate() {
        right_map.entry(key.clone()).or_insert_with(Vec::new).push(idx);
    }

    let mut left_indices = Vec::new();
    let mut right_indices = Vec::new();

    // Probe with left keys
    for (left_idx, key) in left_keys.iter().enumerate() {
        if let Some(right_idxs) = right_map.get(key) {
            for &right_idx in right_idxs {
                left_indices.push(left_idx);
                right_indices.push(right_idx);
            }
        }
    }

    Ok(JoinResult {
        left_indices,
        right_indices,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inner_join() {
        let left = vec![1, 2, 3, 4];
        let right = vec![3, 4, 5, 6];

        let result = inner_join_i32(&left, &right);
        assert_eq!(result.left_indices, vec![2, 3]);
        assert_eq!(result.right_indices, vec![0, 1]);
    }

    #[test]
    fn test_inner_join_duplicates() {
        let left = vec![1, 2, 2, 3];
        let right = vec![2, 2, 4];

        let result = inner_join_i32(&left, &right);
        // Each left 2 should match with each right 2
        assert_eq!(result.left_indices, vec![1, 1, 2, 2]);
        assert_eq!(result.right_indices, vec![0, 1, 0, 1]);
    }

    #[test]
    fn test_left_join() {
        let left = vec![1, 2, 3];
        let right = vec![2, 4];

        let result = left_join_i32(&left, &right);
        assert_eq!(result.left_indices, vec![0, 1, 2]);
        assert_eq!(result.right_indices, vec![usize::MAX, 0, usize::MAX]);
    }

    #[test]
    fn test_right_join() {
        let left = vec![1, 2];
        let right = vec![2, 3, 4];

        let result = right_join_i32(&left, &right);
        assert_eq!(result.left_indices, vec![1, usize::MAX, usize::MAX]);
        assert_eq!(result.right_indices, vec![0, 1, 2]);
    }

    #[test]
    fn test_semi_join() {
        let left = vec![1, 2, 3, 4];
        let right = vec![2, 4, 6];

        let result = semi_join_i32(&left, &right);
        assert_eq!(result, vec![1, 3]); // Indices of 2 and 4 in left
    }

    #[test]
    fn test_anti_join() {
        let left = vec![1, 2, 3, 4];
        let right = vec![2, 4, 6];

        let result = anti_join_i32(&left, &right);
        assert_eq!(result, vec![0, 2]); // Indices of 1 and 3 in left
    }

    #[test]
    fn test_string_join() {
        let left = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let right = vec!["b".to_string(), "c".to_string(), "d".to_string()];

        let result = inner_join_string(&left, &right);
        assert_eq!(result.left_indices, vec![1, 2]);
        assert_eq!(result.right_indices, vec![0, 1]);
    }

    #[test]
    fn test_multi_column_join() {
        let left = vec![
            vec![1, 10],
            vec![2, 20],
            vec![3, 30],
        ];
        let right = vec![
            vec![2, 20],
            vec![3, 30],
            vec![4, 40],
        ];

        let result = inner_join_multi_i32(&left, &right).unwrap();
        assert_eq!(result.left_indices, vec![1, 2]);
        assert_eq!(result.right_indices, vec![0, 1]);
    }
}
