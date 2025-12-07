//! 4096-bit signed integer type (moved from lib.rs)

use super::U4096;
use core::fmt;

/// 4096-bit signed integer
#[repr(align(256))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct I4096 {
    /// Magnitude (absolute value)
    pub mag: U4096,
    /// Sign bit (false = positive, true = negative)
    pub neg: bool,
}

impl I4096 {
    /// Zero constant
    pub const ZERO: Self = Self {
        mag: U4096::ZERO,
        neg: false,
    };

    /// One constant
    pub const ONE: Self = Self {
        mag: U4096::ONE,
        neg: false,
    };

    /// Creates from i64
    pub const fn from_i64(val: i64) -> Self {
        let neg = val < 0;
        let mag = U4096::from_u64(val.unsigned_abs());
        Self { mag, neg }
    }

    /// Checks if zero
    pub fn is_zero(&self) -> bool {
        self.mag.is_zero()
    }

    /// Checks if negative
    pub const fn is_negative(&self) -> bool {
        self.neg
    }
}

impl Default for I4096 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<i64> for I4096 {
    fn from(value: i64) -> Self {
        Self::from_i64(value)
    }
}

impl fmt::Debug for I4096 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.neg {
            write!(f, "-")?;
        }
        write!(f, "{:?}", self.mag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signed() {
        let pos = I4096::from(42i64);
        assert!(!pos.is_negative());

        let neg = I4096::from(-42i64);
        assert!(neg.is_negative());

        let zero = I4096::ZERO;
        assert!(zero.is_zero());
    }
}
