//! 4096-bit unsigned integer type (moved from lib.rs)

use core::fmt;

/// 4096-bit unsigned integer (64 × 64-bit limbs) - RSA-4096
#[repr(align(256))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct U4096 {
    /// Limbs in little-endian order
    pub limbs: [u64; 64],
}

impl U4096 {
    /// Zero constant
    pub const ZERO: Self = Self { limbs: [0; 64] };

    /// One constant
    pub const ONE: Self = {
        let mut limbs = [0u64; 64];
        limbs[0] = 1;
        Self { limbs }
    };

    /// Maximum value
    pub const MAX: Self = Self { limbs: [u64::MAX; 64] };

    /// Number of bits
    pub const BITS: u32 = 4096;

    /// Number of limbs
    pub const LIMBS: usize = 64;

    /// Creates from u64
    #[inline]
    pub const fn from_u64(val: u64) -> Self {
        let mut limbs = [0u64; 64];
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
        for i in 0..64 {
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
        for i in 0..64 {
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
        for i in 0..64 {
            let product = (self.limbs[i] as u128) * (rhs as u128) + (carry as u128);
            result.limbs[i] = product as u64;
            carry = (product >> 64) as u64;
        }
        result
    }

    /// Division by u64 (returns quotient and remainder)
    pub fn div_rem_u64(&self, divisor: u64) -> (Self, u64) {
        if divisor == 0 {
            panic!("Division by zero");
        }
        let mut quotient = Self::ZERO;
        let mut remainder = 0u64;

        for i in (0..64).rev() {
            let dividend = ((remainder as u128) << 64) | (self.limbs[i] as u128);
            quotient.limbs[i] = (dividend / divisor as u128) as u64;
            remainder = (dividend % divisor as u128) as u64;
        }

        (quotient, remainder)
    }
}

impl Default for U4096 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<u64> for U4096 {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl core::ops::Add for U4096 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.add_assign(&rhs);
        self
    }
}

impl core::ops::Sub for U4096 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.sub_assign(&rhs);
        self
    }
}

impl core::ops::Mul for U4096 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Self::ZERO;
        for i in 0..64 {
            if rhs.limbs[i] == 0 {
                continue;
            }
            let mut carry = 0u64;
            for j in 0..64 {
                if i + j >= 64 {
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

impl core::cmp::Ord for U4096 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        for i in (0..64).rev() {
            match self.limbs[i].cmp(&other.limbs[i]) {
                core::cmp::Ordering::Equal => continue,
                ord => return ord,
            }
        }
        core::cmp::Ordering::Equal
    }
}

impl core::cmp::PartialOrd for U4096 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for U4096 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U4096(")?;
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
        assert!(U4096::ZERO.is_zero());
        assert!(!U4096::ONE.is_zero());
    }

    #[test]
    fn test_from_u64() {
        let n = U4096::from(42u64);
        assert_eq!(n.limbs[0], 42);
    }
}
