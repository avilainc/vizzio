//! Prime number generation and testing
//!
//! Implements primality tests and number theory functions

use crate::arithmetic::{cmp, sub, is_zero};
use core::cmp::Ordering;

/// Calculate GCD using Binary GCD (Stein's algorithm)
/// This is an iterative, efficient algorithm that uses shifts instead of division
pub fn gcd(a: &[u64], b: &[u64], result: &mut [u64]) {
    let n = a.len();
    let mut u = a.to_vec();
    let mut v = b.to_vec();

    // Handle zero cases
    if is_zero(&u) {
        result.copy_from_slice(&v);
        return;
    }
    if is_zero(&v) {
        result.copy_from_slice(&u);
        return;
    }

    // Remove common factors of 2
    let mut shift = 0u32;
    while (u[0] & 1) == 0 && (v[0] & 1) == 0 {
        // Both even, divide by 2
        for i in 0..n-1 {
            u[i] = (u[i] >> 1) | (u[i+1] << 63);
            v[i] = (v[i] >> 1) | (v[i+1] << 63);
        }
        u[n-1] >>= 1;
        v[n-1] >>= 1;
        shift += 1;
    }

    // Remove remaining factors of 2 from u
    while (u[0] & 1) == 0 {
        for i in 0..n-1 {
            u[i] = (u[i] >> 1) | (u[i+1] << 63);
        }
        u[n-1] >>= 1;
    }

    loop {
        // Remove factors of 2 from v
        while (v[0] & 1) == 0 {
            for i in 0..n-1 {
                v[i] = (v[i] >> 1) | (v[i+1] << 63);
            }
            v[n-1] >>= 1;
        }

        // Ensure u <= v
        if matches!(cmp(&u, &v), Ordering::Greater) {
            core::mem::swap(&mut u, &mut v);
        }

        // v = v - u
        sub(&v, &u, &mut v);

        // Check if done
        if is_zero(&v) {
            break;
        }
    }

    // Multiply result by 2^shift
    result.copy_from_slice(&u);
    for _ in 0..shift {
        let mut carry = 0u64;
        for i in 0..n {
            let val = (result[i] << 1) | carry;
            carry = result[i] >> 63;
            result[i] = val;
        }
    }
}

/// Check if a number is even
pub fn is_even(n: &[u64]) -> bool {
    (n[0] & 1) == 0
}

/// Check if a number is odd
pub fn is_odd(n: &[u64]) -> bool {
    (n[0] & 1) == 1
}

/// Simple trial division for small primes
pub fn trial_division(n: &[u64]) -> bool {
    const SMALL_PRIMES: [u64; 54] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
        73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151,
        157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233,
        239, 241, 251,
    ];

    // Check against small primes
    for &p in &SMALL_PRIMES {
        // Simple modulo check using the first limb if n is small
        if n[0] < p * p && n[1..].iter().all(|&x| x == 0) {
            return n[0] == p;
        }

        // Check if divisible by p
        let mut remainder = 0u64;
        for &limb in n.iter().rev() {
            let dividend = ((remainder as u128) << 64) | (limb as u128);
            remainder = (dividend % p as u128) as u64;
        }

        if remainder == 0 {
            return false; // Composite
        }
    }

    true // Passed trial division (probably prime)
}

/// Miller-Rabin primality test (simplified version)
/// Returns true if n is probably prime
pub fn is_prime_miller_rabin(n: &[u64], rounds: u32) -> bool {
    // Handle small cases
    if is_zero(n) || (n[0] == 1 && n[1..].iter().all(|&x| x == 0)) {
        return false;
    }
    if n[0] == 2 && n[1..].iter().all(|&x| x == 0) {
        return true;
    }
    if is_even(n) {
        return false;
    }

    // Trial division first
    if !trial_division(n) {
        return false;
    }

    // For now, if it passes trial division, consider it probably prime
    // Full Miller-Rabin would require modular exponentiation
    // which we'll implement when we have better modular arithmetic
    true
}

/// Generate a random prime number of specified bit length
pub fn generate_prime(_bits: usize) -> Vec<u64> {
    // TODO: Implement prime generation with proper RNG
    // For now, return a known small prime
    vec![17u64]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_simple() {
        let a = [48u64];
        let b = [18u64];
        let mut result = [0u64];
        gcd(&a, &b, &mut result);
        assert_eq!(result[0], 6);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(&[4u64]));
        assert!(!is_even(&[5u64]));
    }

    #[test]
    fn test_trial_division() {
        assert!(trial_division(&[17u64])); // Prime
        assert!(!trial_division(&[15u64])); // Composite (3 * 5)
    }

    #[test]
    fn test_is_prime_small() {
        assert!(is_prime_miller_rabin(&[17u64], 20));
        assert!(is_prime_miller_rabin(&[97u64], 20));
        assert!(!is_prime_miller_rabin(&[15u64], 20));
    }
}
