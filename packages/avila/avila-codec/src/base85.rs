//! Base85 encoding/decoding
//!
//! Supports ASCII85 (Adobe) and Z85 (ZeroMQ) variants.

use crate::{Error, ErrorKind, Result};
use alloc::{string::String, vec::Vec};

const ASCII85_ALPHABET: &[u8; 85] = b"!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstu";
const Z85_ALPHABET: &[u8; 85] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.-:+=^!/*?&<>()[]{}@%$#";

/// Base85 encoding variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    /// ASCII85 (Adobe style)
    Ascii85,
    /// Z85 (ZeroMQ style)
    Z85,
}

/// Encodes bytes to ASCII85 string
pub fn encode_ascii85(data: &[u8]) -> String {
    encode_variant(data, Variant::Ascii85)
}

/// Encodes bytes to Z85 string
pub fn encode_z85(data: &[u8]) -> String {
    encode_variant(data, Variant::Z85)
}

/// Encodes bytes to base85 with specified variant
pub fn encode_variant(data: &[u8], variant: Variant) -> String {
    if data.is_empty() {
        return String::new();
    }

    let alphabet = match variant {
        Variant::Ascii85 => ASCII85_ALPHABET,
        Variant::Z85 => Z85_ALPHABET,
    };

    let mut result = String::new();

    for chunk in data.chunks(4) {
        let mut buf = [0u8; 4];
        buf[..chunk.len()].copy_from_slice(chunk);

        let value = u32::from_be_bytes(buf);

        // ASCII85 special case: all zeros -> 'z'
        if variant == Variant::Ascii85 && value == 0 && chunk.len() == 4 {
            result.push('z');
            continue;
        }

        let mut encoded = [0u8; 5];
        let mut val = value;
        for i in (0..5).rev() {
            encoded[i] = alphabet[(val % 85) as usize];
            val /= 85;
        }

        let len = if chunk.len() < 4 { chunk.len() + 1 } else { 5 };
        for &b in &encoded[..len] {
            result.push(b as char);
        }
    }

    result
}

/// Decodes ASCII85 string to bytes
pub fn decode_ascii85(encoded: &str) -> Result<Vec<u8>> {
    decode_variant(encoded, Variant::Ascii85)
}

/// Decodes Z85 string to bytes
pub fn decode_z85(encoded: &str) -> Result<Vec<u8>> {
    decode_variant(encoded, Variant::Z85)
}

/// Decodes base85 with specified variant
pub fn decode_variant(encoded: &str, variant: Variant) -> Result<Vec<u8>> {
    if encoded.is_empty() {
        return Ok(Vec::new());
    }

    let alphabet = match variant {
        Variant::Ascii85 => ASCII85_ALPHABET,
        Variant::Z85 => Z85_ALPHABET,
    };

    let mut result = Vec::new();
    let bytes = encoded.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        // Handle 'z' shortcut in ASCII85
        if variant == Variant::Ascii85 && bytes[i] == b'z' {
            result.extend_from_slice(&[0, 0, 0, 0]);
            i += 1;
            continue;
        }

        let chunk_size = 5.min(bytes.len() - i);
        let mut value = 0u32;

        for j in 0..chunk_size {
            let c = bytes[i + j];
            let digit = alphabet
                .iter()
                .position(|&b| b == c)
                .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Invalid base85 character"))?;

            value = value * 85 + digit as u32;
        }

        // Adjust for incomplete chunks
        if chunk_size < 5 {
            for _ in chunk_size..5 {
                value = value * 85 + 84;
            }
        }

        let decoded = value.to_be_bytes();
        let output_len = if chunk_size < 5 { chunk_size - 1 } else { 4 };
        result.extend_from_slice(&decoded[..output_len]);

        i += chunk_size;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii85_encode() {
        assert_eq!(encode_ascii85(b"Man "), "9jqo^");
        assert_eq!(encode_ascii85(b"\0\0\0\0"), "z");
    }

    #[test]
    fn test_ascii85_decode() {
        assert_eq!(decode_ascii85("9jqo^").unwrap(), b"Man ");
        assert_eq!(decode_ascii85("z").unwrap(), b"\0\0\0\0");
    }

    #[test]
    fn test_z85_roundtrip() {
        let data = b"Hello World!";
        let encoded = encode_z85(data);
        let decoded = decode_z85(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_ascii85_roundtrip() {
        let data = b"The quick brown fox";
        let encoded = encode_ascii85(data);
        let decoded = decode_ascii85(&encoded).unwrap();
        assert_eq!(decoded, data);
    }
}
