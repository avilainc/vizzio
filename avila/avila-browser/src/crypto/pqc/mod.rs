//! Post-Quantum Cryptography module
//!
//! Implements NIST-approved post-quantum algorithms:
//! - Kyber: Key encapsulation mechanism (KEM)
//! - Dilithium: Digital signatures

pub mod kyber;
pub mod dilithium;

pub use kyber::KyberKem;
pub use dilithium::DilithiumSigner;

/// Post-quantum security level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PqSecurityLevel {
    /// NIST Level 1 (~128-bit classical security)
    Level1,
    /// NIST Level 3 (~192-bit classical security)
    Level3,
    /// NIST Level 5 (~256-bit classical security)
    Level5,
}
