//! Base64 encoding/decoding
//!
//! Standard base64 encoding (RFC 4648).

use crate::{Error, ErrorKind, Result};
use alloc::{string::String, vec::Vec};

const ENCODE_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const PAD: u8 = b'=';

/// Encodes bytes to base64 string
pub fn encode(data: &[u8]) -> String {
    let mut result = String::with_capacity((data.len() + 2) / 3 * 4);

    for chunk in data.chunks(3) {
        let mut buf = [0u8; 3];
        buf[..chunk.len()].copy_from_slice(chunk);

        let b1 = (buf[0] >> 2) as usize;
        let b2 = (((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize;
        let b3 = (((buf[1] & 0x0f) << 2) | (buf[2] >> 6)) as usize;
        let b4 = (buf[2] & 0x3f) as usize;

        result.push(ENCODE_TABLE[b1] as char);
        result.push(ENCODE_TABLE[b2] as char);
        result.push(if chunk.len() > 1 { ENCODE_TABLE[b3] as char } else { PAD as char });
        result.push(if chunk.len() > 2 { ENCODE_TABLE[b4] as char } else { PAD as char });
    }

    result
}

/// Decodes base64 string to bytes
pub fn decode(encoded: &str) -> Result<Vec<u8>> {
    decode_bytes(encoded.as_bytes())
}

/// Decodes base64 bytes to data
pub fn decode_bytes(encoded: &[u8]) -> Result<Vec<u8>> {
    if encoded.len() % 4 != 0 {
        return Err(Error::new(ErrorKind::InvalidInput, "Base64 length must be multiple of 4"));
    }

    let mut result = Vec::with_capacity(encoded.len() * 3 / 4);
    for chunk in encoded.chunks(4) {
        let b1 = decode_base64_char(chunk[0])?;
        let b2 = decode_base64_char(chunk[1])?;
        let b3 = if chunk[2] == PAD { 0 } else { decode_base64_char(chunk[2])? };
        let b4 = if chunk[3] == PAD { 0 } else { decode_base64_char(chunk[3])? };

        result.push((b1 << 2) | (b2 >> 4));
        if chunk[2] != PAD {
            result.push((b2 << 4) | (b3 >> 2));
        }
        if chunk[3] != PAD {
            result.push((b3 << 6) | b4);
        }
    }

    Ok(result)
}

fn decode_base64_char(c: u8) -> Result<u8> {
    match c {
        b'A'..=b'Z' => Ok(c - b'A'),
        b'a'..=b'z' => Ok(c - b'a' + 26),
        b'0'..=b'9' => Ok(c - b'0' + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid base64 character")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        assert_eq!(encode(b"Hello"), "SGVsbG8=");
        assert_eq!(encode(b""), "");
        assert_eq!(encode(b"A"), "QQ==");
    }

    #[test]
    fn test_base64_decode() {
        assert_eq!(decode("SGVsbG8=").unwrap(), b"Hello");
        assert_eq!(decode("").unwrap(), b"");
        assert_eq!(decode("QQ==").unwrap(), b"A");
    }

    #[test]
    fn test_base64_roundtrip() {
        let data = b"The quick brown fox";
        let encoded = encode(data);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }
}
