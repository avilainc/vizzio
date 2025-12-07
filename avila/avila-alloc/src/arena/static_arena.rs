//! no_std compatible static arena allocator

use core::{mem::MaybeUninit, ptr::NonNull, alloc::Layout};

/// Static arena allocator (no_std compatible)
pub struct StaticArena<const N: usize> {
    data: [MaybeUninit<u8>; N],
    offset: usize,
}

impl<const N: usize> StaticArena<N> {
    /// Creates a new static arena
    pub const fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            offset: 0,
        }
    }

    /// Allocates space for a value of type T
    pub fn alloc<T>(&mut self) -> Option<&mut T> {
        let layout = Layout::new::<T>();
        self.alloc_layout(layout).map(|ptr| unsafe { &mut *(ptr.as_ptr() as *mut T) })
    }

    /// Allocates space for a slice of type T
    pub fn alloc_slice<T>(&mut self, len: usize) -> Option<&mut [T]> {
        let layout = Layout::array::<T>(len).ok()?;
        self.alloc_layout(layout).map(|ptr| unsafe {
            core::slice::from_raw_parts_mut(ptr.as_ptr() as *mut T, len)
        })
    }

    fn alloc_layout(&mut self, layout: Layout) -> Option<NonNull<u8>> {
        let align = layout.align();
        let size = layout.size();

        let offset = (self.offset + align - 1) & !(align - 1);
        if offset + size > N {
            return None;
        }

        let ptr = unsafe { NonNull::new_unchecked(self.data.as_mut_ptr().add(offset) as *mut u8) };
        self.offset = offset + size;
        Some(ptr)
    }

    /// Resets the arena
    pub fn reset(&mut self) {
        self.offset = 0;
    }

    /// Returns the number of bytes used
    pub const fn used(&self) -> usize {
        self.offset
    }

    /// Returns the total capacity
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns the remaining capacity
    pub const fn remaining(&self) -> usize {
        N - self.offset
    }

    /// Returns the utilization percentage (0.0 to 1.0)
    pub fn utilization(&self) -> f32 {
        self.offset as f32 / N as f32
    }

    /// Allocates and initializes a value
    pub fn alloc_with<T, F>(&mut self, f: F) -> Option<&mut T>
    where
        F: FnOnce() -> T,
    {
        let ptr = self.alloc::<T>()?;
        *ptr = f();
        Some(ptr)
    }

    /// Tries to allocate without returning a reference
    pub fn alloc_bytes(&mut self, size: usize, align: usize) -> Option<NonNull<u8>> {
        let layout = Layout::from_size_align(size, align).ok()?;
        self.alloc_layout(layout)
    }

    /// Allocates a zero-initialized slice
    pub fn alloc_zeroed<T>(&mut self, len: usize) -> Option<&mut [T]>
    where
        T: Copy,
    {
        let slice = self.alloc_slice::<T>(len)?;
        for item in slice.iter_mut() {
            unsafe {
                core::ptr::write_bytes(item as *mut T, 0, 1);
            }
        }
        Some(slice)
    }

    /// Creates a scoped arena that resets on drop
    pub fn scope<'a>(&'a mut self) -> ArenaScope<'a, N> {
        let checkpoint = self.offset;
        ArenaScope {
            arena: self,
            checkpoint,
        }
    }

    /// Returns the current offset (useful for checkpoints)
    pub const fn checkpoint(&self) -> usize {
        self.offset
    }

    /// Resets to a specific checkpoint
    pub fn reset_to(&mut self, checkpoint: usize) {
        if checkpoint <= self.offset {
            self.offset = checkpoint;
        }
    }
}

/// Scoped arena that automatically resets on drop
pub struct ArenaScope<'a, const N: usize> {
    arena: &'a mut StaticArena<N>,
    checkpoint: usize,
}

impl<'a, const N: usize> ArenaScope<'a, N> {
    /// Allocates space for a value
    pub fn alloc<T>(&mut self) -> Option<&mut T> {
        self.arena.alloc()
    }

    /// Allocates space for a slice
    pub fn alloc_slice<T>(&mut self, len: usize) -> Option<&mut [T]> {
        self.arena.alloc_slice(len)
    }

    /// Allocates and initializes
    pub fn alloc_with<T, F>(&mut self, f: F) -> Option<&mut T>
    where
        F: FnOnce() -> T,
    {
        self.arena.alloc_with(f)
    }
}

impl<'a, const N: usize> Drop for ArenaScope<'a, N> {
    fn drop(&mut self) {
        self.arena.reset_to(self.checkpoint);
    }
}

impl<const N: usize> Default for StaticArena<N> {
    fn default() -> Self {
        Self::new()
    }
}
