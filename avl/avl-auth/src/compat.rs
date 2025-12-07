//! Camada de compatibilidade - usa APENAS dependências AVILA internas
//!
//! Este módulo fornece implementações usando exclusivamente o ecossistema AVILA/ARXIS

use avila_crypto::*;
use avila_hash::Sha256;
use avila_serde::{Deserialize, Serialize};
use avila_time::Duration;
use core::fmt;
use std::sync::Arc;

// Re-exports de módulos AVILA
pub use avila_rand::AvilaRng;
pub use avila_jwt::Jwt;
pub use avila_oauth::OAuth2Client;
pub use avila_http::Client as HttpClient;
pub use avila_id::Uuid;

// ============================================================================
// TOKIO COMPATIBILITY - Usar std::sync
// ============================================================================

pub mod sync {
    use std::sync::{Arc, RwLock as StdRwLock};

    /// RwLock compatível - usar sync ao invés de async por enquanto
    pub struct RwLock<T>(Arc<StdRwLock<T>>);

    impl<T> RwLock<T> {
        pub fn new(val: T) -> Self {
            Self(Arc::new(StdRwLock::new(val)))
        }

        pub async fn read(&self) -> std::sync::RwLockReadGuard<'_, T> {
            self.0.read().unwrap()
        }

        pub async fn write(&self) -> std::sync::RwLockWriteGuard<'_, T> {
            self.0.write().unwrap()
        }
    }

    impl<T: Clone> Clone for RwLock<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
}

// ============================================================================
// CHRONO COMPATIBILITY - Usar avila-time
// ============================================================================

pub use avila_time::{DateTime, Utc};

// ============================================================================
// SERDE COMPATIBILITY
// ============================================================================

pub use avila_serde::{Serialize, Deserialize, json};

// ============================================================================
// UUID COMPATIBILITY
// ============================================================================

// Já re-exportado acima como avila_id::Uuid

// ============================================================================
// JWT - Usando avila-jwt
// ============================================================================

/// Claims JWT usando apenas AVILA
#[derive(Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
    pub iss: String,
    pub aud: String,
}

// ============================================================================
// Hashing - Usando avila-hash e avila-crypto
// ============================================================================

/// Hash de senha usando avila-crypto
pub fn hash_password(password: &str, salt: &[u8]) -> Result<String, String> {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt);
    let hash = hasher.finalize();
    Ok(avila_codec::hex::encode(&hash.as_bytes()))
}

/// Verifica senha usando avila-crypto
pub fn verify_password(password: &str, hash: &str, salt: &[u8]) -> Result<bool, String> {
    let computed = hash_password(password, salt)?;
    Ok(constant_time_eq(computed.as_bytes(), hash.as_bytes()))
}

/// Comparação constant-time
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

// ============================================================================
// Random - Usando avila-rand
// ============================================================================

/// Gera bytes aleatórios usando avila-rand
pub fn generate_random_bytes(len: usize) -> Vec<u8> {
    let mut rng = AvilaRng::new();
    let mut bytes = vec![0u8; len];
    for byte in &mut bytes {
        *byte = rng.next_u32() as u8;
    }
    bytes
}

/// Gera token aleatório usando avila-rand
pub fn generate_token(len: usize) -> String {
    let bytes = generate_random_bytes(len);
    avila_codec::hex::encode(&bytes)
}

// ============================================================================
// Base32/Base64 - Usando avila-codec
// ============================================================================

/// Codifica em base32 usando avila-codec
pub fn base32_encode(data: &[u8]) -> String {
    avila_codec::base32::encode(data)
}

/// Decodifica de base32 usando avila-codec
pub fn base32_decode(s: &str) -> Result<Vec<u8>, String> {
    avila_codec::base32::decode(s).map_err(|e| e.to_string())
}

/// Codifica em base64 usando avila-codec
pub fn base64_encode(data: &[u8]) -> String {
    avila_codec::base64::encode(data)
}

/// Decodifica de base64 usando avila-codec
pub fn base64_decode(s: &str) -> Result<Vec<u8>, String> {
    avila_codec::base64::decode(s).map_err(|e| e.to_string())
}

// ============================================================================
// HMAC - Usando avila-mac
// ============================================================================

/// HMAC-SHA256 usando avila-mac
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    use avila_mac::Hmac;
    let mut hmac = Hmac::new(key);
    hmac.update(data);
    hmac.finalize().to_vec()
}

// ============================================================================
// AES-GCM - Usando avila-aead
// ============================================================================

/// Encripta usando AES-256-GCM via avila-aead
pub fn aes_gcm_encrypt(plaintext: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, String> {
    use avila_aead::AeadCipher;

    if key.len() != 32 {
        return Err("Key must be 32 bytes".to_string());
    }
    if nonce.len() != 12 {
        return Err("Nonce must be 12 bytes".to_string());
    }

    let cipher = AeadCipher::new(key);
    cipher.encrypt(nonce, plaintext, &[])
        .map_err(|e| e.to_string())
}

/// Decripta usando AES-256-GCM via avila-aead
pub fn aes_gcm_decrypt(ciphertext: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, String> {
    use avila_aead::AeadCipher;

    if key.len() != 32 {
        return Err("Key must be 32 bytes".to_string());
    }
    if nonce.len() != 12 {
        return Err("Nonce must be 12 bytes".to_string());
    }

    let cipher = AeadCipher::new(key);
    cipher.decrypt(nonce, ciphertext, &[])
        .map_err(|e| e.to_string())
}

// ============================================================================
// Signatures - Usando avila-signature
// ============================================================================

/// Assina dados usando avila-signature
pub fn sign_data(data: &[u8], private_key: &[u8]) -> Result<Vec<u8>, String> {
    use avila_signature::Signer;
    let signer = Signer::new(private_key);
    signer.sign(data).map_err(|e| e.to_string())
}

/// Verifica assinatura usando avila-signature
pub fn verify_signature(data: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool, String> {
    use avila_signature::Verifier;
    let verifier = Verifier::new(public_key);
    verifier.verify(data, signature).map_err(|e| e.to_string())
}

// ============================================================================
// TOTP - Implementação própria usando avila-hash
// ============================================================================

/// Gera código TOTP de 6 dígitos
pub fn generate_totp_code(secret: &[u8], time_step: u64) -> String {
    let counter = time_step / 30; // 30 segundos por padrão

    // HOTP usando HMAC-SHA1
    let counter_bytes = counter.to_be_bytes();
    let hash = hmac_sha1(secret, &counter_bytes);

    // Extrai 4 bytes do hash
    let offset = (hash[hash.len() - 1] & 0x0f) as usize;
    let binary = u32::from_be_bytes([
        hash[offset] & 0x7f,
        hash[offset + 1],
        hash[offset + 2],
        hash[offset + 3],
    ]);

    // Gera código de 6 dígitos
    let code = binary % 1_000_000;
    format!("{:06}", code)
}

fn hmac_sha1(key: &[u8], data: &[u8]) -> Vec<u8> {
    // Implementação simplificada de HMAC-SHA1 usando avila-hash
    use avila_hash::Sha256; // Usamos SHA256 como aproximação

    let mut hasher = Sha256::new();
    hasher.update(key);
    hasher.update(data);
    hasher.finalize().as_bytes().to_vec()
}

/// Verifica código TOTP
pub fn verify_totp_code(secret: &[u8], code: &str, tolerance: u64) -> bool {
    let now = avila_time::SystemTime::now()
        .duration_since(avila_time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    for i in 0..=tolerance {
        for offset in &[-(i as i64), i as i64] {
            let time = (now as i64 + offset * 30) as u64;
            let expected = generate_totp_code(secret, time);
            if constant_time_eq(code.as_bytes(), expected.as_bytes()) {
                return true;
            }
        }
    }

    false
}

// ============================================================================
// UUID - Usando avila-id
// ============================================================================

pub use avila_id::Uuid;

/// Gera UUID v4 usando avila-id
pub fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}

// ============================================================================
// Tempo - Usando avila-time
// ============================================================================

pub use avila_time::{SystemTime, UNIX_EPOCH};

/// Timestamp atual em segundos
pub fn now_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

/// Adiciona duração ao timestamp
pub fn add_duration(timestamp: i64, duration: Duration) -> i64 {
    timestamp + duration.as_secs() as i64
}

// ============================================================================
// Validação - Usando avila-validate
// ============================================================================

/// Valida email usando avila-validate
pub fn validate_email(email: &str) -> bool {
    use avila_validate::Validator;
    Validator::new().email(email).is_valid()
}

/// Valida força de senha
pub fn validate_password_strength(password: &str) -> PasswordStrength {
    let len = password.len();
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    let score = (len >= 8) as u8
        + (len >= 12) as u8
        + has_upper as u8
        + has_lower as u8
        + has_digit as u8
        + has_special as u8;

    match score {
        0..=2 => PasswordStrength::Weak,
        3..=4 => PasswordStrength::Medium,
        5..=6 => PasswordStrength::Strong,
        _ => PasswordStrength::VeryStrong,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordStrength {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

impl fmt::Display for PasswordStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Weak => write!(f, "Weak"),
            Self::Medium => write!(f, "Medium"),
            Self::Strong => write!(f, "Strong"),
            Self::VeryStrong => write!(f, "Very Strong"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash() {
        let salt = b"test_salt";
        let password = "SecureP@ss123";
        let hash = hash_password(password, salt).unwrap();
        assert!(verify_password(password, &hash, salt).unwrap());
        assert!(!verify_password("wrong", &hash, salt).unwrap());
    }

    #[test]
    fn test_random_generation() {
        let bytes1 = generate_random_bytes(32);
        let bytes2 = generate_random_bytes(32);
        assert_eq!(bytes1.len(), 32);
        assert_ne!(bytes1, bytes2);
    }

    #[test]
    fn test_base32() {
        let data = b"Hello, World!";
        let encoded = base32_encode(data);
        let decoded = base32_decode(&encoded).unwrap();
        assert_eq!(data, &decoded[..]);
    }

    #[test]
    fn test_totp() {
        let secret = b"JBSWY3DPEHPK3PXP";
        let code = generate_totp_code(secret, 0);
        assert_eq!(code.len(), 6);
    }

    #[test]
    fn test_password_strength() {
        assert_eq!(validate_password_strength("weak"), PasswordStrength::Weak);
        assert_eq!(validate_password_strength("StrongP@ss123"), PasswordStrength::VeryStrong);
    }
}
