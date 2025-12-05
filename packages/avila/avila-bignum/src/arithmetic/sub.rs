//! Subtraction operations with borrow handling

/// Subtract two u64 values and return the difference and borrow
#[inline]
pub const fn sbb(a: u64, b: u64, borrow: u64) -> (u64, u64) {
    let diff = (a as u128).wrapping_sub(b as u128).wrapping_sub(borrow as u128);
    (diff as u64, if diff >> 64 != 0 { 1 } else { 0 })
}

/// Subtract rhs from result in-place with borrow propagation
pub fn sub_assign(result: &mut [u64], rhs: &[u64]) -> u64 {
    let mut borrow = 0u64;
    for (a, &b) in result.iter_mut().zip(rhs.iter()) {
        let (diff, br) = sbb(*a, b, borrow);
        *a = diff;
        borrow = br;
    }
    borrow
}

/// Subtract rhs from lhs and store in result
pub fn sub(lhs: &[u64], rhs: &[u64], result: &mut [u64]) -> u64 {
    result.copy_from_slice(lhs);
    sub_assign(result, rhs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sbb_no_borrow() {
        let (diff, borrow) = sbb(30, 10, 0);
        assert_eq!(diff, 20);
        assert_eq!(borrow, 0);
    }

    #[test]
    fn test_sbb_with_borrow() {
        let (diff, borrow) = sbb(10, 20, 0);
        assert_eq!(diff, u64::MAX - 9);
        assert_eq!(borrow, 1);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = [30u64, 50u64];
        let b = [10u64, 20u64];
        let borrow = sub_assign(&mut a, &b);
        assert_eq!(a, [20, 30]);
        assert_eq!(borrow, 0);
    }
}
