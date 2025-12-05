//! Modular arithmetic operations
//!
//! Provides modular operations for cryptographic use

use crate::arithmetic::{add, sub, cmp};
use core::cmp::Ordering;

/// Modular addition: (a + b) mod m
pub fn mod_add(a: &[u64], b: &[u64], modulus: &[u64], result: &mut [u64]) {
    // First add a + b
    add(a, b, result);

    // If result >= modulus, subtract modulus
    if matches!(cmp(result, modulus), Ordering::Greater | Ordering::Equal) {
        let mut temp = result.to_vec();
        sub(&temp, modulus, result);
    }
}

/// Modular subtraction: (a - b) mod m
pub fn mod_sub(a: &[u64], b: &[u64], modulus: &[u64], result: &mut [u64]) {
    // If a >= b, just subtract
    if matches!(cmp(a, b), Ordering::Greater | Ordering::Equal) {
        sub(a, b, result);
    } else {
        // If a < b, compute (a + modulus) - b
        let mut temp = vec![0u64; modulus.len()];
        add(a, modulus, &mut temp);
        sub(&temp, b, result);
    }
}

/// Modular multiplication: (a * b) mod m (simple schoolbook)
pub fn mod_mul_simple(a: &[u64], b: &[u64], modulus: &[u64], result: &mut [u64]) {
    let n = modulus.len();
    result.fill(0);

    for i in 0..n {
        if b[i] == 0 {
            continue;
        }

        let mut carry = 0u64;
        for j in 0..n {
            let product = (a[j] as u128) * (b[i] as u128)
                + (result[i + j] as u128)
                + (carry as u128);
            result[i + j] = product as u64;
            carry = (product >> 64) as u64;
        }
    }

    // Reduce modulo m (simple repeated subtraction for now)
    while matches!(cmp(result, modulus), Ordering::Greater | Ordering::Equal) {
        let mut temp = result.to_vec();
        sub(&temp, modulus, result);
    }
}

/// Modular exponentiation: (base^exp) mod m
/// Uses square-and-multiply algorithm
pub fn mod_pow(base: &[u64], exp: &[u64], modulus: &[u64], result: &mut [u64]) {
    let n = modulus.len();

    // Initialize result to 1
    result.fill(0);
    result[0] = 1;

    // Create a copy of base reduced modulo m
    let mut base_copy = base.to_vec();
    while matches!(cmp(&base_copy, modulus), Ordering::Greater | Ordering::Equal) {
        let temp = base_copy.clone();
        sub(&temp, modulus, &mut base_copy);
    }

    // Process each bit of the exponent
    for i in 0..n {
        let mut exp_limb = exp[i];
        for _ in 0..64 {
            if exp_limb & 1 == 1 {
                // result = (result * base) mod m
                let temp = result.to_vec();
                mod_mul_simple(&temp, &base_copy, modulus, result);
            }

            // base = (base * base) mod m
            let temp = base_copy.clone();
            mod_mul_simple(&temp, &temp, modulus, &mut base_copy);

            exp_limb >>= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_add() {
        let a = [5u64];
        let b = [7u64];
        let m = [10u64];
        let mut result = [0u64];
        mod_add(&a, &b, &m, &mut result);
        assert_eq!(result[0], 2); // (5 + 7) mod 10 = 2
    }

    #[test]
    fn test_mod_sub() {
        let a = [3u64];
        let b = [5u64];
        let m = [10u64];
        let mut result = [0u64];
        mod_sub(&a, &b, &m, &mut result);
        assert_eq!(result[0], 8); // (3 - 5) mod 10 = 8
    }

    #[test]
    fn test_mod_mul_simple() {
        let a = [6u64];
        let b = [7u64];
        let m = [10u64];
        let mut result = [0u64; 2];
        mod_mul_simple(&a, &b, &m, &mut result);
        assert_eq!(result[0], 2); // (6 * 7) mod 10 = 2
    }

    #[test]
    fn test_mod_pow_simple() {
        let base = [3u64];
        let exp = [4u64];  // 3^4 = 81
        let m = [10u64];
        let mut result = [0u64];
        mod_pow(&base, &exp, &m, &mut result);
        assert_eq!(result[0], 1); // 81 mod 10 = 1
    }
}
