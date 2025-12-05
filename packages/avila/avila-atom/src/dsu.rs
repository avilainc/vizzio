//! Disjoint Set Union (Union-Find) data structure
//!
//! **Complexity**: O(α(n)) amortized where α is inverse Ackermann function
//! **Use cases**:
//! - Graph connectivity
//! - Kruskal's MST algorithm
//! - Image segmentation

use crate::DynamicArray;

/// Disjoint Set Union with path compression and union by rank
///
/// **Performance**:
/// - Find: O(α(n)) amortized (nearly O(1))
/// - Union: O(α(n)) amortized
///
/// **Optimizations**:
/// - Path compression: Makes trees flat
/// - Union by rank: Keeps trees balanced
pub struct DisjointSet {
    parent: DynamicArray<usize>,
    rank: DynamicArray<usize>,
    size: DynamicArray<usize>,
    count: usize, // Number of disjoint sets
}

impl DisjointSet {
    /// Create new disjoint set with n elements
    pub fn new(n: usize) -> Self {
        let mut parent = DynamicArray::with_capacity(n);
        let mut rank = DynamicArray::with_capacity(n);
        let mut size = DynamicArray::with_capacity(n);

        for i in 0..n {
            parent.push(i);
            rank.push(0);
            size.push(1);
        }

        Self {
            parent,
            rank,
            size,
            count: n,
        }
    }

    /// Find representative of set containing x (with path compression)
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    /// Union sets containing x and y (by rank)
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already in same set
        }

        // Union by rank
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            core::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            }
            core::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
            core::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
                self.rank[root_x] += 1;
            }
        }

        self.count -= 1;
        true
    }

    /// Check if x and y are in the same set
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Get size of set containing x
    pub fn set_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }

    /// Get number of disjoint sets
    pub fn count(&self) -> usize {
        self.count
    }

    /// Get total number of elements
    pub fn len(&self) -> usize {
        self.parent.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dsu_basic() {
        let mut dsu = DisjointSet::new(5);

        assert_eq!(dsu.count(), 5);
        assert!(!dsu.connected(0, 1));

        dsu.union(0, 1);
        assert!(dsu.connected(0, 1));
        assert_eq!(dsu.count(), 4);

        dsu.union(2, 3);
        assert!(dsu.connected(2, 3));
        assert_eq!(dsu.count(), 3);

        dsu.union(1, 3);
        assert!(dsu.connected(0, 2));
        assert_eq!(dsu.count(), 2);
    }

    #[test]
    fn test_dsu_set_size() {
        let mut dsu = DisjointSet::new(6);

        dsu.union(0, 1);
        dsu.union(1, 2);

        assert_eq!(dsu.set_size(0), 3);
        assert_eq!(dsu.set_size(1), 3);
        assert_eq!(dsu.set_size(2), 3);
        assert_eq!(dsu.set_size(3), 1);
    }
}
