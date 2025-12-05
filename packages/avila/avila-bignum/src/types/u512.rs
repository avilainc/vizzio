//! 512-bit unsigned integer type

use core::fmt;

/// 512-bit unsigned integer (8 limbs of 64 bits)
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct U512 {
    pub limbs: [u64; 8],
}

impl U512 {
    /// The value zero
    pub const ZERO: Self = Self { limbs: [0; 8] };

    /// The value one
    pub const ONE: Self = Self { limbs: [1, 0, 0, 0, 0, 0, 0, 0] };

    /// Maximum value
    pub const MAX: Self = Self { limbs: [u64::MAX; 8] };

    /// Create from a single u64
    pub const fn from_u64(value: u64) -> Self {
        Self {
            limbs: [value, 0, 0, 0, 0, 0, 0, 0],
        }
    }
}

impl From<u64> for U512 {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl fmt::Debug for U512 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U512(")?;
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
        assert_eq!(U512::ZERO.limbs, [0; 8]);
        assert_eq!(U512::ONE.limbs, [1, 0, 0, 0, 0, 0, 0, 0]);
    }
}
