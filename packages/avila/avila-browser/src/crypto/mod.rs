//! Cryptographic primitives module
//!
//! Provides high-level cryptographic operations including:
//! - Encryption/Decryption (AES-256-GCM, ChaCha20-Poly1305)
//! - Digital signatures (Ed25519, RSA)
//! - Hashing (SHA3, BLAKE3)
//! - Key exchange (X25519, Kyber for post-quantum)

pub mod encryption;
pub mod signing;
pub mod hashing;
pub mod key_exchange;
pub mod pqc;

pub use encryption::{Cipher, EncryptionError};
pub use signing::{Signer, SignatureError};
pub use hashing::{Hasher, HashAlgorithm};
pub use key_exchange::{KeyExchange, KeyExchangeError};

/// Security level for cryptographic operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    /// 128-bit security
    Standard,
    /// 256-bit security
    High,
    /// Post-quantum resistant
    PostQuantum,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_levels() {
        assert_ne!(SecurityLevel::Standard, SecurityLevel::High);
    }
}
