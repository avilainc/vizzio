//! Demonstração de operações aritméticas implementadas

use avila_bignum::{U1024, U2048};

fn main() {
    println!("=== Avila BigNum - Demonstração de Aritmética ===\n");

    // Operações básicas U1024
    println!("--- U1024 (1024 bits) ---");
    let a = U1024::from(12345u64);
    let b = U1024::from(67890u64);

    println!("a = {}", a.limbs[0]);
    println!("b = {}", b.limbs[0]);

    let sum = a + b;
    println!("a + b = {}", sum.limbs[0]);

    let diff = b - a;
    println!("b - a = {}", diff.limbs[0]);

    let prod = a * b;
    println!("a * b = {}", prod.limbs[0]);

    // Operações bitwise
    println!("\n--- Operações Bitwise ---");
    let x = U1024::from(0b11001100u64);
    let y = U1024::from(0b10101010u64);

    println!("x     = {:08b}", x.limbs[0]);
    println!("y     = {:08b}", y.limbs[0]);
    println!("x & y = {:08b}", (x & y).limbs[0]);
    println!("x | y = {:08b}", (x | y).limbs[0]);
    println!("x ^ y = {:08b}", (x ^ y).limbs[0]);

    // Shifts
    println!("\n--- Operações de Shift ---");
    let val = U1024::from(1u64);
    println!("valor inicial = {}", val.limbs[0]);

    let shifted = val << 10;
    println!("valor << 10   = {}", shifted.limbs[0]);

    let back = shifted >> 5;
    println!("valor >> 5    = {}", back.limbs[0]);

    // Comparações
    println!("\n--- Comparações ---");
    let m = U1024::from(100u64);
    let n = U1024::from(200u64);

    println!("m = {}, n = {}", m.limbs[0], n.limbs[0]);
    println!("m < n  ? {}", m < n);
    println!("m > n  ? {}", m > n);
    println!("m == m ? {}", m == m);
    println!("m <= n ? {}", m <= n);

    // U2048
    println!("\n--- U2048 (2048 bits - RSA) ---");
    let big_a = U2048::from(999999u64);
    let big_b = U2048::from(111111u64);

    println!("a = {}", big_a.limbs[0]);
    println!("b = {}", big_b.limbs[0]);
    println!("a + b = {}", (big_a + big_b).limbs[0]);
    println!("a - b = {}", (big_a - big_b).limbs[0]);
    println!("a * b = {}", (big_a * big_b).limbs[0]);

    // Constantes
    println!("\n--- Constantes ---");
    println!("U1024::ZERO  = {}", U1024::ZERO.limbs[0]);
    println!("U1024::ONE   = {}", U1024::ONE.limbs[0]);
    println!("U1024::BITS  = {}", U1024::BITS);
    println!("U1024::LIMBS = {}", U1024::LIMBS);

    // Contagem de zeros
    println!("\n--- Operações de Bits ---");
    let test = U1024::from(0x8000000000000000u64); // Bit mais alto do primeiro limb
    println!("Valor: 0x{:016x}", test.limbs[0]);
    println!("Leading zeros:  {}", test.leading_zeros());
    println!("Trailing zeros: {}", test.trailing_zeros());

    let test2 = U1024::from(1u64);
    println!("\nValor: {}", test2.limbs[0]);
    println!("Leading zeros:  {}", test2.leading_zeros());
    println!("Trailing zeros: {}", test2.trailing_zeros());

    println!("\n=== Implementação 100% Rust Puro - Sem Dependências! ===");
}
