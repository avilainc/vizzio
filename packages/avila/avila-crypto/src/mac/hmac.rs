//! HMAC - Hash-based Message Authentication Code
//!
//! Generic HMAC implementation (RFC 2104) that works with any hash function.
//! Used for message authentication and key derivation.
//!
//! ## Supported Hash Functions
//! - MD5 (⚠️ legacy only, for CRAM-MD5)
//! - SHA-256
//! - SHA-384
//! - BLAKE3

use alloc::vec::Vec;
use core::cmp::min;

/// HMAC implementation (RFC 2104)
pub struct Hmac<const BLOCK_SIZE: usize, const OUTPUT_SIZE: usize> {
    key: [u8; BLOCK_SIZE],
}

impl<const BLOCK_SIZE: usize, const OUTPUT_SIZE: usize> Hmac<BLOCK_SIZE, OUTPUT_SIZE> {
    /// Create new HMAC with key
    pub fn new(key: &[u8], hash_fn: impl Fn(&[u8]) -> [u8; OUTPUT_SIZE]) -> Self {
        let mut processed_key = [0u8; BLOCK_SIZE];

        if key.len() > BLOCK_SIZE {
            let hashed = hash_fn(key);
            let copy_len = min(OUTPUT_SIZE, BLOCK_SIZE);
            processed_key[..copy_len].copy_from_slice(&hashed[..copy_len]);
        } else {
            processed_key[..key.len()].copy_from_slice(key);
        }

        Self { key: processed_key }
    }

    /// Compute HMAC
    pub fn compute(&self, data: &[u8], hash_fn: impl Fn(&[u8]) -> [u8; OUTPUT_SIZE]) -> [u8; OUTPUT_SIZE] {
        let mut ipad = [0x36u8; BLOCK_SIZE];
        for (i, byte) in ipad.iter_mut().enumerate() {
            *byte ^= self.key[i];
        }

        let mut opad = [0x5cu8; BLOCK_SIZE];
        for (i, byte) in opad.iter_mut().enumerate() {
            *byte ^= self.key[i];
        }

        let mut inner_input = Vec::with_capacity(BLOCK_SIZE + data.len());
        inner_input.extend_from_slice(&ipad);
        inner_input.extend_from_slice(data);
        let inner_hash = hash_fn(&inner_input);

        let mut outer_input = Vec::with_capacity(BLOCK_SIZE + OUTPUT_SIZE);
        outer_input.extend_from_slice(&opad);
        outer_input.extend_from_slice(&inner_hash);
        hash_fn(&outer_input)
    }
}

/// HMAC-MD5 (⚠️ LEGACY ONLY - for CRAM-MD5)
pub fn hmac_md5(key: &[u8], data: &[u8]) -> [u8; 16] {
    use crate::hash::md5::md5;

    let hmac: Hmac<64, 16> = Hmac::new(key, md5);
    hmac.compute(data, md5)
}

/// Convert bytes to hex string (lowercase)
pub fn to_hex(bytes: &[u8]) -> alloc::string::String {
    use alloc::string::String;
    use core::fmt::Write;

    let mut s = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        // Writing to a `String` cannot fail.
        let _ = write!(s, "{:02x}", byte);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::md5::md5;

    #[test]
    fn test_hmac_md5_rfc2202_case1() {
        // Test Case 1 from RFC 2202
        let key = [0x0b; 16];
        let data = b"Hi There";
        let expected = [
            0x92, 0x94, 0x72, 0x7a, 0x36, 0x38, 0xbb, 0x1c,
            0x13, 0xf4, 0x8e, 0xf8, 0x15, 0x8b, 0xfc, 0x9d,
        ];

        let result = hmac_md5(&key, data);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hmac_md5_rfc2202_case2() {
        // Test Case 2 from RFC 2202
        let key = b"Jefe";
        let data = b"what do ya want for nothing?";
        let expected = [
            0x75, 0x0c, 0x78, 0x3e, 0x6a, 0xb0, 0xb5, 0x03,
            0xea, 0xa8, 0x6e, 0x31, 0x0a, 0x5d, 0xb7, 0x38,
        ];

        let result = hmac_md5(key, data);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_hex() {
        let bytes = [0x92, 0x94, 0x72, 0x7a];
        assert_eq!(to_hex(&bytes), "9294727a");
    }
}
