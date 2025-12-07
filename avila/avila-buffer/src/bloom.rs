//! Bloom filter for efficient membership testing

use alloc::vec::Vec;

pub struct BloomFilter {
    bits: Vec<u64>,
    bit_count: usize,
    hash_count: usize,
}

impl BloomFilter {
    pub fn new(capacity: usize, false_positive_rate: f64) -> Self {
        let bit_count = Self::optimal_bit_count(capacity, false_positive_rate);
        let hash_count = Self::optimal_hash_count(capacity, bit_count);
        let word_count = (bit_count + 63) / 64;

        Self {
            bits: vec![0u64; word_count],
            bit_count,
            hash_count,
        }
    }

    fn optimal_bit_count(n: usize, p: f64) -> usize {
        let n = n as f64;
        let ln2_squared = core::f64::consts::LN_2 * core::f64::consts::LN_2;
        (-(n * p.ln()) / ln2_squared).ceil() as usize
    }

    fn optimal_hash_count(n: usize, m: usize) -> usize {
        ((m as f64 / n as f64) * core::f64::consts::LN_2).ceil() as usize
    }

    fn hash(&self, data: &[u8], seed: u32) -> usize {
        let mut hash = seed as u64;
        for &byte in data {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        (hash % self.bit_count as u64) as usize
    }

    pub fn insert(&mut self, data: &[u8]) {
        for i in 0..self.hash_count {
            let bit_index = self.hash(data, i as u32);
            let word_index = bit_index / 64;
            let bit_offset = bit_index % 64;
            self.bits[word_index] |= 1u64 << bit_offset;
        }
    }

    pub fn contains(&self, data: &[u8]) -> bool {
        for i in 0..self.hash_count {
            let bit_index = self.hash(data, i as u32);
            let word_index = bit_index / 64;
            let bit_offset = bit_index % 64;
            if self.bits[word_index] & (1u64 << bit_offset) == 0 {
                return false;
            }
        }
        true
    }

    pub fn clear(&mut self) {
        for word in &mut self.bits {
            *word = 0;
        }
    }

    pub fn bit_count(&self) -> usize {
        self.bit_count
    }

    pub fn hash_count(&self) -> usize {
        self.hash_count
    }

    pub fn estimated_count(&self) -> usize {
        let set_bits = self.bits.iter()
            .map(|w| w.count_ones() as usize)
            .sum::<usize>();

        let m = self.bit_count as f64;
        let k = self.hash_count as f64;
        let x = set_bits as f64;

        (-(m / k) * ((m - x) / m).ln()) as usize
    }
}

pub struct CountingBloomFilter {
    counters: Vec<u8>,
    bit_count: usize,
    hash_count: usize,
}

impl CountingBloomFilter {
    pub fn new(capacity: usize, false_positive_rate: f64) -> Self {
        let bit_count = BloomFilter::optimal_bit_count(capacity, false_positive_rate);
        let hash_count = BloomFilter::optimal_hash_count(capacity, bit_count);

        Self {
            counters: vec![0u8; bit_count],
            bit_count,
            hash_count,
        }
    }

    fn hash(&self, data: &[u8], seed: u32) -> usize {
        let mut hash = seed as u64;
        for &byte in data {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        (hash % self.bit_count as u64) as usize
    }

    pub fn insert(&mut self, data: &[u8]) {
        for i in 0..self.hash_count {
            let index = self.hash(data, i as u32);
            if self.counters[index] < 255 {
                self.counters[index] += 1;
            }
        }
    }

    pub fn remove(&mut self, data: &[u8]) {
        for i in 0..self.hash_count {
            let index = self.hash(data, i as u32);
            if self.counters[index] > 0 {
                self.counters[index] -= 1;
            }
        }
    }

    pub fn contains(&self, data: &[u8]) -> bool {
        for i in 0..self.hash_count {
            let index = self.hash(data, i as u32);
            if self.counters[index] == 0 {
                return false;
            }
        }
        true
    }

    pub fn clear(&mut self) {
        for counter in &mut self.counters {
            *counter = 0;
        }
    }
}

pub struct ScalableBloomFilter {
    filters: Vec<BloomFilter>,
    capacity: usize,
    false_positive_rate: f64,
    growth_factor: usize,
}

impl ScalableBloomFilter {
    pub fn new(initial_capacity: usize, false_positive_rate: f64) -> Self {
        let mut filters = Vec::new();
        filters.push(BloomFilter::new(initial_capacity, false_positive_rate));

        Self {
            filters,
            capacity: initial_capacity,
            false_positive_rate,
            growth_factor: 2,
        }
    }

    pub fn insert(&mut self, data: &[u8]) {
        if !self.filters.last().unwrap().contains(data) {
            self.filters.last_mut().unwrap().insert(data);
        }
    }

    pub fn contains(&self, data: &[u8]) -> bool {
        self.filters.iter().any(|f| f.contains(data))
    }

    pub fn add_filter(&mut self) {
        let new_capacity = self.capacity * self.growth_factor;
        self.filters.push(BloomFilter::new(new_capacity, self.false_positive_rate));
        self.capacity = new_capacity;
    }

    pub fn filter_count(&self) -> usize {
        self.filters.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_filter() {
        let mut filter = BloomFilter::new(1000, 0.01);

        filter.insert(b"hello");
        filter.insert(b"world");

        assert!(filter.contains(b"hello"));
        assert!(filter.contains(b"world"));
        assert!(!filter.contains(b"rust"));
    }

    #[test]
    fn test_bloom_clear() {
        let mut filter = BloomFilter::new(100, 0.01);

        filter.insert(b"test");
        assert!(filter.contains(b"test"));

        filter.clear();
        assert!(!filter.contains(b"test"));
    }

    #[test]
    fn test_counting_bloom() {
        let mut filter = CountingBloomFilter::new(100, 0.01);

        filter.insert(b"test");
        assert!(filter.contains(b"test"));

        filter.remove(b"test");
        assert!(!filter.contains(b"test"));
    }

    #[test]
    fn test_scalable_bloom() {
        let mut filter = ScalableBloomFilter::new(10, 0.01);

        for i in 0..20 {
            let data = format!("item{}", i);
            filter.insert(data.as_bytes());
        }

        assert!(filter.contains(b"item0"));
        assert!(filter.contains(b"item19"));
    }
}
