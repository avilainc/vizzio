//! # AVL-AUTH - Autenticação Mínima AVILA
//!
//! Implementação mínima funcional usando APENAS dependências AVILA internas.
//!
//! ## Features
//! - ✅ Autenticação usuário/senha
//! - ✅ Hash seguro de senhas (SHA-256 + salt)
//! - ✅ Sessões básicas
//! - ✅ 100% dependências AVILA

#![cfg_attr(not(feature = "std"), no_std)]

use avila_error::{Error, ErrorKind, Result};
use avila_hash::Sha256;
use avila_codec::hex;
use core::fmt;

#[cfg(feature = "std")]
use std::collections::HashMap;

// ============================================================================
// CORE TYPES
// ============================================================================

/// User ID (simple string for now)
pub type UserId = String;

/// Session ID (simple string for now)
pub type SessionId = String;

/// User credentials
#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub password_hash: String,
    pub salt: Vec<u8>,
}

/// Login credentials
#[derive(Debug, Clone)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

/// Active session
#[derive(Debug, Clone)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    pub expires_at: u64, // Unix timestamp
}

// ============================================================================
// AUTH CLIENT
// ============================================================================

/// Minimal authentication client
#[cfg(feature = "std")]
pub struct AuthClient {
    users: HashMap<String, User>,
    sessions: HashMap<SessionId, Session>,
}

#[cfg(feature = "std")]
impl AuthClient {
    /// Create new auth client
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            sessions: HashMap::new(),
        }
    }

    /// Register new user
    pub fn register(&mut self, email: &str, password: &str) -> Result<UserId> {
        // Check if user exists
        if self.users.contains_key(email) {
            return Err(Error::new(ErrorKind::InvalidInput, "User already exists"));
        }

        // Generate salt (simple counter-based for now)
        let salt = generate_salt();

        // Hash password
        let password_hash = hash_password(password, &salt)?;

        // Create user
        let user_id = format!("user_{}", self.users.len());
        let user = User {
            id: user_id.clone(),
            email: email.to_string(),
            password_hash,
            salt,
        };

        self.users.insert(email.to_string(), user);
        Ok(user_id)
    }

    /// Login user
    pub fn login(&mut self, credentials: Credentials) -> Result<Session> {
        // Find user
        let user = self.users
            .get(&credentials.email)
            .ok_or_else(|| Error::new(ErrorKind::Auth, "Invalid credentials"))?;

        // Verify password
        if !verify_password(&credentials.password, &user.password_hash, &user.salt)? {
            return Err(Error::new(ErrorKind::Auth, "Invalid credentials"));
        }

        // Create session
        let session_id = format!("session_{}", self.sessions.len());
        let session = Session {
            id: session_id.clone(),
            user_id: user.id.clone(),
            expires_at: current_timestamp() + 3600, // 1 hour
        };

        self.sessions.insert(session_id, session.clone());
        Ok(session)
    }

    /// Verify session
    pub fn verify_session(&self, session_id: &str) -> Result<&Session> {
        let session = self.sessions
            .get(session_id)
            .ok_or_else(|| Error::new(ErrorKind::Auth, "Invalid session"))?;

        // Check expiration
        if session.expires_at < current_timestamp() {
            return Err(Error::new(ErrorKind::Auth, "Session expired"));
        }

        Ok(session)
    }

    /// Logout (invalidate session)
    pub fn logout(&mut self, session_id: &str) -> Result<()> {
        self.sessions.remove(session_id)
            .ok_or_else(|| Error::new(ErrorKind::Auth, "Session not found"))?;
        Ok(())
    }
}

#[cfg(feature = "std")]
impl Default for AuthClient {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// PASSWORD HASHING
// ============================================================================

/// Hash password using SHA-256 + salt
pub fn hash_password(password: &str, salt: &[u8]) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt);
    let hash = hasher.finalize();
    Ok(hex::encode(&hash.as_bytes()))
}

/// Verify password against hash
pub fn verify_password(password: &str, hash: &str, salt: &[u8]) -> Result<bool> {
    let computed = hash_password(password, salt)?;
    Ok(constant_time_eq(computed.as_bytes(), hash.as_bytes()))
}

/// Constant-time comparison
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

/// Generate salt (simple implementation)
fn generate_salt() -> Vec<u8> {
    // Simple deterministic salt for now
    // In production, use avila-rand
    let timestamp = current_timestamp();
    timestamp.to_le_bytes().to_vec()
}

/// Get current Unix timestamp
fn current_timestamp() -> u64 {
    #[cfg(feature = "std")]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    #[cfg(not(feature = "std"))]
    {
        0 // No-std fallback
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "secure_password_123";
        let salt = b"test_salt";

        let hash1 = hash_password(password, salt).unwrap();
        let hash2 = hash_password(password, salt).unwrap();

        assert_eq!(hash1, hash2);
        assert!(verify_password(password, &hash1, salt).unwrap());
        assert!(!verify_password("wrong_password", &hash1, salt).unwrap());
    }

    #[test]
    fn test_register_and_login() {
        let mut client = AuthClient::new();

        // Register
        let user_id = client.register("test@example.com", "password123").unwrap();
        assert!(!user_id.is_empty());

        // Login with correct credentials
        let session = client.login(Credentials {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        }).unwrap();

        assert_eq!(session.user_id, user_id);

        // Verify session
        let verified = client.verify_session(&session.id).unwrap();
        assert_eq!(verified.user_id, user_id);

        // Logout
        client.logout(&session.id).unwrap();
        assert!(client.verify_session(&session.id).is_err());
    }

    #[test]
    fn test_duplicate_registration() {
        let mut client = AuthClient::new();

        client.register("test@example.com", "password123").unwrap();
        let result = client.register("test@example.com", "password456");

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_login() {
        let mut client = AuthClient::new();

        client.register("test@example.com", "password123").unwrap();

        // Wrong password
        let result = client.login(Credentials {
            email: "test@example.com".to_string(),
            password: "wrong_password".to_string(),
        });
        assert!(result.is_err());

        // Wrong email
        let result = client.login(Credentials {
            email: "wrong@example.com".to_string(),
            password: "password123".to_string(),
        });
        assert!(result.is_err());
    }
}
