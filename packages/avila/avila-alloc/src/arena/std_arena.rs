//! Standard library Arena allocator

use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

/// Bump allocator for temporary allocations (requires std)
pub struct Arena {
    ptr: NonNull<u8>,
    capacity: usize,
    offset: usize,
}

impl Arena {
    /// Creates a new arena with the given capacity
    pub fn new(capacity: usize) -> Self {
        let layout = Layout::from_size_align(capacity, 8).unwrap();
        let ptr = unsafe { alloc(layout) };
        let ptr = NonNull::new(ptr).expect("allocation failed");

        Self {
            ptr,
            capacity,
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
            std::slice::from_raw_parts_mut(ptr.as_ptr() as *mut T, len)
        })
    }

    fn alloc_layout(&mut self, layout: Layout) -> Option<NonNull<u8>> {
        let align = layout.align();
        let size = layout.size();

        let offset = (self.offset + align - 1) & !(align - 1);
        if offset + size > self.capacity {
            return None;
        }

        let ptr = unsafe { NonNull::new_unchecked(self.ptr.as_ptr().add(offset)) };
        self.offset = offset + size;
        Some(ptr)
    }

    /// Resets the arena, invalidating all previous allocations
    pub fn reset(&mut self) {
        self.offset = 0;
    }

    /// Returns the number of bytes used
    pub fn used(&self) -> usize {
        self.offset
    }

    /// Returns the total capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.capacity, 8).unwrap();
        unsafe { dealloc(self.ptr.as_ptr(), layout) };
    }
}

unsafe impl Send for Arena {}
unsafe impl Sync for Arena {}
