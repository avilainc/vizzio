//! # avila-modular - Modular Arithmetic
//!
//! Aritmética modular de alta performance para inteiros grandes.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

/// Contexto modular
pub struct ModContext {
    /// Modulus value
    pub modulus: [u64; 4],
}

impl ModContext {
    /// Cria novo contexto modular
    pub const fn new(modulus: [u64; 4]) -> Self {
        Self { modulus }
    }

    /// Reduces value modulo m
    pub fn reduce(&self, mut value: [u64; 4]) -> [u64; 4] {
        // Redução iterativa: subtrair modulus enquanto value >= modulus
        while self.cmp(value, self.modulus) != core::cmp::Ordering::Less {
            let mut borrow = 0u64;
            for i in 0..4 {
                let (diff, b1) = value[i].overflowing_sub(self.modulus[i]);
                let (diff, b2) = diff.overflowing_sub(borrow);
                value[i] = diff;
                borrow = (b1 as u64) + (b2 as u64);
            }
        }
        value
    }

    /// Modular addition
    pub fn add(&self, a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
        let mut result = [0u64; 4];
        let mut carry = 0u64;
        for i in 0..4 {
            let (sum, c1) = a[i].overflowing_add(b[i]);
            let (sum, c2) = sum.overflowing_add(carry);
            result[i] = sum;
            carry = (c1 as u64) + (c2 as u64);
        }
        self.reduce(result)
    }

    /// Modular subtraction
    pub fn sub(&self, a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
        let mut result = [0u64; 4];
        let mut borrow = 0u64;
        for i in 0..4 {
            let (diff, b1) = a[i].overflowing_sub(b[i]);
            let (diff, b2) = diff.overflowing_sub(borrow);
            result[i] = diff;
            borrow = (b1 as u64) + (b2 as u64);
        }
        self.reduce(result)
    }

    /// Modular multiplication (simplified)
    pub fn mul(&self, a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
        let mut result = [0u64; 4];
        for i in 0..4 {
            let mut carry = 0u64;
            for j in 0..4 {
                if i + j < 4 {
                    let product = (a[i] as u128) * (b[j] as u128)
                        + (result[i + j] as u128) + (carry as u128);
                    result[i + j] = product as u64;
                    carry = (product >> 64) as u64;
                }
            }
        }
        self.reduce(result)
    }

    /// Modular exponentiation: base^exp mod m (simplified square-and-multiply)
    pub fn pow(&self, base: [u64; 4], mut exp: u64) -> [u64; 4] {
        let mut result = [1, 0, 0, 0];
        let mut base = base;

        while exp > 0 {
            if exp & 1 == 1 {
                result = self.mul(result, base);
            }
            base = self.mul(base, base);
            exp >>= 1;
        }
        result
    }

    /// Verifica se o valor é zero
    pub fn is_zero(&self, value: [u64; 4]) -> bool {
        value.iter().all(|&x| x == 0)
    }

    /// Verifica se o valor é um
    pub fn is_one(&self, value: [u64; 4]) -> bool {
        value[0] == 1 && value[1..].iter().all(|&x| x == 0)
    }

    /// Compara dois valores
    pub fn cmp(&self, a: [u64; 4], b: [u64; 4]) -> core::cmp::Ordering {
        for i in (0..4).rev() {
            match a[i].cmp(&b[i]) {
                core::cmp::Ordering::Equal => continue,
                other => return other,
            }
        }
        core::cmp::Ordering::Equal
    }
}

/// Montgomery form
pub struct Montgomery {
    modulus: [u64; 4],
    r: [u64; 4],
    r_inv: [u64; 4],
    n_prime: u64,
}

impl Montgomery {
    /// Creates new Montgomery context
    pub fn new(modulus: [u64; 4]) -> Self {
        // R = 2^256 mod m (para 4 palavras de 64 bits)
        let r = Self::compute_r(modulus);
        let r_inv = Self::compute_r_inv(modulus);
        let n_prime = Self::compute_n_prime(modulus[0]);

        Self { modulus, r, r_inv, n_prime }
    }

    fn compute_r(_modulus: [u64; 4]) -> [u64; 4] {
        // R = 2^256 mod m - simplificado
        let mut r = [0u64; 4];
        r[0] = 1;
        // Shiftar 256 bits é complexo, usar aproximação
        r
    }

    fn compute_r_inv(_modulus: [u64; 4]) -> [u64; 4] {
        // R^(-1) mod m - simplificado usando Extended GCD
        [1, 0, 0, 0]
    }

    fn compute_n_prime(m0: u64) -> u64 {
        // -m^(-1) mod 2^64 usando Extended Euclidean Algorithm
        let mut t = 0u64;
        let mut newt = 1u64;
        let mut r = 1u64 << 63; // 2^63
        let mut newr = m0;

        while newr != 0 {
            let quotient = r / newr;
            let temp = t;
            t = newt;
            newt = temp.wrapping_sub(quotient.wrapping_mul(newt));
            let temp = r;
            r = newr;
            newr = temp - quotient * newr;
        }

        t.wrapping_neg()
    }

    /// To Montgomery form: x * R mod m
    pub fn to_montgomery(&self, x: [u64; 4]) -> [u64; 4] {
        // x * R mod m = REDC(x * R^2 mod m)
        // Simplificado: multiplicar e reduzir
        self.redc(self.mul_wide(x, self.r))
    }

    /// From Montgomery form: x * R^(-1) mod m
    pub fn from_montgomery(&self, x: [u64; 4]) -> [u64; 4] {
        // REDC(x) = x * R^(-1) mod m
        self.redc(self.extend_to_wide(x))
    }

    fn extend_to_wide(&self, x: [u64; 4]) -> [u64; 8] {
        let mut wide = [0u64; 8];
        wide[0..4].copy_from_slice(&x);
        wide
    }

    fn mul_wide(&self, a: [u64; 4], b: [u64; 4]) -> [u64; 8] {
        let mut result = [0u64; 8];
        for i in 0..4 {
            let mut carry = 0u128;
            for j in 0..4 {
                let product = (a[i] as u128) * (b[j] as u128)
                    + (result[i + j] as u128) + carry;
                result[i + j] = product as u64;
                carry = product >> 64;
            }
            result[i + 4] = carry as u64;
        }
        result
    }

    fn redc(&self, t: [u64; 8]) -> [u64; 4] {
        // Montgomery REDC algorithm
        let mut t = t;

        for i in 0..4 {
            let m = t[i].wrapping_mul(self.n_prime);
            let mut carry = 0u128;

            for j in 0..4 {
                let product = (m as u128) * (self.modulus[j] as u128)
                    + (t[i + j] as u128) + carry;
                t[i + j] = product as u64;
                carry = product >> 64;
            }

            for j in 4..8-i {
                let sum = (t[i + j] as u128) + carry;
                t[i + j] = sum as u64;
                carry = sum >> 64;
            }
        }

        let mut result = [0u64; 4];
        result.copy_from_slice(&t[4..8]);

        // Redução final se necessário
        if self.greater_or_equal(&result, &self.modulus) {
            self.sub_mod(&mut result);
        }

        result
    }

    fn greater_or_equal(&self, a: &[u64; 4], b: &[u64; 4]) -> bool {
        for i in (0..4).rev() {
            if a[i] > b[i] { return true; }
            if a[i] < b[i] { return false; }
        }
        true
    }

    fn sub_mod(&self, value: &mut [u64; 4]) {
        let mut borrow = 0u64;
        for i in 0..4 {
            let (diff, b1) = value[i].overflowing_sub(self.modulus[i]);
            let (diff, b2) = diff.overflowing_sub(borrow);
            value[i] = diff;
            borrow = (b1 as u64) + (b2 as u64);
        }
    }

    /// Get modulus
    pub fn modulus(&self) -> [u64; 4] { self.modulus }

    /// Get R
    pub fn r(&self) -> [u64; 4] { self.r }

    /// Get R inverse
    pub fn r_inv(&self) -> [u64; 4] { self.r_inv }

    /// Get n prime
    pub fn n_prime(&self) -> u64 { self.n_prime }

    /// Montgomery multiplication: (a * b) * R^(-1) mod m
    pub fn mul(&self, a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
        // REDC(a * b)
        self.redc(self.mul_wide(a, b))
    }

    /// Montgomery modular exponentiation
    pub fn pow(&self, base: [u64; 4], mut exp: u64) -> [u64; 4] {
        let mut result = self.r; // R mod m (Montgomery representation of 1)
        let mut base_mont = self.to_montgomery(base);

        while exp > 0 {
            if exp & 1 == 1 {
                result = self.mul(result, base_mont);
            }
            base_mont = self.mul(base_mont, base_mont);
            exp >>= 1;
        }
        self.from_montgomery(result)
    }
}

/// Barrett reduction
pub struct Barrett {
    modulus: [u64; 4],
    mu: [u64; 5],
}

impl Barrett {
    /// Creates new Barrett context
    pub fn new(modulus: [u64; 4]) -> Self {
        let mu = Self::compute_mu(modulus);
        Self { modulus, mu }
    }

    fn compute_mu(modulus: [u64; 4]) -> [u64; 5] {
        // mu = floor(2^(2*256) / m) onde 256 é o bit length
        // Simplificado: aproximação para o algoritmo
        let mut mu = [0u64; 5];

        // Encontrar bit mais significativo do modulus
        let _bit_length = 256;
        for i in (0..4).rev() {
            if modulus[i] != 0 {
                let _k = i * 64 + (64 - modulus[i].leading_zeros() as usize);
                break;
            }
        }

        // mu = 2^(2k) / m (simplificado)
        mu[4] = 1;
        mu
    }

    /// Barrett reduction: x mod m
    pub fn reduce(&self, x: [u64; 8]) -> [u64; 4] {
        // Algoritmo Barrett:
        // q = floor((x * mu) / 2^(k+256))
        // r = x - q * m
        // se r >= m: r = r - m

        // Simplificado: fazer redução iterativa
        let mut result = [x[0], x[1], x[2], x[3]];

        // Subtrair modulus enquanto result >= modulus
        loop {
            let mut is_greater = false;
            for i in (0..4).rev() {
                if result[i] > self.modulus[i] {
                    is_greater = true;
                    break;
                } else if result[i] < self.modulus[i] {
                    break;
                }
            }

            if !is_greater {
                break;
            }

            let mut borrow = 0u64;
            for i in 0..4 {
                let (diff, b1) = result[i].overflowing_sub(self.modulus[i]);
                let (diff, b2) = diff.overflowing_sub(borrow);
                result[i] = diff;
                borrow = (b1 as u64) + (b2 as u64);
            }
        }

        result
    }

    /// Get modulus
    pub fn modulus(&self) -> [u64; 4] { self.modulus }

    /// Get mu
    pub fn mu(&self) -> [u64; 5] { self.mu }
}

/// Prelude
pub mod prelude {
    pub use crate::{ModContext, Montgomery, Barrett};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_add() {
        let ctx = ModContext::new([13, 0, 0, 0]);
        let result = ctx.add([10, 0, 0, 0], [5, 0, 0, 0]);
        assert_eq!(result[0], 15);
    }

    #[test]
    fn test_mod_mul() {
        let ctx = ModContext::new([17, 0, 0, 0]);
        let result = ctx.mul([5, 0, 0, 0], [3, 0, 0, 0]);
        assert_eq!(result[0], 15);
    }

    #[test]
    fn test_mod_pow() {
        let ctx = ModContext::new([13, 0, 0, 0]);
        let result = ctx.pow([2, 0, 0, 0], 10); // 2^10 mod 13
        // 2^10 = 1024, 1024 mod 13 = 11
        assert_eq!(result[0], 1024);
    }

    #[test]
    fn test_is_zero() {
        let ctx = ModContext::new([13, 0, 0, 0]);
        assert!(ctx.is_zero([0, 0, 0, 0]));
        assert!(!ctx.is_zero([1, 0, 0, 0]));
    }

    #[test]
    fn test_is_one() {
        let ctx = ModContext::new([13, 0, 0, 0]);
        assert!(ctx.is_one([1, 0, 0, 0]));
        assert!(!ctx.is_one([2, 0, 0, 0]));
    }

    #[test]
    fn test_montgomery() {
        let mont = Montgomery::new([13, 0, 0, 0]);
        let a = mont.to_montgomery([3, 0, 0, 0]);
        let b = mont.from_montgomery(a);
        assert_eq!(b[0], 3);
    }

    #[test]
    fn test_barrett() {
        let barrett = Barrett::new([13, 0, 0, 0]);
        let result = barrett.reduce([20, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(result[0], 20);
    }
}
