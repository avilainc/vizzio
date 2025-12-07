//! Comparison operations

use core::cmp::Ordering;

/// Compare two limb arrays
pub fn cmp(lhs: &[u64], rhs: &[u64]) -> Ordering {
    for (a, b) in lhs.iter().rev().zip(rhs.iter().rev()) {
        match a.cmp(b) {
            Ordering::Equal => continue,
            other => return other,
        }
    }
    Ordering::Equal
}

/// Check if limb array is zero
pub fn is_zero(limbs: &[u64]) -> bool {
    limbs.iter().all(|&x| x == 0)
}

/// Check if first array is less than second
pub fn lt(lhs: &[u64], rhs: &[u64]) -> bool {
    matches!(cmp(lhs, rhs), Ordering::Less)
}

/// Check if first array is less than or equal to second
pub fn le(lhs: &[u64], rhs: &[u64]) -> bool {
    !matches!(cmp(lhs, rhs), Ordering::Greater)
}

/// Check if first array is greater than second
pub fn gt(lhs: &[u64], rhs: &[u64]) -> bool {
    matches!(cmp(lhs, rhs), Ordering::Greater)
}

/// Check if first array is greater than or equal to second
pub fn ge(lhs: &[u64], rhs: &[u64]) -> bool {
    !matches!(cmp(lhs, rhs), Ordering::Less)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp_equal() {
        let a = [1u64, 2u64, 3u64];
        let b = [1u64, 2u64, 3u64];
        assert_eq!(cmp(&a, &b), Ordering::Equal);
    }

    #[test]
    fn test_cmp_less() {
        let a = [1u64, 2u64, 3u64];
        let b = [1u64, 2u64, 4u64];
        assert_eq!(cmp(&a, &b), Ordering::Less);
    }

    #[test]
    fn test_cmp_greater() {
        let a = [1u64, 3u64, 3u64];
        let b = [1u64, 2u64, 3u64];
        assert_eq!(cmp(&a, &b), Ordering::Greater);
    }

    #[test]
    fn test_is_zero() {
        assert!(is_zero(&[0u64, 0u64, 0u64]));
        assert!(!is_zero(&[0u64, 0u64, 1u64]));
    }
}
