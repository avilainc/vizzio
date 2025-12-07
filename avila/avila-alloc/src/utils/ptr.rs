//! Pointer utilities for safe pointer arithmetic

use core::ptr::{self, NonNull};
use core::mem;

/// Safe pointer arithmetic operations
pub struct PtrExt;

impl PtrExt {
    /// Safely adds an offset to a pointer
    #[inline]
    pub unsafe fn offset_bytes<T>(ptr: *mut T, bytes: usize) -> *mut T {
        (ptr as *mut u8).add(bytes) as *mut T
    }

    /// Safely subtracts pointers and returns byte distance
    #[inline]
    pub unsafe fn distance_bytes<T>(start: *const T, end: *const T) -> usize {
        (end as usize).saturating_sub(start as usize)
    }

    /// Checks if pointer is aligned for type T
    #[inline]
    pub fn is_aligned_for<T>(ptr: *const T) -> bool {
        let align = mem::align_of::<T>();
        (ptr as usize) & (align - 1) == 0
    }

    /// Aligns pointer up to type T's alignment
    #[inline]
    pub fn align_up_for<T>(ptr: *mut u8) -> *mut u8 {
        let align = mem::align_of::<T>();
        let addr = ptr as usize;
        let aligned = super::align_up(addr, align);
        aligned as *mut u8
    }

    /// Checks if two memory ranges overlap
    #[inline]
    pub fn ranges_overlap(
        ptr1: *const u8,
        len1: usize,
        ptr2: *const u8,
        len2: usize,
    ) -> bool {
        let start1 = ptr1 as usize;
        let end1 = start1 + len1;
        let start2 = ptr2 as usize;
        let end2 = start2 + len2;

        start1 < end2 && start2 < end1
    }
}

/// NonNull pointer extensions
pub trait NonNullExt<T> {
    /// Creates from raw pointer with alignment check
    fn new_aligned(ptr: *mut T) -> Option<NonNull<T>>;

    /// Offsets the pointer by count elements
    unsafe fn offset(self, count: isize) -> NonNull<T>;

    /// Adds offset in bytes
    unsafe fn add_bytes(self, bytes: usize) -> NonNull<T>;
}

impl<T> NonNullExt<T> for NonNull<T> {
    fn new_aligned(ptr: *mut T) -> Option<NonNull<T>> {
        if ptr.is_null() || !PtrExt::is_aligned_for(ptr) {
            None
        } else {
            NonNull::new(ptr)
        }
    }

    unsafe fn offset(self, count: isize) -> NonNull<T> {
        NonNull::new_unchecked(self.as_ptr().offset(count))
    }

    unsafe fn add_bytes(self, bytes: usize) -> NonNull<T> {
        let ptr = PtrExt::offset_bytes(self.as_ptr(), bytes);
        NonNull::new_unchecked(ptr)
    }
}

/// Slice utilities
pub struct SliceExt;

impl SliceExt {
    /// Safely splits a slice into two parts
    #[inline]
    pub fn split_at_checked<T>(slice: &[T], mid: usize) -> Option<(&[T], &[T])> {
        if mid <= slice.len() {
            Some(slice.split_at(mid))
        } else {
            None
        }
    }

    /// Safely splits a mutable slice into two parts
    #[inline]
    pub fn split_at_mut_checked<T>(slice: &mut [T], mid: usize) -> Option<(&mut [T], &mut [T])> {
        if mid <= slice.len() {
            Some(slice.split_at_mut(mid))
        } else {
            None
        }
    }

    /// Copies non-overlapping memory
    #[inline]
    pub unsafe fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize) {
        debug_assert!(!PtrExt::ranges_overlap(
            src as *const u8,
            count * mem::size_of::<T>(),
            dst as *const u8,
            count * mem::size_of::<T>(),
        ));
        ptr::copy_nonoverlapping(src, dst, count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ptr_alignment() {
        let value: u64 = 42;
        let ptr = &value as *const u64;
        assert!(PtrExt::is_aligned_for(ptr));
    }

    #[test]
    fn test_ranges_overlap() {
        let data = [1, 2, 3, 4, 5];
        let ptr = data.as_ptr();

        // Same range
        assert!(PtrExt::ranges_overlap(ptr, 5, ptr, 5));

        // Non-overlapping
        unsafe {
            assert!(!PtrExt::ranges_overlap(
                ptr,
                2,
                ptr.add(3),
                2,
            ));
        }
    }

    #[test]
    fn test_slice_split_checked() {
        let data = [1, 2, 3, 4, 5];

        let (left, right) = SliceExt::split_at_checked(&data, 3).unwrap();
        assert_eq!(left, &[1, 2, 3]);
        assert_eq!(right, &[4, 5]);

        assert!(SliceExt::split_at_checked(&data, 10).is_none());
    }
}
