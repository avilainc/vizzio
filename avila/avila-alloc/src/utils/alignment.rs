//! Memory alignment utilities

/// Aligns a value up to the next multiple of align
pub const fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

/// Aligns a value down to the previous multiple of align
pub const fn align_down(value: usize, align: usize) -> usize {
    value & !(align - 1)
}

/// Checks if a value is aligned
pub const fn is_aligned(value: usize, align: usize) -> bool {
    value & (align - 1) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_up() {
        assert_eq!(align_up(0, 8), 0);
        assert_eq!(align_up(1, 8), 8);
        assert_eq!(align_up(7, 8), 8);
        assert_eq!(align_up(8, 8), 8);
        assert_eq!(align_up(9, 8), 16);
    }

    #[test]
    fn test_align_down() {
        assert_eq!(align_down(0, 8), 0);
        assert_eq!(align_down(7, 8), 0);
        assert_eq!(align_down(8, 8), 8);
        assert_eq!(align_down(15, 8), 8);
        assert_eq!(align_down(16, 8), 16);
    }

    #[test]
    fn test_is_aligned() {
        assert!(is_aligned(0, 8));
        assert!(!is_aligned(1, 8));
        assert!(is_aligned(8, 8));
        assert!(is_aligned(16, 8));
        assert!(!is_aligned(9, 8));
    }
}
