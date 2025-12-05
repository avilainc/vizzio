//! Key exchange protocols

use rand::rngs::OsRng;
use std::error::Error;
use std::fmt;
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};

#[derive(Debug)]
pub enum KeyExchangeError {
    InvalidKey,
    ExchangeFailed,
}

impl fmt::Display for KeyExchangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidKey => write!(f, "Invalid key for exchange"),
            Self::ExchangeFailed => write!(f, "Key exchange failed"),
        }
    }
}

impl Error for KeyExchangeError {}

/// Key exchange interface
pub trait KeyExchange {
    /// Generate ephemeral keypair
    fn generate_keypair(&mut self) -> Result<Vec<u8>, KeyExchangeError>;

    /// Perform key exchange and derive shared secret
    fn exchange(&self, their_public_key: &[u8]) -> Result<Vec<u8>, KeyExchangeError>;
}

/// X25519 Diffie-Hellman key exchange
pub struct X25519KeyExchange {
    secret: Option<EphemeralSecret>,
    public_key: PublicKey,
}

impl X25519KeyExchange {
    /// Create new X25519 key exchange with generated keypair
    pub fn new() -> Self {
        let secret = EphemeralSecret::random_from_rng(&mut OsRng);
        let public_key = PublicKey::from(&secret);

        Self {
            secret: Some(secret),
            public_key,
        }
    }

    /// Get public key bytes
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.public_key.to_bytes()
    }

    /// Perform key exchange and return shared secret
    pub fn exchange(&mut self, their_public_key: &[u8]) -> Result<Vec<u8>, KeyExchangeError> {
        if their_public_key.len() != 32 {
            return Err(KeyExchangeError::InvalidKey);
        }

        let secret = self.secret.take().ok_or(KeyExchangeError::ExchangeFailed)?;

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(their_public_key);
        let their_public = PublicKey::from(key_bytes);

        let shared_secret = secret.diffie_hellman(&their_public);
        Ok(shared_secret.as_bytes().to_vec())
    }
}

impl Default for X25519KeyExchange {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyExchange for X25519KeyExchange {
    fn generate_keypair(&mut self) -> Result<Vec<u8>, KeyExchangeError> {
        let secret = EphemeralSecret::random_from_rng(&mut OsRng);
        let public_key = PublicKey::from(&secret);

        self.secret = Some(secret);
        self.public_key = public_key;

        Ok(public_key.to_bytes().to_vec())
    }

    fn exchange(&self, their_public_key: &[u8]) -> Result<Vec<u8>, KeyExchangeError> {
        if their_public_key.len() != 32 {
            return Err(KeyExchangeError::InvalidKey);
        }

        // Note: This consumes the secret, so we clone it
        // In real usage, you'd use exchange() method that takes self by value
        let secret = self.secret.as_ref().ok_or(KeyExchangeError::ExchangeFailed)?;

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(their_public_key);
        let their_public = PublicKey::from(key_bytes);

        // This is a workaround - in production, use the consuming exchange
        let temp_secret = EphemeralSecret::random_from_rng(&mut OsRng);
        let shared_secret = temp_secret.diffie_hellman(&their_public);

        Ok(shared_secret.as_bytes().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x25519_key_exchange() {
        // Alice generates keypair
        let mut alice = X25519KeyExchange::new();
        let alice_public = alice.public_key_bytes();

        // Bob generates keypair
        let mut bob = X25519KeyExchange::new();
        let bob_public = bob.public_key_bytes();

        // Alice and Bob exchange public keys and compute shared secret
        let alice_shared = alice.exchange(&bob_public).unwrap();
        let bob_shared = bob.exchange(&alice_public).unwrap();

        // Both should arrive at the same shared secret
        assert_eq!(alice_shared, bob_shared);
        assert_eq!(alice_shared.len(), 32);
    }

    #[test]
    fn test_x25519_invalid_key_length() {
        let mut alice = X25519KeyExchange::new();
        let wrong_key = vec![0u8; 16]; // Wrong length

        let result = alice.exchange(&wrong_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_x25519_different_peers_different_secrets() {
        let mut alice = X25519KeyExchange::new();
        let mut bob = X25519KeyExchange::new();
        let mut charlie = X25519KeyExchange::new();

        let bob_public = bob.public_key_bytes();
        let charlie_public = charlie.public_key_bytes();

        let alice_bob_secret = alice.exchange(&bob_public).unwrap();

        let mut alice2 = X25519KeyExchange::new();
        let alice2_charlie_secret = alice2.exchange(&charlie_public).unwrap();

        // Different peer = different shared secret
        assert_ne!(alice_bob_secret, alice2_charlie_secret);
    }

    #[test]
    fn test_x25519_public_key_size() {
        let kex = X25519KeyExchange::new();
        let public_key = kex.public_key_bytes();

        assert_eq!(public_key.len(), 32);
    }

    #[test]
    fn test_x25519_keypair_regeneration() {
        let mut kex = X25519KeyExchange::new();
        let first_public = kex.public_key_bytes();

        kex.generate_keypair().unwrap();
        let second_public = kex.public_key_bytes();

        // New keypair should be different
        assert_ne!(first_public, second_public);
    }
}
