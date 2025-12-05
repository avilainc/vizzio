//! Bloom Filter - Probabilistic set membership testing
//!
//! **Properties**:
//! - False positives possible
//! - False negatives impossible
//! - Space-efficient
//!
//! **Use cases**:
//! - Cache filtering
//! - Deduplication
//! - Database query optimization

use crate::DynamicArray;
use core::hash::{Hash, Hasher};

/// Bloom filter for probabilistic set membership
///
/// **Performance**:
/// - Insert: O(k) where k = number of hash functions
/// - Query: O(k)
/// - Space: O(m) where m = bit array size
///
/// **Trade-offs**:
/// - Lower false positive rate = more memory
/// - More hash functions = slower but more accurate
pub struct BloomFilter {
    bits: DynamicArray<u8>,
    bit_count: usize,
    hash_count: usize,
    item_count: usize,
}

impl BloomFilter {
    /// Create new bloom filter
    ///
    /// # Arguments
    /// * `capacity` - Expected number of elements
    /// * `false_positive_rate` - Desired false positive rate (e.g., 0.01 for 1%)
    pub fn new(capacity: usize, false_positive_rate: f64) -> Self {
        let bit_count = Self::optimal_bit_count(capacity, false_positive_rate);
        let hash_count = Self::optimal_hash_count(bit_count, capacity);
        let byte_count = (bit_count + 7) / 8;

        let mut bits = DynamicArray::with_capacity(byte_count);
        bits.resize(byte_count, 0);

        Self {
            bits,
            bit_count,
            hash_count,
            item_count: 0,
        }
    }

    /// Insert item into filter
    pub fn insert<T: Hash>(&mut self, item: &T) {
        for i in 0..self.hash_count {
            let hash = self.hash(item, i);
            let bit_index = (hash % self.bit_count as u64) as usize;
            self.set_bit(bit_index);
        }
        self.item_count += 1;
    }

    /// Check if item might be in set
    ///
    /// Returns:
    /// - `true`: Item might be in set (possible false positive)
    /// - `false`: Item definitely not in set (no false negatives)
    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        for i in 0..self.hash_count {
            let hash = self.hash(item, i);
            let bit_index = (hash % self.bit_count as u64) as usize;
            if !self.get_bit(bit_index) {
                return false;
            }
        }
        true
    }

    /// Clear all bits
    pub fn clear(&mut self) {
        for byte in &mut self.bits {
            *byte = 0;
        }
        self.item_count = 0;
    }

    /// Get number of items inserted
    pub fn len(&self) -> usize {
        self.item_count
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.item_count == 0
    }

    /// Get current false positive rate estimate
    pub fn false_positive_rate(&self) -> f64 {
        let k = self.hash_count as f64;
        let m = self.bit_count as f64;
        let n = self.item_count as f64;

        (1.0 - (-k * n / m).exp()).powf(k)
    }

    // Calculate optimal number of bits
    fn optimal_bit_count(capacity: usize, fpr: f64) -> usize {
        let n = capacity as f64;
        let ln2_squared = core::f64::consts::LN_2 * core::f64::consts::LN_2;
        (-(n * fpr.ln()) / ln2_squared).ceil() as usize
    }

    // Calculate optimal number of hash functions
    fn optimal_hash_count(bit_count: usize, capacity: usize) -> usize {
        let m = bit_count as f64;
        let n = capacity as f64;
        ((m / n) * core::f64::consts::LN_2).ceil() as usize
    }

    // Hash item with seed
    fn hash<T: Hash>(&self, item: &T, seed: usize) -> u64 {
        let mut hasher = FnvHasher::new(seed as u64);
        item.hash(&mut hasher);
        hasher.finish()
    }

    // Set bit at index
    fn set_bit(&mut self, index: usize) {
        let byte_index = index / 8;
        let bit_index = index % 8;
        self.bits[byte_index] |= 1 << bit_index;
    }

    // Get bit at index
    fn get_bit(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let bit_index = index % 8;
        (self.bits[byte_index] & (1 << bit_index)) != 0
    }
}

/// Simple FNV-1a hasher for bloom filter
struct FnvHasher {
    state: u64,
}

impl FnvHasher {
    fn new(seed: u64) -> Self {
        Self {
            state: 0xcbf29ce484222325_u64.wrapping_add(seed),
        }
    }
}

impl Hasher for FnvHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state ^= byte as u64;
            self.state = self.state.wrapping_mul(0x100000001b3);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_filter_basic() {
        let mut filter = BloomFilter::new(100, 0.01);

        filter.insert(&"hello");
        filter.insert(&"world");

        assert!(filter.contains(&"hello"));
        assert!(filter.contains(&"world"));
        assert!(!filter.contains(&"rust"));
    }

    #[test]
    fn test_bloom_filter_numbers() {
        let mut filter = BloomFilter::new(1000, 0.01);

        for i in 0..100 {
            filter.insert(&i);
        }

        for i in 0..100 {
            assert!(filter.contains(&i));
        }

        let mut false_positives = 0;
        for i in 100..200 {
            if filter.contains(&i) {
                false_positives += 1;
            }
        }

        // Should have very few false positives
        assert!(false_positives < 5);
    }

    #[test]
    fn test_bloom_filter_clear() {
        let mut filter = BloomFilter::new(100, 0.01);

        filter.insert(&"test");
        assert!(filter.contains(&"test"));

        filter.clear();
        assert!(!filter.contains(&"test"));
        assert_eq!(filter.len(), 0);
    }
}
