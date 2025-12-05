//! Sliding window buffer for stream processing
//! Maintains a fixed-size window over streaming data

use alloc::vec::Vec;
use alloc::collections::VecDeque;

/// Sliding window buffer with fixed size
///
/// Efficiently maintains a window of recent data for pattern matching,
/// statistics calculation, or compression algorithms.
pub struct SlidingWindow {
    buffer: VecDeque<u8>,
    capacity: usize,
    total_processed: usize,
}

impl SlidingWindow {
    /// Creates a new sliding window with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            total_processed: 0,
        }
    }

    /// Returns the window capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns current window size
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Checks if window is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Checks if window is full
    pub fn is_full(&self) -> bool {
        self.buffer.len() >= self.capacity
    }

    /// Returns total bytes processed
    pub fn total_processed(&self) -> usize {
        self.total_processed
    }

    /// Pushes a byte into window
    pub fn push(&mut self, byte: u8) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(byte);
        self.total_processed += 1;
    }

    /// Pushes multiple bytes
    pub fn push_slice(&mut self, data: &[u8]) {
        for &byte in data {
            self.push(byte);
        }
    }

    /// Gets byte at index (0 = oldest)
    pub fn get(&self, index: usize) -> Option<u8> {
        self.buffer.get(index).copied()
    }

    /// Gets slice of window
    pub fn as_slice(&self) -> Vec<u8> {
        self.buffer.iter().copied().collect()
    }

    /// Clears the window
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.total_processed = 0;
    }

    /// Finds pattern in window
    pub fn find(&self, pattern: &[u8]) -> Option<usize> {
        if pattern.is_empty() || pattern.len() > self.buffer.len() {
            return None;
        }

        let window_slice = self.as_slice();
        for i in 0..=(window_slice.len() - pattern.len()) {
            if &window_slice[i..i + pattern.len()] == pattern {
                return Some(i);
            }
        }

        None
    }

    /// Counts occurrences of byte
    pub fn count(&self, byte: u8) -> usize {
        self.buffer.iter().filter(|&&b| b == byte).count()
    }

    /// Gets minimum value in window
    pub fn min(&self) -> Option<u8> {
        self.buffer.iter().copied().min()
    }

    /// Gets maximum value in window
    pub fn max(&self) -> Option<u8> {
        self.buffer.iter().copied().max()
    }

    /// Calculates average value
    pub fn average(&self) -> f32 {
        if self.buffer.is_empty() {
            return 0.0;
        }
        let sum: u32 = self.buffer.iter().map(|&b| b as u32).sum();
        sum as f32 / self.buffer.len() as f32
    }

    /// Calculates sum of all bytes
    pub fn sum(&self) -> u64 {
        self.buffer.iter().map(|&b| b as u64).sum()
    }
}

/// Rolling hash for sliding window
/// Useful for fast pattern matching in streams
pub struct RollingHash {
    window: SlidingWindow,
    hash: u64,
    base: u64,
    modulo: u64,
    power: u64,
}

impl RollingHash {
    const BASE: u64 = 256;
    const MODULO: u64 = 1_000_000_007;

    /// Creates rolling hash with window size
    pub fn new(window_size: usize) -> Self {
        let mut power = 1u64;
        for _ in 0..window_size.saturating_sub(1) {
            power = (power * Self::BASE) % Self::MODULO;
        }

        Self {
            window: SlidingWindow::new(window_size),
            hash: 0,
            base: Self::BASE,
            modulo: Self::MODULO,
            power,
        }
    }

    /// Pushes byte and updates hash
    pub fn push(&mut self, byte: u8) {
        if self.window.is_full() {
            // Remove oldest byte from hash
            if let Some(old_byte) = self.window.get(0) {
                self.hash = (self.hash + self.modulo
                    - (old_byte as u64 * self.power) % self.modulo) % self.modulo;
            }
        }

        self.window.push(byte);

        // Add new byte to hash
        self.hash = (self.hash * self.base + byte as u64) % self.modulo;
    }

    /// Gets current hash value
    pub fn hash(&self) -> u64 {
        self.hash
    }

    /// Gets window content
    pub fn window(&self) -> Vec<u8> {
        self.window.as_slice()
    }

    /// Checks if window is full
    pub fn is_full(&self) -> bool {
        self.window.is_full()
    }
}

/// Minimum/Maximum tracker with sliding window
/// Efficiently tracks min/max in O(1) amortized time
pub struct MinMaxTracker {
    window: SlidingWindow,
    min_deque: VecDeque<(u8, usize)>, // (value, position)
    max_deque: VecDeque<(u8, usize)>,
    position: usize,
}

impl MinMaxTracker {
    /// Creates new tracker with window size
    pub fn new(window_size: usize) -> Self {
        Self {
            window: SlidingWindow::new(window_size),
            min_deque: VecDeque::new(),
            max_deque: VecDeque::new(),
            position: 0,
        }
    }

    /// Pushes value and updates min/max
    pub fn push(&mut self, value: u8) {
        self.window.push(value);

        // Update min deque
        while let Some(&(v, _)) = self.min_deque.back() {
            if v >= value {
                self.min_deque.pop_back();
            } else {
                break;
            }
        }
        self.min_deque.push_back((value, self.position));

        // Update max deque
        while let Some(&(v, _)) = self.max_deque.back() {
            if v <= value {
                self.max_deque.pop_back();
            } else {
                break;
            }
        }
        self.max_deque.push_back((value, self.position));

        // Remove old values
        let oldest_valid = self.position.saturating_sub(self.window.capacity() - 1);
        while let Some(&(_, pos)) = self.min_deque.front() {
            if pos < oldest_valid {
                self.min_deque.pop_front();
            } else {
                break;
            }
        }
        while let Some(&(_, pos)) = self.max_deque.front() {
            if pos < oldest_valid {
                self.max_deque.pop_front();
            } else {
                break;
            }
        }

        self.position += 1;
    }

    /// Gets current minimum (O(1))
    pub fn min(&self) -> Option<u8> {
        self.min_deque.front().map(|&(v, _)| v)
    }

    /// Gets current maximum (O(1))
    pub fn max(&self) -> Option<u8> {
        self.max_deque.front().map(|&(v, _)| v)
    }

    /// Gets window size
    pub fn len(&self) -> usize {
        self.window.len()
    }

    /// Checks if empty
    pub fn is_empty(&self) -> bool {
        self.window.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliding_window_basic() {
        let mut window = SlidingWindow::new(3);

        window.push(1);
        window.push(2);
        window.push(3);

        assert_eq!(window.len(), 3);
        assert!(window.is_full());

        window.push(4); // Should remove 1
        assert_eq!(window.as_slice(), vec![2, 3, 4]);
    }

    #[test]
    fn test_sliding_window_push_slice() {
        let mut window = SlidingWindow::new(5);
        window.push_slice(b"ABCDEFG");

        assert_eq!(window.as_slice(), b"CDEFG");
    }

    #[test]
    fn test_find_pattern() {
        let mut window = SlidingWindow::new(10);
        window.push_slice(b"ABCABC");

        assert_eq!(window.find(b"ABC"), Some(0));
        assert_eq!(window.find(b"CA"), Some(2));
        assert_eq!(window.find(b"XYZ"), None);
    }

    #[test]
    fn test_statistics() {
        let mut window = SlidingWindow::new(5);
        window.push_slice(&[1, 2, 3, 4, 5]);

        assert_eq!(window.min(), Some(1));
        assert_eq!(window.max(), Some(5));
        assert_eq!(window.average(), 3.0);
        assert_eq!(window.sum(), 15);
    }

    #[test]
    fn test_rolling_hash() {
        let mut hash = RollingHash::new(3);

        hash.push(b'A');
        hash.push(b'B');
        hash.push(b'C');

        let hash1 = hash.hash();

        hash.push(b'D'); // Window now "BCD"
        let hash2 = hash.hash();

        assert_ne!(hash1, hash2);
        assert!(hash.is_full());
    }

    #[test]
    fn test_rolling_hash_pattern() {
        let pattern = b"ABC";
        let text = b"XYZABCDEF";

        let mut pattern_hash = RollingHash::new(3);
        for &byte in pattern {
            pattern_hash.push(byte);
        }
        let target = pattern_hash.hash();

        let mut text_hash = RollingHash::new(3);
        let mut found_at = None;

        for (i, &byte) in text.iter().enumerate() {
            text_hash.push(byte);
            if text_hash.is_full() && text_hash.hash() == target {
                found_at = Some(i - 2);
                break;
            }
        }

        assert_eq!(found_at, Some(3));
    }

    #[test]
    fn test_min_max_tracker() {
        let mut tracker = MinMaxTracker::new(3);

        tracker.push(5);
        tracker.push(3);
        tracker.push(7);

        assert_eq!(tracker.min(), Some(3));
        assert_eq!(tracker.max(), Some(7));

        tracker.push(2); // Window: [3, 7, 2]
        assert_eq!(tracker.min(), Some(2));
        assert_eq!(tracker.max(), Some(7));

        tracker.push(1); // Window: [7, 2, 1]
        assert_eq!(tracker.min(), Some(1));
        assert_eq!(tracker.max(), Some(7));

        tracker.push(8); // Window: [2, 1, 8]
        assert_eq!(tracker.min(), Some(1));
        assert_eq!(tracker.max(), Some(8));
    }

    #[test]
    fn test_count() {
        let mut window = SlidingWindow::new(6);
        window.push_slice(b"AABBAA");

        assert_eq!(window.count(b'A'), 4);
        assert_eq!(window.count(b'B'), 2);
    }
}
