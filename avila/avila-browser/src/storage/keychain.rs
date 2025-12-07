//! Secure keychain for storing credentials

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum KeychainError {
    NotFound,
    AccessDenied,
    StorageFailed,
}

impl fmt::Display for KeychainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "Credential not found in keychain"),
            Self::AccessDenied => write!(f, "Access to keychain denied"),
            Self::StorageFailed => write!(f, "Failed to store credential"),
        }
    }
}

impl Error for KeychainError {}

/// Credential stored in keychain
#[derive(Debug, Clone)]
pub struct Credential {
    pub username: String,
    pub password: String,
    pub domain: String,
}

/// Secure keychain for storing credentials
pub struct Keychain {
    credentials: HashMap<String, Credential>,
    encrypted: bool,
}

impl Keychain {
    /// Create new keychain
    pub fn new(encrypted: bool) -> Self {
        Self {
            credentials: HashMap::new(),
            encrypted,
        }
    }

    /// Store a credential
    pub fn store(&mut self, credential: Credential) -> Result<(), KeychainError> {
        let key = format!("{}@{}", credential.username, credential.domain);

        // TODO: Encrypt credential if encryption is enabled
        self.credentials.insert(key, credential);
        Ok(())
    }

    /// Retrieve a credential
    pub fn retrieve(&self, domain: &str, username: &str) -> Result<Credential, KeychainError> {
        let key = format!("{}@{}", username, domain);

        self.credentials
            .get(&key)
            .cloned()
            .ok_or(KeychainError::NotFound)
    }

    /// Delete a credential
    pub fn delete(&mut self, domain: &str, username: &str) -> Result<(), KeychainError> {
        let key = format!("{}@{}", username, domain);

        self.credentials
            .remove(&key)
            .ok_or(KeychainError::NotFound)?;

        Ok(())
    }

    /// List all stored credentials (without passwords)
    pub fn list(&self) -> Vec<(String, String)> {
        self.credentials
            .values()
            .map(|cred| (cred.domain.clone(), cred.username.clone()))
            .collect()
    }

    /// Clear all credentials
    pub fn clear(&mut self) {
        self.credentials.clear();
    }

    /// Check if credential exists
    pub fn exists(&self, domain: &str, username: &str) -> bool {
        let key = format!("{}@{}", username, domain);
        self.credentials.contains_key(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keychain_operations() {
        let mut keychain = Keychain::new(false);

        let cred = Credential {
            username: "user@example.com".to_string(),
            password: "secret123".to_string(),
            domain: "example.com".to_string(),
        };

        // Store credential
        keychain.store(cred.clone()).unwrap();

        // Retrieve credential
        let retrieved = keychain.retrieve("example.com", "user@example.com").unwrap();
        assert_eq!(retrieved.password, "secret123");

        // Check exists
        assert!(keychain.exists("example.com", "user@example.com"));

        // Delete credential
        keychain.delete("example.com", "user@example.com").unwrap();
        assert!(!keychain.exists("example.com", "user@example.com"));
    }
}
