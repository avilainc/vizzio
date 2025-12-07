//! Encryption/Decryption primitives

use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes256Gcm as Aes256GcmImpl, Nonce,
};
use chacha20poly1305::{
    aead::{Aead as ChachaAead, KeyInit as ChachaKeyInit},
    ChaCha20Poly1305 as ChaCha20Poly1305Impl, Key, Nonce as ChachaNonce,
};
use rand::{rngs::OsRng, RngCore};
use std::error::Error;
use std::fmt;
use zeroize::Zeroize;

#[derive(Debug)]
pub enum EncryptionError {
    InvalidKey,
    InvalidNonce,
    EncryptionFailed,
    DecryptionFailed,
    InvalidCiphertext,
}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidKey => write!(f, "Invalid encryption key"),
            Self::InvalidNonce => write!(f, "Invalid nonce"),
            Self::EncryptionFailed => write!(f, "Encryption operation failed"),
            Self::DecryptionFailed => write!(f, "Decryption operation failed"),
            Self::InvalidCiphertext => write!(f, "Invalid ciphertext format"),
        }
    }
}

impl Error for EncryptionError {}

/// Cipher interface for encryption operations
pub trait Cipher {
    /// Encrypt plaintext with authenticated encryption
    fn encrypt(&self, plaintext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>, EncryptionError>;

    /// Decrypt ciphertext with authentication verification
    fn decrypt(&self, ciphertext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>, EncryptionError>;
}

/// AES-256-GCM cipher (AEAD)
pub struct Aes256Gcm {
    cipher: Aes256GcmImpl,
}

impl Aes256Gcm {
    /// Create new AES-256-GCM cipher with 32-byte key
    pub fn new(key: [u8; 32]) -> Self {
        let cipher = Aes256GcmImpl::new(&key.into());
        Self { cipher }
    }

    /// Generate random 32-byte key
    pub fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        key
    }

    /// Generate random 12-byte nonce (96 bits for GCM)
    pub fn generate_nonce() -> [u8; 12] {
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);
        nonce
    }
}

impl Cipher for Aes256Gcm {
    fn encrypt(&self, plaintext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        // Generate random nonce
        let nonce_bytes = Self::generate_nonce();
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Create payload with AAD
        let payload = Payload {
            msg: plaintext,
            aad: associated_data,
        };

        // Encrypt
        let ciphertext = self
            .cipher
            .encrypt(nonce, payload)
            .map_err(|_| EncryptionError::EncryptionFailed)?;

        // Prepend nonce to ciphertext (nonce doesn't need to be secret)
        let mut result = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    fn decrypt(&self, ciphertext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        // Extract nonce (first 12 bytes)
        if ciphertext.len() < 12 {
            return Err(EncryptionError::InvalidCiphertext);
        }

        let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // Create payload with AAD
        let payload = Payload {
            msg: encrypted_data,
            aad: associated_data,
        };

        // Decrypt and verify authentication tag
        self.cipher
            .decrypt(nonce, payload)
            .map_err(|_| EncryptionError::DecryptionFailed)
    }
}

/// ChaCha20-Poly1305 cipher (AEAD)
pub struct ChaCha20Poly1305 {
    cipher: ChaCha20Poly1305Impl,
}

impl ChaCha20Poly1305 {
    /// Create new ChaCha20-Poly1305 cipher with 32-byte key
    pub fn new(key: [u8; 32]) -> Self {
        let cipher = ChaCha20Poly1305Impl::new(Key::from_slice(&key));
        Self { cipher }
    }

    /// Generate random 32-byte key
    pub fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        key
    }

    /// Generate random 12-byte nonce (96 bits)
    pub fn generate_nonce() -> [u8; 12] {
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);
        nonce
    }
}

impl Cipher for ChaCha20Poly1305 {
    fn encrypt(&self, plaintext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        // Generate random nonce
        let nonce_bytes = Self::generate_nonce();
        let nonce = ChachaNonce::from_slice(&nonce_bytes);

        // Create payload with AAD
        let payload = chacha20poly1305::aead::Payload {
            msg: plaintext,
            aad: associated_data,
        };

        // Encrypt
        let ciphertext = self
            .cipher
            .encrypt(nonce, payload)
            .map_err(|_| EncryptionError::EncryptionFailed)?;

        // Prepend nonce to ciphertext
        let mut result = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    fn decrypt(&self, ciphertext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        // Extract nonce (first 12 bytes)
        if ciphertext.len() < 12 {
            return Err(EncryptionError::InvalidCiphertext);
        }

        let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
        let nonce = ChachaNonce::from_slice(nonce_bytes);

        // Create payload with AAD
        let payload = chacha20poly1305::aead::Payload {
            msg: encrypted_data,
            aad: associated_data,
        };

        // Decrypt and verify authentication tag
        self.cipher
            .decrypt(nonce, payload)
            .map_err(|_| EncryptionError::DecryptionFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes256_gcm_encrypt_decrypt() {
        let key = Aes256Gcm::generate_key();
        let cipher = Aes256Gcm::new(key);
        let plaintext = b"Hello, World! This is a secret message.";
        let aad = b"additional authenticated data";

        // Encrypt
        let ciphertext = cipher.encrypt(plaintext, aad).unwrap();

        // Ciphertext should be different from plaintext
        assert_ne!(&ciphertext[12..], plaintext);

        // Ciphertext should include nonce (12 bytes) + encrypted data + tag (16 bytes)
        assert!(ciphertext.len() > plaintext.len() + 12);

        // Decrypt
        let decrypted = cipher.decrypt(&ciphertext, aad).unwrap();
        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_aes256_gcm_with_empty_aad() {
        let key = Aes256Gcm::generate_key();
        let cipher = Aes256Gcm::new(key);
        let plaintext = b"test data";

        let ciphertext = cipher.encrypt(plaintext, &[]).unwrap();
        let decrypted = cipher.decrypt(&ciphertext, &[]).unwrap();
        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_aes256_gcm_wrong_aad_fails() {
        let key = Aes256Gcm::generate_key();
        let cipher = Aes256Gcm::new(key);
        let plaintext = b"secret";

        let ciphertext = cipher.encrypt(plaintext, b"correct aad").unwrap();
        let result = cipher.decrypt(&ciphertext, b"wrong aad");
        assert!(result.is_err());
    }

    #[test]
    fn test_aes256_gcm_tampering_fails() {
        let key = Aes256Gcm::generate_key();
        let cipher = Aes256Gcm::new(key);
        let plaintext = b"important data";

        let mut ciphertext = cipher.encrypt(plaintext, &[]).unwrap();

        // Tamper with ciphertext (flip a bit)
        if ciphertext.len() > 15 {
            ciphertext[15] ^= 0xFF;
        }

        let result = cipher.decrypt(&ciphertext, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_chacha20_poly1305_encrypt_decrypt() {
        let key = ChaCha20Poly1305::generate_key();
        let cipher = ChaCha20Poly1305::new(key);
        let plaintext = b"ChaCha20-Poly1305 is fast and secure!";
        let aad = b"metadata";

        let ciphertext = cipher.encrypt(plaintext, aad).unwrap();
        assert_ne!(&ciphertext[12..], plaintext);

        let decrypted = cipher.decrypt(&ciphertext, aad).unwrap();
        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_chacha20_poly1305_wrong_key_fails() {
        let key1 = ChaCha20Poly1305::generate_key();
        let key2 = ChaCha20Poly1305::generate_key();

        let cipher1 = ChaCha20Poly1305::new(key1);
        let cipher2 = ChaCha20Poly1305::new(key2);

        let plaintext = b"encrypted with key1";
        let ciphertext = cipher1.encrypt(plaintext, &[]).unwrap();

        let result = cipher2.decrypt(&ciphertext, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_ciphertext() {
        let key = Aes256Gcm::generate_key();
        let cipher = Aes256Gcm::new(key);

        // Ciphertext too short (less than nonce size)
        let result = cipher.decrypt(&[1, 2, 3], &[]);
        assert!(matches!(result, Err(EncryptionError::InvalidCiphertext)));
    }

    #[test]
    fn test_different_nonces_produce_different_ciphertexts() {
        let key = Aes256Gcm::generate_key();
        let cipher = Aes256Gcm::new(key);
        let plaintext = b"same plaintext";

        let ciphertext1 = cipher.encrypt(plaintext, &[]).unwrap();
        let ciphertext2 = cipher.encrypt(plaintext, &[]).unwrap();

        // Nonces are random, so ciphertexts should differ
        assert_ne!(ciphertext1, ciphertext2);

        // But both should decrypt to same plaintext
        assert_eq!(cipher.decrypt(&ciphertext1, &[]).unwrap(), plaintext);
        assert_eq!(cipher.decrypt(&ciphertext2, &[]).unwrap(), plaintext);
    }
}
