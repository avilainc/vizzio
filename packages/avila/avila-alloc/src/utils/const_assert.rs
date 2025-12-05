//! Compile-time assertions and type-level checks

/// Asserts that a type is Send at compile time
pub const fn assert_send<T: Send>() {}

/// Asserts that a type is Sync at compile time
pub const fn assert_sync<T: Sync>() {}

/// Asserts that a type is Copy at compile time
pub const fn assert_copy<T: Copy>() {}

/// Asserts that a type is Sized at compile time
pub const fn assert_sized<T: Sized>() {}

/// Asserts that alignment is a power of 2
pub const fn assert_alignment_pow2(align: usize) {
    assert!(align > 0 && (align & (align - 1)) == 0, "alignment must be power of 2");
}

/// Asserts that size is within reasonable bounds
pub const fn assert_size_reasonable<T>() {
    const MAX_SIZE: usize = isize::MAX as usize;
    assert!(core::mem::size_of::<T>() <= MAX_SIZE, "size too large");
}

/// Compile-time min
pub const fn const_min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

/// Compile-time max
pub const fn const_max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

/// Checks if value is power of 2
pub const fn is_pow2(value: usize) -> bool {
    value > 0 && (value & (value - 1)) == 0
}

/// Next power of 2 >= value
pub const fn next_pow2(mut value: usize) -> usize {
    value -= 1;
    value |= value >> 1;
    value |= value >> 2;
    value |= value >> 4;
    value |= value >> 8;
    value |= value >> 16;
    #[cfg(target_pointer_width = "64")]
    {
        value |= value >> 32;
    }
    value + 1
}

/// Calculates number of elements that fit in capacity
pub const fn elements_fit<T>(capacity_bytes: usize) -> usize {
    capacity_bytes / core::mem::size_of::<T>()
}

/// Calculates bytes needed for N elements of T
pub const fn bytes_for_elements<T>(n: usize) -> usize {
    n * core::mem::size_of::<T>()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_pow2() {
        assert!(is_pow2(1));
        assert!(is_pow2(2));
        assert!(is_pow2(4));
        assert!(is_pow2(8));
        assert!(!is_pow2(3));
        assert!(!is_pow2(6));
    }
    
    #[test]
    fn test_next_pow2() {
        assert_eq!(next_pow2(1), 1);
        assert_eq!(next_pow2(2), 2);
        assert_eq!(next_pow2(3), 4);
        assert_eq!(next_pow2(5), 8);
        assert_eq!(next_pow2(15), 16);
    }
    
    #[test]
    fn test_elements_fit() {
        assert_eq!(elements_fit::<u8>(10), 10);
        assert_eq!(elements_fit::<u32>(16), 4);
        assert_eq!(elements_fit::<u64>(32), 4);
    }
    
    #[test]
    fn test_const_minmax() {
        assert_eq!(const_min(5, 10), 5);
        assert_eq!(const_max(5, 10), 10);
    }
}
