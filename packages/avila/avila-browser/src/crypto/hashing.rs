//! Cryptographic hashing functions

use blake3::Hasher as Blake3HasherImpl;
use sha2::{Digest, Sha256};
use sha3::Sha3_256;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    Sha256,
    Sha3_256,
    Blake3,
}

#[derive(Debug)]
pub enum HashError {
    InvalidInput,
    HashFailed,
}

impl fmt::Display for HashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput => write!(f, "Invalid input for hashing"),
            Self::HashFailed => write!(f, "Hashing operation failed"),
        }
    }
}

impl Error for HashError {}

/// Hasher interface
pub trait Hasher {
    /// Hash input data
    fn hash(&self, data: &[u8]) -> Result<Vec<u8>, HashError>;

    /// Get the output size in bytes
    fn output_size(&self) -> usize;
}

/// SHA-256 hasher
pub struct Sha256Hasher;

impl Hasher for Sha256Hasher {
    fn hash(&self, data: &[u8]) -> Result<Vec<u8>, HashError> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        Ok(hasher.finalize().to_vec())
    }

    fn output_size(&self) -> usize {
        32
    }
}

/// SHA3-256 hasher
pub struct Sha3_256Hasher;

impl Hasher for Sha3_256Hasher {
    fn hash(&self, data: &[u8]) -> Result<Vec<u8>, HashError> {
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        Ok(hasher.finalize().to_vec())
    }

    fn output_size(&self) -> usize {
        32
    }
}

/// BLAKE3 hasher
pub struct Blake3Hasher;

impl Hasher for Blake3Hasher {
    fn hash(&self, data: &[u8]) -> Result<Vec<u8>, HashError> {
        let hash = blake3::hash(data);
        Ok(hash.as_bytes().to_vec())
    }

    fn output_size(&self) -> usize {
        32
    }
}

/// Create a hasher for the given algorithm
pub fn create_hasher(algorithm: HashAlgorithm) -> Box<dyn Hasher> {
    match algorithm {
        HashAlgorithm::Sha256 => Box::new(Sha256Hasher),
        HashAlgorithm::Sha3_256 => Box::new(Sha3_256Hasher),
        HashAlgorithm::Blake3 => Box::new(Blake3Hasher),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hash() {
        let hasher = Sha256Hasher;
        let data = b"test data";
        let hash = hasher.hash(data).unwrap();

        assert_eq!(hash.len(), 32);

        // Same input should produce same hash
        let hash2 = hasher.hash(data).unwrap();
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_sha3_256_hash() {
        let hasher = Sha3_256Hasher;
        let data = b"SHA3 test";
        let hash = hasher.hash(data).unwrap();

        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_blake3_hash() {
        let hasher = Blake3Hasher;
        let data = b"BLAKE3 is fast";
        let hash = hasher.hash(data).unwrap();

        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_different_inputs_different_hashes() {
        let hasher = Blake3Hasher;
        let hash1 = hasher.hash(b"input1").unwrap();
        let hash2 = hasher.hash(b"input2").unwrap();

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hash_empty_input() {
        let hasher = Sha256Hasher;
        let hash = hasher.hash(&[]).unwrap();

        assert_eq!(hash.len(), 32);
        // SHA-256 of empty string is known
        assert_eq!(
            hex::encode(hash),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_create_hasher() {
        let sha256 = create_hasher(HashAlgorithm::Sha256);
        assert_eq!(sha256.output_size(), 32);

        let blake3 = create_hasher(HashAlgorithm::Blake3);
        assert_eq!(blake3.output_size(), 32);
    }
}

// Helper for tests
#[cfg(test)]
mod hex {
    pub fn encode(bytes: Vec<u8>) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}
