//! Base32 encoding/decoding (RFC 4648)
//!
//! Supports standard and extended hex alphabets.

use crate::{Error, ErrorKind, Result};
use alloc::{string::String, vec::Vec};

const STANDARD_ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const HEX_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHIJKLMNOPQRSTUV";
const PAD: u8 = b'=';

/// Base32 encoding variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    /// Standard Base32 (RFC 4648)
    Standard,
    /// Base32hex (extended hex alphabet)
    Hex,
}

impl Variant {
    fn alphabet(&self) -> &'static [u8; 32] {
        match self {
            Variant::Standard => STANDARD_ALPHABET,
            Variant::Hex => HEX_ALPHABET,
        }
    }
}

/// Encodes bytes to base32 string (standard alphabet)
pub fn encode(data: &[u8]) -> String {
    encode_variant(data, Variant::Standard)
}

/// Encodes bytes to base32hex string
pub fn encode_hex(data: &[u8]) -> String {
    encode_variant(data, Variant::Hex)
}

/// Encodes bytes to base32 with specified variant
pub fn encode_variant(data: &[u8], variant: Variant) -> String {
    if data.is_empty() {
        return String::new();
    }

    let alphabet = variant.alphabet();
    let mut result = String::with_capacity((data.len() * 8 + 4) / 5);

    for chunk in data.chunks(5) {
        let mut buf = [0u8; 5];
        buf[..chunk.len()].copy_from_slice(chunk);

        result.push(alphabet[((buf[0] & 0xF8) >> 3) as usize] as char);
        result.push(alphabet[(((buf[0] & 0x07) << 2) | ((buf[1] & 0xC0) >> 6)) as usize] as char);

        if chunk.len() > 1 {
            result.push(alphabet[((buf[1] & 0x3E) >> 1) as usize] as char);
            result.push(alphabet[(((buf[1] & 0x01) << 4) | ((buf[2] & 0xF0) >> 4)) as usize] as char);
        } else {
            result.push(PAD as char);
            result.push(PAD as char);
        }

        if chunk.len() > 2 {
            result.push(alphabet[(((buf[2] & 0x0F) << 1) | ((buf[3] & 0x80) >> 7)) as usize] as char);
        } else {
            result.push(PAD as char);
        }

        if chunk.len() > 3 {
            result.push(alphabet[((buf[3] & 0x7C) >> 2) as usize] as char);
            result.push(alphabet[(((buf[3] & 0x03) << 3) | ((buf[4] & 0xE0) >> 5)) as usize] as char);
        } else {
            result.push(PAD as char);
            result.push(PAD as char);
        }

        if chunk.len() > 4 {
            result.push(alphabet[(buf[4] & 0x1F) as usize] as char);
        } else {
            result.push(PAD as char);
        }
    }

    result
}

/// Decodes base32 string to bytes (standard alphabet)
pub fn decode(encoded: &str) -> Result<Vec<u8>> {
    decode_variant(encoded, Variant::Standard)
}

/// Decodes base32hex string to bytes
pub fn decode_hex(encoded: &str) -> Result<Vec<u8>> {
    decode_variant(encoded, Variant::Hex)
}

/// Decodes base32 with specified variant
pub fn decode_variant(encoded: &str, variant: Variant) -> Result<Vec<u8>> {
    if encoded.is_empty() {
        return Ok(Vec::new());
    }

    let bytes = encoded.as_bytes();
    if bytes.len() % 8 != 0 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Base32 length must be multiple of 8",
        ));
    }

    let mut result = Vec::with_capacity(bytes.len() * 5 / 8);

    for chunk in bytes.chunks(8) {
        let b = [
            decode_char(chunk[0], variant)?,
            decode_char(chunk[1], variant)?,
            if chunk[2] == PAD { 0 } else { decode_char(chunk[2], variant)? },
            if chunk[3] == PAD { 0 } else { decode_char(chunk[3], variant)? },
            if chunk[4] == PAD { 0 } else { decode_char(chunk[4], variant)? },
            if chunk[5] == PAD { 0 } else { decode_char(chunk[5], variant)? },
            if chunk[6] == PAD { 0 } else { decode_char(chunk[6], variant)? },
            if chunk[7] == PAD { 0 } else { decode_char(chunk[7], variant)? },
        ];

        result.push((b[0] << 3) | (b[1] >> 2));

        if chunk[2] != PAD {
            result.push((b[1] << 6) | (b[2] << 1) | (b[3] >> 4));
        }

        if chunk[4] != PAD {
            result.push((b[3] << 4) | (b[4] >> 1));
        }

        if chunk[5] != PAD {
            result.push((b[4] << 7) | (b[5] << 2) | (b[6] >> 3));
        }

        if chunk[7] != PAD {
            result.push((b[6] << 5) | b[7]);
        }
    }

    Ok(result)
}

fn decode_char(c: u8, variant: Variant) -> Result<u8> {
    let alphabet = variant.alphabet();
    alphabet
        .iter()
        .position(|&b| b == c)
        .map(|p| p as u8)
        .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Invalid base32 character"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_standard() {
        assert_eq!(encode(b""), "");
        assert_eq!(encode(b"f"), "MY======");
        assert_eq!(encode(b"fo"), "MZXQ====");
        assert_eq!(encode(b"foo"), "MZXW6===");
        assert_eq!(encode(b"foob"), "MZXW6YQ=");
        assert_eq!(encode(b"fooba"), "MZXW6YTB");
        assert_eq!(encode(b"foobar"), "MZXW6YTBOI======");
    }

    #[test]
    fn test_decode_standard() {
        assert_eq!(decode("MY======").unwrap(), b"f");
        assert_eq!(decode("MZXQ====").unwrap(), b"fo");
        assert_eq!(decode("MZXW6===").unwrap(), b"foo");
        assert_eq!(decode("MZXW6YQ=").unwrap(), b"foob");
        assert_eq!(decode("MZXW6YTB").unwrap(), b"fooba");
        assert_eq!(decode("MZXW6YTBOI======").unwrap(), b"foobar");
    }

    #[test]
    fn test_roundtrip() {
        let data = b"The quick brown fox jumps over the lazy dog";
        let encoded = encode(data);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_hex_variant() {
        let data = b"hello";
        let encoded = encode_hex(data);
        let decoded = decode_hex(&encoded).unwrap();
        assert_eq!(decoded, data);
    }
}
