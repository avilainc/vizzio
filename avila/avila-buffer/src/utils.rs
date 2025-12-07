//! Advanced buffer utilities and helpers
//! Pure Rust implementations

use alloc::vec::Vec;
use core::cmp::Ordering;

/// Bit manipulation utilities
pub struct BitOps;

impl BitOps {
    /// Counts set bits (population count)
    pub fn popcount(n: u64) -> u32 {
        n.count_ones()
    }

    /// Counts trailing zeros
    pub fn trailing_zeros(n: u64) -> u32 {
        n.trailing_zeros()
    }

    /// Counts leading zeros
    pub fn leading_zeros(n: u64) -> u32 {
        n.leading_zeros()
    }

    /// Reverses bits
    pub fn reverse_bits(n: u8) -> u8 {
        n.reverse_bits()
    }

    /// Checks if power of two
    pub fn is_power_of_two(n: usize) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }

    /// Next power of two
    pub fn next_power_of_two(n: usize) -> usize {
        n.checked_next_power_of_two().unwrap_or(0)
    }

    /// Rotates bits left
    pub fn rotate_left(byte: u8, n: u32) -> u8 {
        byte.rotate_left(n)
    }

    /// Rotates bits right
    pub fn rotate_right(byte: u8, n: u32) -> u8 {
        byte.rotate_right(n)
    }

    /// Swaps adjacent bits
    pub fn swap_bits(byte: u8) -> u8 {
        ((byte & 0xAA) >> 1) | ((byte & 0x55) << 1)
    }
}

/// Byte array comparison utilities
pub struct ByteCompare;

impl ByteCompare {
    /// Constant-time equality check (timing-safe)
    pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut result = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }

        result == 0
    }

    /// Lexicographic comparison
    pub fn compare(a: &[u8], b: &[u8]) -> Ordering {
        a.cmp(b)
    }

    /// Case-insensitive ASCII comparison
    pub fn compare_ignore_case(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        a.iter().zip(b.iter()).all(|(x, y)| {
            x.to_ascii_lowercase() == y.to_ascii_lowercase()
        })
    }

    /// Hamming distance between two byte arrays
    pub fn hamming_distance(a: &[u8], b: &[u8]) -> Option<usize> {
        if a.len() != b.len() {
            return None;
        }

        let mut distance = 0;
        for (x, y) in a.iter().zip(b.iter()) {
            distance += (x ^ y).count_ones() as usize;
        }

        Some(distance)
    }
}

/// Byte array transformation utilities
pub struct ByteTransform;

impl ByteTransform {
    /// Reverses byte array
    pub fn reverse(data: &mut [u8]) {
        data.reverse();
    }

    /// Rotates array left
    pub fn rotate_left(data: &mut [u8], n: usize) {
        data.rotate_left(n % data.len());
    }

    /// Rotates array right
    pub fn rotate_right(data: &mut [u8], n: usize) {
        data.rotate_right(n % data.len());
    }

    /// Converts to uppercase (ASCII)
    pub fn to_uppercase(data: &mut [u8]) {
        for byte in data {
            byte.make_ascii_uppercase();
        }
    }

    /// Converts to lowercase (ASCII)
    pub fn to_lowercase(data: &mut [u8]) {
        for byte in data {
            byte.make_ascii_lowercase();
        }
    }

    /// Replaces all occurrences
    pub fn replace(data: &mut [u8], from: u8, to: u8) -> usize {
        let mut count = 0;
        for byte in data {
            if *byte == from {
                *byte = to;
                count += 1;
            }
        }
        count
    }

    /// Removes duplicates (in-place, unstable order)
    pub fn dedup(data: &mut Vec<u8>) {
        data.sort_unstable();
        data.dedup();
    }

    /// Interleaves two arrays
    pub fn interleave(a: &[u8], b: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(a.len() + b.len());
        let mut iter_a = a.iter();
        let mut iter_b = b.iter();

        loop {
            match (iter_a.next(), iter_b.next()) {
                (Some(&x), Some(&y)) => {
                    result.push(x);
                    result.push(y);
                }
                (Some(&x), None) => result.push(x),
                (None, Some(&y)) => result.push(y),
                (None, None) => break,
            }
        }

        result
    }
}

/// Statistics calculator for byte arrays
pub struct ByteStats;

impl ByteStats {
    /// Calculates mean
    pub fn mean(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        let sum: u64 = data.iter().map(|&b| b as u64).sum();
        sum as f64 / data.len() as f64
    }

    /// Calculates median
    pub fn median(data: &[u8]) -> Option<f64> {
        if data.is_empty() {
            return None;
        }

        let mut sorted = data.to_vec();
        sorted.sort_unstable();

        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            Some((sorted[mid - 1] as f64 + sorted[mid] as f64) / 2.0)
        } else {
            Some(sorted[mid] as f64)
        }
    }

    /// Calculates mode (most frequent value)
    pub fn mode(data: &[u8]) -> Option<u8> {
        if data.is_empty() {
            return None;
        }

        let mut counts = [0usize; 256];
        for &byte in data {
            counts[byte as usize] += 1;
        }

        let (max_idx, _) = counts
            .iter()
            .enumerate()
            .max_by_key(|(_, &count)| count)?;

        Some(max_idx as u8)
    }

    /// Calculates variance
    pub fn variance(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mean = Self::mean(data);
        let sum_sq: f64 = data
            .iter()
            .map(|&b| {
                let diff = b as f64 - mean;
                diff * diff
            })
            .sum();

        sum_sq / data.len() as f64
    }

    /// Calculates standard deviation
    pub fn std_deviation(data: &[u8]) -> f64 {
        Self::variance(data).sqrt()
    }

    /// Calculates entropy (Shannon)
    pub fn entropy(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mut counts = [0usize; 256];
        for &byte in data {
            counts[byte as usize] += 1;
        }

        let len = data.len() as f64;
        let mut entropy = 0.0;

        for count in counts.iter().filter(|&&c| c > 0) {
            let p = *count as f64 / len;
            entropy -= p * p.log2();
        }

        entropy
    }

    /// Frequency distribution
    pub fn frequency_distribution(data: &[u8]) -> [usize; 256] {
        let mut counts = [0usize; 256];
        for &byte in data {
            counts[byte as usize] += 1;
        }
        counts
    }
}

/// Memory alignment utilities
pub struct Alignment;

impl Alignment {
    /// Checks if pointer is aligned
    pub fn is_aligned(ptr: *const u8, alignment: usize) -> bool {
        ptr as usize % alignment == 0
    }

    /// Aligns size up to alignment boundary
    pub fn align_up(size: usize, alignment: usize) -> usize {
        (size + alignment - 1) & !(alignment - 1)
    }

    /// Aligns size down to alignment boundary
    pub fn align_down(size: usize, alignment: usize) -> usize {
        size & !(alignment - 1)
    }

    /// Padding needed for alignment
    pub fn padding_needed(size: usize, alignment: usize) -> usize {
        Self::align_up(size, alignment) - size
    }
}

/// Endianness conversion utilities
pub struct Endian;

impl Endian {
    /// Swaps byte order of u16
    pub fn swap_u16(n: u16) -> u16 {
        n.swap_bytes()
    }

    /// Swaps byte order of u32
    pub fn swap_u32(n: u32) -> u32 {
        n.swap_bytes()
    }

    /// Swaps byte order of u64
    pub fn swap_u64(n: u64) -> u64 {
        n.swap_bytes()
    }

    /// Converts to big-endian
    pub fn to_be_u32(n: u32) -> u32 {
        n.to_be()
    }

    /// Converts to little-endian
    pub fn to_le_u32(n: u32) -> u32 {
        n.to_le()
    }

    /// Converts from big-endian
    pub fn from_be_u32(n: u32) -> u32 {
        u32::from_be(n)
    }

    /// Converts from little-endian
    pub fn from_le_u32(n: u32) -> u32 {
        u32::from_le(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_ops() {
        assert_eq!(BitOps::popcount(0b1011), 3);
        assert_eq!(BitOps::trailing_zeros(8), 3);
        assert_eq!(BitOps::reverse_bits(0b10101010), 0b01010101);
        assert!(BitOps::is_power_of_two(8));
        assert!(!BitOps::is_power_of_two(10));
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(ByteCompare::constant_time_eq(b"test", b"test"));
        assert!(!ByteCompare::constant_time_eq(b"test", b"best"));
    }

    #[test]
    fn test_hamming_distance() {
        let dist = ByteCompare::hamming_distance(b"test", b"best");
        assert_eq!(dist, Some(2)); // 't' vs 'b' differs in 2 bits
    }

    #[test]
    fn test_byte_transform() {
        let mut data = b"Hello".to_vec();
        ByteTransform::to_uppercase(&mut data);
        assert_eq!(&data, b"HELLO");

        ByteTransform::to_lowercase(&mut data);
        assert_eq!(&data, b"hello");
    }

    #[test]
    fn test_replace() {
        let mut data = b"Hello World".to_vec();
        let count = ByteTransform::replace(&mut data, b'o', b'0');
        assert_eq!(count, 2);
        assert_eq!(&data, b"Hell0 W0rld");
    }

    #[test]
    fn test_interleave() {
        let a = b"ABC";
        let b = b"123";
        let result = ByteTransform::interleave(a, b);
        assert_eq!(&result, b"A1B2C3");
    }

    #[test]
    fn test_stats() {
        let data = &[1, 2, 3, 4, 5];
        assert_eq!(ByteStats::mean(data), 3.0);
        assert_eq!(ByteStats::median(data), Some(3.0));
    }

    #[test]
    fn test_entropy() {
        // All same values = 0 entropy
        let uniform = &[1u8; 100];
        assert_eq!(ByteStats::entropy(uniform), 0.0);

        // Mixed values > 0 entropy
        let mixed = b"ABCDEFGH";
        assert!(ByteStats::entropy(mixed) > 0.0);
    }

    #[test]
    fn test_alignment() {
        assert_eq!(Alignment::align_up(10, 8), 16);
        assert_eq!(Alignment::align_down(10, 8), 8);
        assert_eq!(Alignment::padding_needed(10, 8), 6);
    }

    #[test]
    fn test_endian() {
        let n = 0x12345678u32;
        let swapped = Endian::swap_u32(n);
        assert_eq!(swapped, 0x78563412);

        let back = Endian::swap_u32(swapped);
        assert_eq!(back, n);
    }
}
