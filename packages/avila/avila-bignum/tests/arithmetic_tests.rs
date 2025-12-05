//! Integration tests for basic arithmetic operations

use avila_bignum::{U1024, U2048, U4096};

#[test]
fn test_u1024_addition() {
    let a = U1024::from(100u64);
    let b = U1024::from(200u64);
    let c = a + b;
    assert_eq!(c, U1024::from(300u64));
}

#[test]
fn test_u1024_subtraction() {
    let a = U1024::from(500u64);
    let b = U1024::from(200u64);
    let c = a - b;
    assert_eq!(c, U1024::from(300u64));
}

#[test]
fn test_u1024_multiplication() {
    let a = U1024::from(25u64);
    let b = U1024::from(4u64);
    let c = a * b;
    assert_eq!(c, U1024::from(100u64));
}

#[test]
fn test_u1024_bitwise() {
    let a = U1024::from(0b11110000u64);
    let b = U1024::from(0b10101010u64);
    
    assert_eq!(a & b, U1024::from(0b10100000u64));
    assert_eq!(a | b, U1024::from(0b11111010u64));
    assert_eq!(a ^ b, U1024::from(0b01011010u64));
}

#[test]
fn test_u1024_shifts() {
    let val = U1024::from(1u64);
    assert_eq!(val << 8, U1024::from(256u64));
    assert_eq!(U1024::from(256u64) >> 4, U1024::from(16u64));
}

#[test]
fn test_u1024_comparison() {
    let a = U1024::from(100u64);
    let b = U1024::from(200u64);
    
    assert!(a < b);
    assert!(b > a);
    assert!(a <= b);
    assert!(b >= a);
    assert_eq!(a, a);
    assert_ne!(a, b);
}

#[test]
fn test_u2048_operations() {
    let a = U2048::from(1000u64);
    let b = U2048::from(500u64);
    
    assert_eq!(a + b, U2048::from(1500u64));
    assert_eq!(a - b, U2048::from(500u64));
    assert_eq!(a * b, U2048::from(500000u64));
}

#[test]
fn test_u4096_operations() {
    let a = U4096::from(9999u64);
    let b = U4096::from(1111u64);
    
    assert_eq!(a + b, U4096::from(11110u64));
    assert!(a > b);
}

#[test]
fn test_u4096_div_rem() {
    let a = U4096::from(100u64);
    let (quotient, remainder) = a.div_rem_u64(7);
    assert_eq!(quotient, U4096::from(14u64));
    assert_eq!(remainder, 2);
}

#[test]
fn test_constants() {
    assert_eq!(U1024::ZERO + U1024::ONE, U1024::ONE);
    assert_eq!(U1024::ONE + U1024::ONE, U1024::from(2u64));
    assert!(U1024::ZERO.is_zero());
    assert!(!U1024::ONE.is_zero());
}

#[test]
fn test_default() {
    let zero: U1024 = Default::default();
    assert_eq!(zero, U1024::ZERO);
}

#[test]
fn test_leading_trailing_zeros() {
    let one = U1024::from(1u64);
    assert_eq!(one.trailing_zeros(), 0);
    assert_eq!(one.leading_zeros(), 1023);
    
    let eight = U1024::from(8u64);
    assert_eq!(eight.trailing_zeros(), 3);
}

#[test]
fn test_large_multiplication() {
    let a = U1024::from(123456789u64);
    let b = U1024::from(987654321u64);
    let c = a * b;
    // 123456789 * 987654321 = 121932631112635269
    assert_eq!(c.limbs[0], 121932631112635269u64);
}

#[test]
fn test_overflow_behavior() {
    let max = U1024::MAX;
    let one = U1024::ONE;
    let result = max + one; // Should wrap
    assert_eq!(result, U1024::ZERO);
}

