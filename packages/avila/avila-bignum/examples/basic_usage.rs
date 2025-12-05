//! Basic usage example for avila-bignum

use avila_bignum::{U1024, U2048};

fn main() {
    println!("=== avila-bignum Basic Usage ===\n");

    // Creating numbers
    let a = U1024::from(42u64);
    let b = U1024::from(100u64);

    println!("a = {:?}", a);
    println!("b = {:?}", b);

    // Addition
    let sum = a + b;
    println!("\na + b = {:?}", sum);

    // Constants
    println!("\nConstants:");
    println!("ZERO = {:?}", U1024::ZERO);
    println!("ONE  = {:?}", U1024::ONE);
    println!("MAX  = {:?}", U1024::MAX);

    // Different sizes
    println!("\nDifferent sizes:");
    let small = U1024::from(123u64);
    let large = U2048::from(456u64);
    println!("U1024: {:?}", small);
    println!("U2048: {:?}", large);
}
