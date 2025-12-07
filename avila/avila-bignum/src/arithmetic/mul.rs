//! Multiplication operations
//!
//! Implements various multiplication algorithms:
//! - Schoolbook multiplication for smaller numbers
//! - Karatsuba for medium-sized numbers
//! - FFT-based for very large numbers (future)

/// Multiply two u64 values and return low and high parts
#[inline]
pub const fn mul_wide(a: u64, b: u64) -> (u64, u64) {
    let product = a as u128 * b as u128;
    (product as u64, (product >> 64) as u64)
}

/// Schoolbook multiplication
/// result must be initialized to zero and have length lhs.len() + rhs.len()
pub fn mul_schoolbook(lhs: &[u64], rhs: &[u64], result: &mut [u64]) {
    for (i, &a) in lhs.iter().enumerate() {
        let mut carry = 0u64;
        for (j, &b) in rhs.iter().enumerate() {
            let (lo, hi) = mul_wide(a, b);
            let (sum1, c1) = lo.overflowing_add(result[i + j]);
            let (sum2, c2) = sum1.overflowing_add(carry);
            result[i + j] = sum2;
            carry = hi + c1 as u64 + c2 as u64;
        }
        result[i + rhs.len()] = carry;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_wide() {
        let (lo, hi) = mul_wide(2, 3);
        assert_eq!(lo, 6);
        assert_eq!(hi, 0);
    }

    #[test]
    fn test_mul_wide_overflow() {
        let (lo, hi) = mul_wide(u64::MAX, u64::MAX);
        assert_eq!(lo, 1);
        assert_eq!(hi, u64::MAX - 1);
    }

    #[test]
    fn test_mul_schoolbook() {
        let a = [5u64, 0u64];
        let b = [3u64, 0u64];
        let mut result = [0u64; 4];
        mul_schoolbook(&a, &b, &mut result);
        assert_eq!(result[0], 15);
        assert_eq!(result[1], 0);
    }
}
