//! Cryptographic operations module
//!
//! This module provides cryptographic primitives including:
//! - Modular arithmetic (addition, multiplication, exponentiation)
//! - Number theory functions (GCD, primality testing)
//! - RSA key generation and operations

pub mod modular;
pub mod prime;
pub mod rsa;
pub mod montgomery;

pub use modular::*;
pub use prime::*;
pub use rsa::*;
pub use montgomery::*;
