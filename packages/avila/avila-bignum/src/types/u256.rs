//! 256-bit unsigned integer type

use core::fmt;

/// 256-bit unsigned integer (4 limbs of 64 bits)
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct U256 {
    pub limbs: [u64; 4],
}

impl U256 {
    /// The value zero
    pub const ZERO: Self = Self { limbs: [0; 4] };

    /// The value one
    pub const ONE: Self = Self { limbs: [1, 0, 0, 0] };

    /// Maximum value
    pub const MAX: Self = Self { limbs: [u64::MAX; 4] };

    /// Create from a single u64
    pub const fn from_u64(value: u64) -> Self {
        Self {
            limbs: [value, 0, 0, 0],
        }
    }
}

impl From<u64> for U256 {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl fmt::Debug for U256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U256({:016x}{:016x}{:016x}{:016x})",
            self.limbs[3], self.limbs[2], self.limbs[1], self.limbs[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(U256::ZERO.limbs, [0, 0, 0, 0]);
        assert_eq!(U256::ONE.limbs, [1, 0, 0, 0]);
        assert_eq!(U256::MAX.limbs, [u64::MAX; 4]);
    }

    #[test]
    fn test_from_u64() {
        let val = U256::from(42u64);
        assert_eq!(val.limbs, [42, 0, 0, 0]);
    }
}
