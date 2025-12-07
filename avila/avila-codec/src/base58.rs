//! Base58 encoding/decoding (Bitcoin style)
//!
//! Bitcoin-style base58 encoding without similar-looking characters.

use crate::{Error, ErrorKind, Result};
use alloc::{string::String, vec::Vec};

const ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Encodes bytes to base58 string
pub fn encode(data: &[u8]) -> String {
    if data.is_empty() {
        return String::new();
    }

    // Count leading zeros
    let leading_zeros = data.iter().take_while(|&&b| b == 0).count();

    // Convert to base58
    let mut num = Vec::from(data);
    let mut encoded = Vec::new();

    while !num.is_empty() && num.iter().any(|&b| b != 0) {
        let mut carry = 0u32;
        for byte in num.iter_mut() {
            carry = carry * 256 + *byte as u32;
            *byte = (carry / 58) as u8;
            carry %= 58;
        }
        encoded.push(ALPHABET[carry as usize]);

        // Remove leading zeros
        while num.first() == Some(&0) {
            num.remove(0);
        }
    }

    // Add leading '1's for leading zeros in input
    let mut result = String::with_capacity(leading_zeros + encoded.len());
    for _ in 0..leading_zeros {
        result.push('1');
    }
    for &byte in encoded.iter().rev() {
        result.push(byte as char);
    }

    result
}

/// Decodes base58 string to bytes
pub fn decode(encoded: &str) -> Result<Vec<u8>> {
    if encoded.is_empty() {
        return Ok(Vec::new());
    }

    // Count leading '1's
    let leading_ones = encoded.chars().take_while(|&c| c == '1').count();

    // Convert from base58
    let mut result = Vec::new();
    for c in encoded.chars() {
        let digit = ALPHABET.iter()
            .position(|&b| b == c as u8)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Invalid base58 character"))?;

        let mut carry = digit as u32;
        for byte in result.iter_mut() {
            carry += *byte as u32 * 58;
            *byte = carry as u8;
            carry >>= 8;
        }

        while carry > 0 {
            result.push(carry as u8);
            carry >>= 8;
        }
    }

    // Add leading zeros
    let mut output = vec![0u8; leading_ones];
    output.extend(result.iter().rev());

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base58_encode() {
        assert_eq!(encode(b"Hello"), "9Ajdvzr");
        assert_eq!(encode(b""), "");
        assert_eq!(encode(b"\x00\x00test"), "113yZe7d");
    }

    #[test]
    fn test_base58_decode() {
        assert_eq!(decode("9Ajdvzr").unwrap(), b"Hello");
        assert_eq!(decode("").unwrap(), b"");
    }

    #[test]
    fn test_base58_roundtrip() {
        let data = b"Bitcoin";
        let encoded = encode(data);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }
}
