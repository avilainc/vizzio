//! Integration tests for cryptographic operations

use avila_bignum::crypto::{modular, prime};

#[test]
fn test_modular_addition() {
    let a = [8u64];
    let b = [7u64];
    let m = [10u64];
    let mut result = [0u64];
    
    modular::mod_add(&a, &b, &m, &mut result);
    assert_eq!(result[0], 5); // (8 + 7) mod 10 = 5
}

#[test]
fn test_modular_subtraction() {
    let a = [3u64];
    let b = [8u64];
    let m = [10u64];
    let mut result = [0u64];
    
    modular::mod_sub(&a, &b, &m, &mut result);
    assert_eq!(result[0], 5); // (3 - 8) mod 10 = 5
}

#[test]
fn test_modular_multiplication() {
    let a = [7u64];
    let b = [8u64];
    let m = [10u64];
    let mut result = [0u64; 2];
    
    modular::mod_mul_simple(&a, &b, &m, &mut result);
    assert_eq!(result[0], 6); // (7 * 8) mod 10 = 6
}

#[test]
fn test_modular_exponentiation() {
    let base = [2u64];
    let exp = [10u64];
    let m = [1000u64];
    let mut result = [0u64];
    
    modular::mod_pow(&base, &exp, &m, &mut result);
    assert_eq!(result[0], 24); // 2^10 mod 1000 = 1024 mod 1000 = 24
}

#[test]
fn test_gcd() {
    let a = [48u64];
    let b = [18u64];
    let mut result = [0u64];
    
    prime::gcd(&a, &b, &mut result);
    assert_eq!(result[0], 6);
}

#[test]
fn test_gcd_coprime() {
    let a = [17u64];
    let b = [19u64];
    let mut result = [0u64];
    
    prime::gcd(&a, &b, &mut result);
    assert_eq!(result[0], 1); // Coprimes
}

#[test]
fn test_is_even_odd() {
    assert!(prime::is_even(&[42u64]));
    assert!(!prime::is_even(&[43u64]));
    assert!(prime::is_odd(&[43u64]));
    assert!(!prime::is_odd(&[42u64]));
}

#[test]
fn test_trial_division() {
    // Primes should pass
    assert!(prime::trial_division(&[17u64]));
    assert!(prime::trial_division(&[97u64]));
    
    // Composites should fail
    assert!(!prime::trial_division(&[15u64])); // 3 * 5
    assert!(!prime::trial_division(&[21u64])); // 3 * 7
}

#[test]
fn test_is_prime_small() {
    // Small primes
    assert!(prime::is_prime_miller_rabin(&[2u64], 20));
    assert!(prime::is_prime_miller_rabin(&[3u64], 20));
    assert!(prime::is_prime_miller_rabin(&[5u64], 20));
    assert!(prime::is_prime_miller_rabin(&[7u64], 20));
    assert!(prime::is_prime_miller_rabin(&[11u64], 20));
    assert!(prime::is_prime_miller_rabin(&[17u64], 20));
    
    // Composites
    assert!(!prime::is_prime_miller_rabin(&[4u64], 20));
    assert!(!prime::is_prime_miller_rabin(&[6u64], 20));
    assert!(!prime::is_prime_miller_rabin(&[8u64], 20));
    assert!(!prime::is_prime_miller_rabin(&[9u64], 20));
}

#[test]
fn test_crypto_edge_cases() {
    // Zero cases
    let zero = [0u64];
    let five = [5u64];
    let mut result = [0u64];
    
    prime::gcd(&zero, &five, &mut result);
    assert_eq!(result[0], 5);
    
    prime::gcd(&five, &zero, &mut result);
    assert_eq!(result[0], 5);
}

#[test]
fn test_modular_with_large_numbers() {
    let a = [999999u64];
    let b = [111111u64];
    let m = [100000u64];
    let mut result = [0u64; 2];
    
    modular::mod_add(&a, &b, &m, &mut result);
    // (999999 + 111111) mod 100000 = 1111110 mod 100000 = 11110
    assert_eq!(result[0], 11110);
}

