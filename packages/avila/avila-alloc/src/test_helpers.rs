//! Common testing utilities for avila-alloc

#![allow(dead_code)]

use core::fmt::Debug;

/// Test helper for checking allocator behavior
pub struct AllocTester;

impl AllocTester {
    /// Tests basic allocation and deallocation
    pub fn test_alloc_dealloc<F, T>(mut alloc_fn: F, item: T)
    where
        F: FnMut(T) -> Result<(), T>,
        T: Clone + Debug,
    {
        // Should succeed with capacity
        assert!(alloc_fn(item.clone()).is_ok(), "allocation should succeed");
    }

    /// Tests capacity limits
    pub fn test_capacity<F, T>(mut alloc_fn: F, item: T, capacity: usize)
    where
        F: FnMut(T) -> Result<(), T>,
        T: Clone + Debug,
    {
        // Fill to capacity
        for i in 0..capacity {
            assert!(
                alloc_fn(item.clone()).is_ok(),
                "allocation {} should succeed",
                i
            );
        }

        // Next allocation should fail
        assert!(
            alloc_fn(item.clone()).is_err(),
            "allocation beyond capacity should fail"
        );
    }

    /// Tests that operations maintain invariants
    pub fn test_invariants<F>(check_fn: F)
    where
        F: Fn() -> bool,
    {
        assert!(check_fn(), "invariants violated");
    }
}

/// Drop counter for testing Drop implementations
#[derive(Debug, Clone)]
pub struct DropCounter {
    counter: *mut usize,
}

impl DropCounter {
    pub fn new(counter: &mut usize) -> Self {
        Self {
            counter: counter as *mut usize,
        }
    }
}

impl Drop for DropCounter {
    fn drop(&mut self) {
        unsafe {
            *self.counter += 1;
        }
    }
}

/// Test data generator
pub struct TestData;

impl TestData {
    /// Generates a sequence of test integers
    pub fn integers(count: usize) -> impl Iterator<Item = i32> {
        (0..count as i32)
    }

    /// Generates a sequence of test strings
    pub fn strings(count: usize) -> impl Iterator<Item = &'static str> {
        ["alpha", "beta", "gamma", "delta", "epsilon"]
            .iter()
            .copied()
            .cycle()
            .take(count)
    }

    /// Generates random-like data (deterministic)
    pub fn pseudo_random(seed: u32, count: usize) -> impl Iterator<Item = u32> {
        let mut state = seed;
        (0..count).map(move |_| {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            state
        })
    }
}

/// Memory pattern checker
pub struct MemPattern;

impl MemPattern {
    /// Fills memory with a pattern
    pub unsafe fn fill(ptr: *mut u8, len: usize, pattern: u8) {
        core::ptr::write_bytes(ptr, pattern, len);
    }

    /// Checks if memory matches a pattern
    pub unsafe fn check(ptr: *const u8, len: usize, pattern: u8) -> bool {
        for i in 0..len {
            if *ptr.add(i) != pattern {
                return false;
            }
        }
        true
    }

    /// Fills with alternating pattern
    pub unsafe fn fill_alternating(ptr: *mut u8, len: usize) {
        for i in 0..len {
            *ptr.add(i) = (i & 0xFF) as u8;
        }
    }
}

/// Benchmark helper
#[derive(Default)]
pub struct BenchStats {
    pub iterations: usize,
    pub total_ns: u64,
}

impl BenchStats {
    pub fn new() -> Self {
        Self {
            iterations: 0,
            total_ns: 0,
        }
    }

    pub fn avg_ns(&self) -> f64 {
        if self.iterations == 0 {
            0.0
        } else {
            self.total_ns as f64 / self.iterations as f64
        }
    }

    pub fn throughput(&self, bytes: usize) -> f64 {
        if self.total_ns == 0 {
            0.0
        } else {
            (bytes as f64 * self.iterations as f64) / (self.total_ns as f64 / 1_000_000_000.0)
        }
    }
}

/// Macro for testing panic scenarios
#[macro_export]
macro_rules! should_panic {
    ($code:expr) => {{
        let result = std::panic::catch_unwind(|| {
            $code
        });
        assert!(result.is_err(), "expected panic but code succeeded");
    }};
}

/// Macro for creating test allocators
#[macro_export]
macro_rules! test_allocator {
    ($allocator:expr, $capacity:expr) => {{
        use $crate::test_helpers::AllocTester;

        let mut alloc = $allocator;

        // Test basic operations
        AllocTester::test_capacity(
            |item| alloc.push(item),
            42,
            $capacity,
        );
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_counter() {
        let mut count = 0;
        {
            let _counter = DropCounter::new(&mut count);
        }
        assert_eq!(count, 1);
    }

    #[test]
    fn test_data_generators() {
        let ints: Vec<_> = TestData::integers(5).collect();
        assert_eq!(ints, vec![0, 1, 2, 3, 4]);

        let strings: Vec<_> = TestData::strings(3).collect();
        assert_eq!(strings, vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn test_mem_pattern() {
        let mut buffer = [0u8; 10];
        unsafe {
            MemPattern::fill(buffer.as_mut_ptr(), 10, 0xAA);
            assert!(MemPattern::check(buffer.as_ptr(), 10, 0xAA));
        }
    }
}
