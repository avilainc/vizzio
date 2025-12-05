//! 1024-bit unsigned integer type (moved from lib.rs)

use core::fmt;

/// 1024-bit unsigned integer (16 × 64-bit limbs)
#[repr(align(64))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct U1024 {
    /// Limbs in little-endian order
    pub limbs: [u64; 16],
}

impl U1024 {
    /// Zero constant
    pub const ZERO: Self = Self { limbs: [0; 16] };

    /// One constant
    pub const ONE: Self = Self {
        limbs: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };

    /// Maximum value
    pub const MAX: Self = Self { limbs: [u64::MAX; 16] };

    /// Number of bits
    pub const BITS: u32 = 1024;

    /// Number of limbs
    pub const LIMBS: usize = 16;

    /// Creates from u64
    #[inline]
    pub const fn from_u64(val: u64) -> Self {
        let mut limbs = [0u64; 16];
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
        for i in 0..16 {
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
        for i in 0..16 {
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
        for i in 0..16 {
            let product = (self.limbs[i] as u128) * (rhs as u128) + (carry as u128);
            result.limbs[i] = product as u64;
            carry = (product >> 64) as u64;
        }
        result
    }

    /// Left shift by bits
    pub fn shl(&self, bits: u32) -> Self {
        if bits == 0 {
            return *self;
        }
        if bits >= 1024 {
            return Self::ZERO;
        }

        let mut result = Self::ZERO;
        let limb_shift = (bits / 64) as usize;
        let bit_shift = (bits % 64) as u32;

        if bit_shift == 0 {
            for i in limb_shift..16 {
                result.limbs[i] = self.limbs[i - limb_shift];
            }
        } else {
            let carry_shift = 64 - bit_shift;
            for i in limb_shift..16 {
                result.limbs[i] = self.limbs[i - limb_shift] << bit_shift;
                if i > limb_shift {
                    result.limbs[i] |= self.limbs[i - limb_shift - 1] >> carry_shift;
                }
            }
        }
        result
    }

    /// Right shift by bits
    pub fn shr(&self, bits: u32) -> Self {
        if bits == 0 {
            return *self;
        }
        if bits >= 1024 {
            return Self::ZERO;
        }

        let mut result = Self::ZERO;
        let limb_shift = (bits / 64) as usize;
        let bit_shift = (bits % 64) as u32;

        if bit_shift == 0 {
            for i in 0..(16 - limb_shift) {
                result.limbs[i] = self.limbs[i + limb_shift];
            }
        } else {
            let carry_shift = 64 - bit_shift;
            for i in 0..(16 - limb_shift) {
                result.limbs[i] = self.limbs[i + limb_shift] >> bit_shift;
                if i + limb_shift + 1 < 16 {
                    result.limbs[i] |= self.limbs[i + limb_shift + 1] << carry_shift;
                }
            }
        }
        result
    }

    /// Bitwise AND
    pub fn bitand(&self, other: &Self) -> Self {
        let mut result = Self::ZERO;
        for i in 0..16 {
            result.limbs[i] = self.limbs[i] & other.limbs[i];
        }
        result
    }

    /// Bitwise OR
    pub fn bitor(&self, other: &Self) -> Self {
        let mut result = Self::ZERO;
        for i in 0..16 {
            result.limbs[i] = self.limbs[i] | other.limbs[i];
        }
        result
    }

    /// Bitwise XOR
    pub fn bitxor(&self, other: &Self) -> Self {
        let mut result = Self::ZERO;
        for i in 0..16 {
            result.limbs[i] = self.limbs[i] ^ other.limbs[i];
        }
        result
    }

    /// Bitwise NOT
    pub fn not(&self) -> Self {
        let mut result = Self::ZERO;
        for i in 0..16 {
            result.limbs[i] = !self.limbs[i];
        }
        result
    }

    /// Count leading zeros
    pub fn leading_zeros(&self) -> u32 {
        for i in (0..16).rev() {
            let zeros = self.limbs[i].leading_zeros();
            if zeros < 64 {
                return ((15 - i) as u32) * 64 + zeros;
            }
        }
        1024
    }

    /// Count trailing zeros
    pub fn trailing_zeros(&self) -> u32 {
        for i in 0..16 {
            let zeros = self.limbs[i].trailing_zeros();
            if zeros < 64 {
                return (i as u32) * 64 + zeros;
            }
        }
        1024
    }
}

impl Default for U1024 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<u64> for U1024 {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl core::ops::Add for U1024 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.add_assign(&rhs);
        self
    }
}

impl core::ops::Sub for U1024 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.sub_assign(&rhs);
        self
    }
}

impl core::ops::Mul for U1024 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // Schoolbook multiplication (truncated to 1024 bits)
        let mut result = Self::ZERO;
        for i in 0..16 {
            if rhs.limbs[i] == 0 {
                continue;
            }
            let mut carry = 0u64;
            for j in 0..16 {
                if i + j >= 16 {
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

impl core::ops::BitAnd for U1024 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.bitand(&rhs)
    }
}

impl core::ops::BitOr for U1024 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.bitor(&rhs)
    }
}

impl core::ops::BitXor for U1024 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.bitxor(&rhs)
    }
}

impl core::ops::Not for U1024 {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.not()
    }
}

impl core::ops::Shl<u32> for U1024 {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        self.shl(rhs)
    }
}

impl core::ops::Shr<u32> for U1024 {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        self.shr(rhs)
    }
}

impl core::cmp::Ord for U1024 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        for i in (0..16).rev() {
            match self.limbs[i].cmp(&other.limbs[i]) {
                core::cmp::Ordering::Equal => continue,
                ord => return ord,
            }
        }
        core::cmp::Ordering::Equal
    }
}

impl core::cmp::PartialOrd for U1024 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for U1024 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U1024(")?;
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
        assert!(U1024::ZERO.is_zero());
        assert!(!U1024::ONE.is_zero());
        assert_eq!(U1024::ONE.limbs[0], 1);
    }

    #[test]
    fn test_from_u64() {
        let n = U1024::from(42u64);
        assert_eq!(n.limbs[0], 42);
    }

    #[test]
    fn test_add() {
        let a = U1024::from(10u64);
        let b = U1024::from(32u64);
        let c = a + b;
        assert_eq!(c.limbs[0], 42);
    }

    #[test]
    fn test_sub() {
        let a = U1024::from(100u64);
        let b = U1024::from(42u64);
        let c = a - b;
        assert_eq!(c.limbs[0], 58);
    }

    #[test]
    fn test_mul() {
        let a = U1024::from(6u64);
        let b = U1024::from(7u64);
        let c = a * b;
        assert_eq!(c.limbs[0], 42);
    }

    #[test]
    fn test_mul_large() {
        let a = U1024::from(1000u64);
        let b = U1024::from(2000u64);
        let c = a * b;
        assert_eq!(c.limbs[0], 2000000);
    }

    #[test]
    fn test_shifts() {
        let a = U1024::from(1u64);
        let b = a << 5;
        assert_eq!(b.limbs[0], 32);
        let c = b >> 4;
        assert_eq!(c.limbs[0], 2);
    }

    #[test]
    fn test_bitwise() {
        let a = U1024::from(0b1010u64);
        let b = U1024::from(0b1100u64);
        assert_eq!((a & b).limbs[0], 0b1000);
        assert_eq!((a | b).limbs[0], 0b1110);
        assert_eq!((a ^ b).limbs[0], 0b0110);
    }

    #[test]
    fn test_comparison() {
        let a = U1024::from(42u64);
        let b = U1024::from(100u64);
        assert!(a < b);
        assert!(b > a);
        assert!(a <= a);
        assert_eq!(a, a);
    }

    #[test]
    fn test_leading_zeros() {
        let a = U1024::from(1u64);
        assert_eq!(a.leading_zeros(), 1023);
        let b = U1024::ZERO;
        assert_eq!(b.leading_zeros(), 1024);
    }
}
