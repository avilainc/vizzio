//! Digital signature operations

use ed25519_dalek::{Signature, Signer as DalekSigner, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SignatureError {
    InvalidKey,
    SigningFailed,
    VerificationFailed,
    InvalidSignature,
}

impl fmt::Display for SignatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidKey => write!(f, "Invalid signing key"),
            Self::SigningFailed => write!(f, "Signature generation failed"),
            Self::VerificationFailed => write!(f, "Signature verification failed"),
            Self::InvalidSignature => write!(f, "Invalid signature format"),
        }
    }
}

impl Error for SignatureError {}

/// Signer interface for digital signatures
pub trait Signer {
    /// Sign a message
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, SignatureError>;

    /// Verify a signature
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, SignatureError>;
}

/// Ed25519 digital signature
pub struct Ed25519Signer {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl Ed25519Signer {
    /// Create Ed25519 signer from existing keys
    pub fn from_keys(signing_key: SigningKey) -> Self {
        let verifying_key = signing_key.verifying_key();
        Self {
            signing_key,
            verifying_key,
        }
    }

    /// Generate a new Ed25519 keypair
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        Self {
            signing_key,
            verifying_key,
        }
    }

    /// Get the public key bytes
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.verifying_key.to_bytes()
    }

    /// Get the secret key bytes
    pub fn secret_key_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }

    /// Create from secret key bytes
    pub fn from_bytes(secret_bytes: &[u8]) -> Result<Self, SignatureError> {
        if secret_bytes.len() != 32 {
            return Err(SignatureError::InvalidKey);
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(secret_bytes);

        let signing_key = SigningKey::from_bytes(&bytes);
        Ok(Self::from_keys(signing_key))
    }
}

impl Signer for Ed25519Signer {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, SignatureError> {
        let signature = self.signing_key.sign(message);
        Ok(signature.to_bytes().to_vec())
    }

    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, SignatureError> {
        if signature.len() != 64 {
            return Err(SignatureError::InvalidSignature);
        }

        let sig = Signature::from_slice(signature)
            .map_err(|_| SignatureError::InvalidSignature)?;

        match self.verifying_key.verify(message, &sig) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519_sign_verify() {
        let signer = Ed25519Signer::generate();
        let message = b"Test message for Ed25519 signature";

        let signature = signer.sign(message).unwrap();
        assert_eq!(signature.len(), 64);

        let valid = signer.verify(message, &signature).unwrap();
        assert!(valid);
    }

    #[test]
    fn test_ed25519_wrong_message_fails() {
        let signer = Ed25519Signer::generate();
        let message = b"Original message";
        let wrong_message = b"Wrong message";

        let signature = signer.sign(message).unwrap();
        let valid = signer.verify(wrong_message, &signature).unwrap();
        assert!(!valid);
    }

    #[test]
    fn test_ed25519_tampered_signature_fails() {
        let signer = Ed25519Signer::generate();
        let message = b"Important data";

        let mut signature = signer.sign(message).unwrap();
        signature[0] ^= 0xFF; // Tamper with signature

        let valid = signer.verify(message, &signature).unwrap();
        assert!(!valid);
    }

    #[test]
    fn test_ed25519_from_bytes() {
        let signer1 = Ed25519Signer::generate();
        let secret_bytes = signer1.secret_key_bytes();

        let signer2 = Ed25519Signer::from_bytes(&secret_bytes).unwrap();
        assert_eq!(signer1.public_key_bytes(), signer2.public_key_bytes());

        let message = b"test";
        let sig1 = signer1.sign(message).unwrap();
        let valid = signer2.verify(message, &sig1).unwrap();
        assert!(valid);
    }

    #[test]
    fn test_ed25519_invalid_signature_length() {
        let signer = Ed25519Signer::generate();
        let message = b"test";
        let short_sig = vec![0u8; 32];

        let result = signer.verify(message, &short_sig);
        assert!(result.is_err());
    }

    #[test]
    fn test_ed25519_deterministic() {
        let signer = Ed25519Signer::generate();
        let message = b"deterministic test";

        let sig1 = signer.sign(message).unwrap();
        let sig2 = signer.sign(message).unwrap();

        // Ed25519 signatures are deterministic
        assert_eq!(sig1, sig2);
    }
}
