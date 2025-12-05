//! Demonstração de operações criptográficas implementadas

fn main() {
    println!("=== Avila BigNum - Demonstração de Criptografia ===\n");

    // Operações modulares básicas
    println!("--- Aritmética Modular ---");
    
    // Exemplo: (5 + 7) mod 10 = 2
    let a = [5u64];
    let b = [7u64];
    let m = [10u64];
    let mut result = [0u64];
    
    avila_bignum::crypto::modular::mod_add(&a, &b, &m, &mut result);
    println!("(5 + 7) mod 10 = {}", result[0]);
    
    // Exemplo: (3 - 5) mod 10 = 8
    let a2 = [3u64];
    let b2 = [5u64];
    avila_bignum::crypto::modular::mod_sub(&a2, &b2, &m, &mut result);
    println!("(3 - 5) mod 10 = {}", result[0]);
    
    // Exemplo: (6 * 7) mod 10 = 2
    let a3 = [6u64];
    let b3 = [7u64];
    let mut result_mul = [0u64; 2];
    avila_bignum::crypto::modular::mod_mul_simple(&a3, &b3, &m, &mut result_mul);
    println!("(6 * 7) mod 10 = {}", result_mul[0]);
    
    // Exponenciação modular: 3^4 mod 10 = 81 mod 10 = 1
    println!("\n--- Exponenciação Modular ---");
    let base = [3u64];
    let exp = [4u64];
    let modulus = [10u64];
    let mut result_pow = [0u64];
    avila_bignum::crypto::modular::mod_pow(&base, &exp, &modulus, &mut result_pow);
    println!("3^4 mod 10 = {}", result_pow[0]);
    
    // Outro exemplo: 2^10 mod 1000
    let base2 = [2u64];
    let exp2 = [10u64];
    let mod2 = [1000u64];
    let mut result_pow2 = [0u64];
    avila_bignum::crypto::modular::mod_pow(&base2, &exp2, &mod2, &mut result_pow2);
    println!("2^10 mod 1000 = {} (esperado: 24)", result_pow2[0]);
    
    // Teoria dos números
    println!("\n--- Teoria dos Números ---");
    
    // GCD (Greatest Common Divisor)
    let num1 = [48u64];
    let num2 = [18u64];
    let mut gcd_result = [0u64];
    avila_bignum::crypto::prime::gcd(&num1, &num2, &mut gcd_result);
    println!("GCD(48, 18) = {}", gcd_result[0]);
    
    let num3 = [1071u64];
    let num4 = [462u64];
    avila_bignum::crypto::prime::gcd(&num3, &num4, &mut gcd_result);
    println!("GCD(1071, 462) = {}", gcd_result[0]);
    
    // Testes de primalidade
    println!("\n--- Testes de Primalidade ---");
    
    let primes = [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    let composites = [4u64, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20];
    
    print!("Números primos: ");
    for p in &primes {
        if avila_bignum::crypto::prime::is_prime_miller_rabin(&[*p], 20) {
            print!("{} ", p);
        }
    }
    println!();
    
    print!("Números compostos (devem falhar): ");
    for c in &composites {
        if !avila_bignum::crypto::prime::is_prime_miller_rabin(&[*c], 20) {
            print!("{} ", c);
        }
    }
    println!();
    
    // Testes par/ímpar
    println!("\n--- Par/Ímpar ---");
    println!("42 é par? {}", avila_bignum::crypto::prime::is_even(&[42u64]));
    println!("43 é ímpar? {}", avila_bignum::crypto::prime::is_odd(&[43u64]));
    
    println!("\n=== Operações Criptográficas Fundamentais Implementadas! ===");
    println!("Ready para RSA e outras aplicações criptográficas! 🔐");
}
