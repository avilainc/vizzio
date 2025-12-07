//! Memory layout utilities

use core::alloc::Layout;
use core::mem;

/// Extended layout calculations
pub struct LayoutExt;

impl LayoutExt {
    /// Calculates layout for an array with padding
    pub const fn array_with_padding<T>(n: usize, padding: usize) -> Result<Layout, ()> {
        let elem_size = mem::size_of::<T>();
        let elem_align = mem::align_of::<T>();

        let padded_size = elem_size + padding;
        let total_size = match padded_size.checked_mul(n) {
            Some(size) => size,
            None => return Err(()),
        };

        match Layout::from_size_align(total_size, elem_align) {
            Ok(layout) => Ok(layout),
            Err(_) => Err(()),
        }
    }

    /// Calculates layout for a struct with tail array
    pub const fn struct_with_array<T, U>(array_len: usize) -> Result<Layout, ()> {
        let struct_size = mem::size_of::<T>();
        let struct_align = mem::align_of::<T>();
        let elem_size = mem::size_of::<U>();
        let elem_align = mem::align_of::<U>();

        let max_align = if struct_align > elem_align { struct_align } else { elem_align };

        let array_offset = super::align_up(struct_size, elem_align);
        let array_size = match elem_size.checked_mul(array_len) {
            Some(size) => size,
            None => return Err(()),
        };

        let total_size = match array_offset.checked_add(array_size) {
            Some(size) => size,
            None => return Err(()),
        };

        match Layout::from_size_align(total_size, max_align) {
            Ok(layout) => Ok(layout),
            Err(_) => Err(()),
        }
    }
}

/// Memory statistics tracker
#[derive(Debug, Clone, Copy, Default)]
pub struct MemStats {
    /// Total bytes allocated
    pub allocated: usize,
    /// Total bytes deallocated
    pub deallocated: usize,
    /// Current bytes in use
    pub in_use: usize,
    /// Peak memory usage
    pub peak: usize,
    /// Number of allocations
    pub alloc_count: usize,
    /// Number of deallocations
    pub dealloc_count: usize,
}

impl MemStats {
    /// Creates new empty statistics
    pub const fn new() -> Self {
        Self {
            allocated: 0,
            deallocated: 0,
            in_use: 0,
            peak: 0,
            alloc_count: 0,
            dealloc_count: 0,
        }
    }

    /// Records an allocation
    pub fn record_alloc(&mut self, size: usize) {
        self.allocated = self.allocated.saturating_add(size);
        self.in_use = self.in_use.saturating_add(size);
        self.alloc_count += 1;

        if self.in_use > self.peak {
            self.peak = self.in_use;
        }
    }

    /// Records a deallocation
    pub fn record_dealloc(&mut self, size: usize) {
        self.deallocated = self.deallocated.saturating_add(size);
        self.in_use = self.in_use.saturating_sub(size);
        self.dealloc_count += 1;
    }

    /// Resets all statistics
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Returns the fragmentation ratio (0.0 to 1.0)
    pub fn fragmentation(&self) -> f32 {
        if self.allocated == 0 {
            return 0.0;
        }
        let wasted = self.allocated - self.in_use;
        wasted as f32 / self.allocated as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_stats() {
        let mut stats = MemStats::new();

        stats.record_alloc(100);
        assert_eq!(stats.allocated, 100);
        assert_eq!(stats.in_use, 100);
        assert_eq!(stats.peak, 100);

        stats.record_alloc(50);
        assert_eq!(stats.allocated, 150);
        assert_eq!(stats.in_use, 150);
        assert_eq!(stats.peak, 150);

        stats.record_dealloc(50);
        assert_eq!(stats.in_use, 100);
        assert_eq!(stats.peak, 150);

        stats.reset();
        assert_eq!(stats.in_use, 0);
    }
}
