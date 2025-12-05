//! Fenwick Tree (Binary Indexed Tree)
//!
//! **Complexity**: O(log n) for both query and update
//! **Use cases**:
//! - Range sum queries
//! - Cumulative frequency
//! - Inversion count

use crate::DynamicArray;

/// Fenwick Tree for efficient range sum queries
///
/// **Performance**:
/// - Point update: O(log n)
/// - Range query: O(log n)
/// - Space: O(n)
///
/// **Advantages**:
/// - Simpler than segment tree
/// - Less memory overhead
/// - Faster constants
pub struct FenwickTree<T> {
    tree: DynamicArray<T>,
}

impl<T: Clone + Default + core::ops::AddAssign + core::ops::Sub<Output = T>> FenwickTree<T> {
    /// Create new Fenwick tree with n elements
    pub fn new(n: usize) -> Self {
        let mut tree = DynamicArray::with_capacity(n + 1);
        tree.resize(n + 1, T::default());

        Self { tree }
    }

    /// Create from initial array
    pub fn from_vec(arr: &[T]) -> Self {
        let mut ft = Self::new(arr.len());
        for (i, val) in arr.iter().enumerate() {
            ft.update(i, val.clone());
        }
        ft
    }

    /// Update value at index (0-indexed)
    pub fn update(&mut self, mut idx: usize, delta: T) {
        idx += 1; // Fenwick tree is 1-indexed internally

        while idx < self.tree.len() {
            self.tree[idx] += delta.clone();
            idx += Self::lowbit(idx);
        }
    }

    /// Get prefix sum [0, idx] (0-indexed)
    pub fn prefix_sum(&self, mut idx: usize) -> T {
        idx += 1; // Convert to 1-indexed
        let mut sum = T::default();

        while idx > 0 {
            sum += self.tree[idx].clone();
            idx -= Self::lowbit(idx);
        }

        sum
    }

    /// Get range sum [left, right] (0-indexed, inclusive)
    pub fn range_sum(&self, left: usize, right: usize) -> T {
        if left > 0 {
            self.prefix_sum(right) - self.prefix_sum(left - 1)
        } else {
            self.prefix_sum(right)
        }
    }

    /// Get size
    pub fn len(&self) -> usize {
        if self.tree.is_empty() {
            0
        } else {
            self.tree.len() - 1
        }
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get lowest bit (x & -x in two's complement)
    #[inline]
    fn lowbit(x: usize) -> usize {
        let x = x as isize;
        (x & -x) as usize
    }
}

// Implement for common numeric types
impl FenwickTree<i32> {
    /// Create from slice
    pub fn from_slice(arr: &[i32]) -> Self {
        Self::from_vec(arr)
    }
}

impl FenwickTree<i64> {
    /// Create from slice
    pub fn from_slice(arr: &[i64]) -> Self {
        Self::from_vec(arr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fenwick_basic() {
        let arr = vec![1, 2, 3, 4, 5];
        let ft = FenwickTree::from_slice(&arr);

        assert_eq!(ft.prefix_sum(0), 1);
        assert_eq!(ft.prefix_sum(2), 6);  // 1+2+3
        assert_eq!(ft.prefix_sum(4), 15); // 1+2+3+4+5
    }

    #[test]
    fn test_fenwick_range_sum() {
        let arr = vec![1, 2, 3, 4, 5];
        let ft = FenwickTree::from_slice(&arr);

        assert_eq!(ft.range_sum(1, 3), 9);  // 2+3+4
        assert_eq!(ft.range_sum(0, 4), 15); // 1+2+3+4+5
        assert_eq!(ft.range_sum(2, 2), 3);  // 3
    }

    #[test]
    fn test_fenwick_update() {
        let arr = vec![1, 2, 3, 4, 5];
        let mut ft = FenwickTree::from_slice(&arr);

        assert_eq!(ft.prefix_sum(4), 15);

        ft.update(2, 10); // Add 10 to index 2
        assert_eq!(ft.prefix_sum(4), 25); // 1+2+13+4+5

        ft.update(0, -1); // Subtract 1 from index 0
        assert_eq!(ft.prefix_sum(4), 24); // 0+2+13+4+5
    }
}
