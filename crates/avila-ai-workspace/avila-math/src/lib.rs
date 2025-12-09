//! # Ávila Math
//!
//! Operações matemáticas avançadas para criptografia.
//!
//! ## Módulos
//! - `modular`: Aritmética modular (adição, subtração, multiplicação, inversão)
//! - `montgomery`: Montgomery reduction para exponenciação modular eficiente
//! - `barrett`: Barrett reduction (alternativa ao Montgomery)
//! - `karatsuba`: Multiplicação rápida O(n^1.585)

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]

extern crate alloc;

pub mod modular;
pub mod montgomery;

// Removido: use avila_primitives::U256;

/// Trait para operações modulares
pub trait ModularArithmetic: Sized {
    /// Adição modular: (a + b) mod m
    fn add_mod(&self, rhs: &Self, modulus: &Self) -> Self;

    /// Subtração modular: (a - b) mod m
    fn sub_mod(&self, rhs: &Self, modulus: &Self) -> Self;

    /// Multiplicação modular: (a × b) mod m
    fn mul_mod(&self, rhs: &Self, modulus: &Self) -> Self;

    /// Exponenciação modular: a^exp mod m
    fn pow_mod(&self, exp: &Self, modulus: &Self) -> Self;

    /// Inverso modular: a^(-1) mod m (usando Extended Euclidean Algorithm)
    fn mod_inverse(&self, modulus: &Self) -> Self;
}

impl ModularArithmetic for u64 {
    #[inline]
    fn add_mod(&self, rhs: &Self, modulus: &Self) -> Self {
        let modulus = *modulus;
        assert_ne!(modulus, 0, "modulus must be nonzero");

        let lhs = *self % modulus;
        let rhs = *rhs % modulus;
        let sum = (lhs as u128 + rhs as u128) % modulus as u128;
        sum as u64
    }

    #[inline]
    fn sub_mod(&self, rhs: &Self, modulus: &Self) -> Self {
        let modulus = *modulus;
        assert_ne!(modulus, 0, "modulus must be nonzero");

        let lhs = *self % modulus;
        let rhs = *rhs % modulus;
        if lhs >= rhs {
            lhs - rhs
        } else {
            (modulus as u128 - (rhs as u128 - lhs as u128)) as u64
        }
    }

    #[inline]
    fn mul_mod(&self, rhs: &Self, modulus: &Self) -> Self {
        let modulus = *modulus;
        assert_ne!(modulus, 0, "modulus must be nonzero");

        let lhs = *self % modulus;
        let rhs = *rhs % modulus;
        ((lhs as u128 * rhs as u128) % modulus as u128) as u64
    }

    #[inline]
    fn pow_mod(&self, exp: &Self, modulus: &Self) -> Self {
        let modulus = *modulus;
        assert_ne!(modulus, 0, "modulus must be nonzero");
        if modulus == 1 {
            return 0;
        }

        let mut base = (*self % modulus) as u128;
        let mut exponent = *exp;
        let mut result = 1u128;
        let modulus128 = modulus as u128;

        while exponent > 0 {
            if exponent & 1 == 1 {
                result = (result * base) % modulus128;
            }
            base = (base * base) % modulus128;
            exponent >>= 1;
        }

        result as u64
    }

    #[inline]
    fn mod_inverse(&self, modulus: &Self) -> Self {
        let modulus = *modulus;
        assert_ne!(modulus, 0, "modulus must be nonzero");
        if modulus == 1 {
            return 0;
        }

        let mut a = (*self % modulus) as i128;
        let mut m = modulus as i128;
        if a == 0 {
            panic!("value and modulus must be coprime");
        }

        let mut t0 = 0i128;
        let mut t1 = 1i128;
        let mut r0 = m;
        let mut r1 = a;

        while r1 != 0 {
            let q = r0 / r1;

            let tmp_r = r0 - q * r1;
            r0 = r1;
            r1 = tmp_r;

            let tmp_t = t0 - q * t1;
            t0 = t1;
            t1 = tmp_t;
        }

        if r0 != 1 {
            panic!("value and modulus must be coprime");
        }

        let mut inv = t0 % m;
        if inv < 0 {
            inv += m;
        }

        inv as u64
    }
}

#[cfg(test)]
mod tests {
    use super::ModularArithmetic;

    #[test]
    fn test_add_mod() {
        assert_eq!(5u64.add_mod(&9, &10), 4);
        assert_eq!(u64::MAX.add_mod(&1, &u64::MAX), 0);
    }

    #[test]
    fn test_sub_mod() {
        assert_eq!(3u64.sub_mod(&8, &13), 8);
        assert_eq!(2u64.sub_mod(&2, &17), 0);
    }

    #[test]
    fn test_mul_mod() {
        assert_eq!(7u64.mul_mod(&11, &13), 12);
        assert_eq!(u64::MAX.mul_mod(&u64::MAX, &u64::MAX - 2), 1);
    }

    #[test]
    fn test_pow_mod() {
        assert_eq!(4u64.pow_mod(&13, &497), 445);
        assert_eq!(7u64.pow_mod(&0, &41), 1);
        assert_eq!(123u64.pow_mod(&456, &1), 0);
    }

    #[test]
    fn test_mod_inverse() {
        let modulus = 1_000_000_007u64;
        let value = 5u64;
        let inverse = value.mod_inverse(&modulus);
        assert_eq!(value.mul_mod(&inverse, &modulus), 1);

        let large_modulus = u64::MAX - 58;
        let value = 3u64;
        let inverse = value.mod_inverse(&large_modulus);
        assert_eq!(value.mul_mod(&inverse, &large_modulus), 1);
    }

    #[test]
    #[should_panic]
    fn mod_inverse_with_zero_modulus_panics() {
        let _ = 7u64.mod_inverse(&0);
    }

    #[test]
    #[should_panic]
    fn mod_inverse_non_coprime_panics() {
        let _ = 4u64.mod_inverse(&8);
    }
}
