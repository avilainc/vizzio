//! 2048-bit unsigned integer type (moved from lib.rs)

use core::fmt;

/// 2048-bit unsigned integer (32 × 64-bit limbs) - RSA-2048
#[repr(align(128))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct U2048 {
    /// Limbs in little-endian order
    pub limbs: [u64; 32],
}

impl U2048 {
    /// Zero constant
    pub const ZERO: Self = Self { limbs: [0; 32] };

    /// One constant
    pub const ONE: Self = {
        let mut limbs = [0u64; 32];
        limbs[0] = 1;
        Self { limbs }
    };

    /// Maximum value
    pub const MAX: Self = Self { limbs: [u64::MAX; 32] };

    /// Number of bits
    pub const BITS: u32 = 2048;

    /// Number of limbs
    pub const LIMBS: usize = 32;

    /// Creates from u64
    #[inline]
    pub const fn from_u64(val: u64) -> Self {
        let mut limbs = [0u64; 32];
        limbs[0] = val;
        Self { limbs }
    }

    /// Checks if zero
    pub fn is_zero(&self) -> bool {
        self.limbs.iter().all(|&x| x == 0)
    }

    /// Addition with carry
    pub fn add_assign(&mut self, other: &Self) -> bool {
        let mut carry = 0u64;
        for i in 0..32 {
            let (sum, c1) = self.limbs[i].overflowing_add(other.limbs[i]);
            let (sum, c2) = sum.overflowing_add(carry);
            self.limbs[i] = sum;
            carry = (c1 as u64) + (c2 as u64);
        }
        carry != 0
    }

    /// Subtraction with borrow
    pub fn sub_assign(&mut self, other: &Self) -> bool {
        let mut borrow = 0u64;
        for i in 0..32 {
            let (diff, b1) = self.limbs[i].overflowing_sub(other.limbs[i]);
            let (diff, b2) = diff.overflowing_sub(borrow);
            self.limbs[i] = diff;
            borrow = (b1 as u64) + (b2 as u64);
        }
        borrow != 0
    }

    /// Multiply by u64
    pub fn mul_u64(&self, rhs: u64) -> Self {
        let mut result = Self::ZERO;
        let mut carry = 0u64;
        for i in 0..32 {
            let product = (self.limbs[i] as u128) * (rhs as u128) + (carry as u128);
            result.limbs[i] = product as u64;
            carry = (product >> 64) as u64;
        }
        result
    }
}

impl Default for U2048 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<u64> for U2048 {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl core::ops::Add for U2048 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.add_assign(&rhs);
        self
    }
}

impl core::ops::Sub for U2048 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.sub_assign(&rhs);
        self
    }
}

impl core::ops::Mul for U2048 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Self::ZERO;
        for i in 0..32 {
            if rhs.limbs[i] == 0 {
                continue;
            }
            let mut carry = 0u64;
            for j in 0..32 {
                if i + j >= 32 {
                    break;
                }
                let product = (self.limbs[j] as u128) * (rhs.limbs[i] as u128)
                    + (result.limbs[i + j] as u128)
                    + (carry as u128);
                result.limbs[i + j] = product as u64;
                carry = (product >> 64) as u64;
            }
        }
        result
    }
}

impl core::cmp::Ord for U2048 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        for i in (0..32).rev() {
            match self.limbs[i].cmp(&other.limbs[i]) {
                core::cmp::Ordering::Equal => continue,
                ord => return ord,
            }
        }
        core::cmp::Ordering::Equal
    }
}

impl core::cmp::PartialOrd for U2048 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for U2048 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U2048(")?;
        for limb in self.limbs.iter().rev() {
            write!(f, "{:016x}", limb)?;
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert!(U2048::ZERO.is_zero());
        assert!(!U2048::ONE.is_zero());
    }

    #[test]
    fn test_from_u64() {
        let n = U2048::from(42u64);
        assert_eq!(n.limbs[0], 42);
    }

    #[test]
    fn test_arithmetic() {
        let a = U2048::from(100u64);
        let b = U2048::from(50u64);
        assert_eq!((a + b).limbs[0], 150);
        assert_eq!((a - b).limbs[0], 50);
        assert_eq!((a * b).limbs[0], 5000);
    }

    #[test]
    fn test_comparison() {
        let a = U2048::from(42u64);
        let b = U2048::from(100u64);
        assert!(a < b);
        assert!(b > a);
    }
}
