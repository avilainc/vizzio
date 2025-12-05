//! Hex encoding/decoding
//!
//! Fast hex encode/decode with constant-time operations.

use crate::{Error, ErrorKind, Result};
use alloc::{string::String, vec::Vec};

const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

/// Encodes bytes to hex string
pub fn encode(data: &[u8]) -> String {
    let mut result = String::with_capacity(data.len() * 2);
    for &byte in data {
        result.push(HEX_CHARS[(byte >> 4) as usize] as char);
        result.push(HEX_CHARS[(byte & 0x0f) as usize] as char);
    }
    result
}

/// Encodes bytes to hex (stack-allocated)
pub fn encode_to_slice(data: &[u8], output: &mut [u8]) -> Result<usize> {
    if output.len() < data.len() * 2 {
        return Err(Error::new(ErrorKind::InvalidInput, "Output buffer too small"));
    }

    for (i, &byte) in data.iter().enumerate() {
        output[i * 2] = HEX_CHARS[(byte >> 4) as usize];
        output[i * 2 + 1] = HEX_CHARS[(byte & 0x0f) as usize];
    }

    Ok(data.len() * 2)
}

/// Decodes hex string to bytes
pub fn decode(hex: &str) -> Result<Vec<u8>> {
    decode_bytes(hex.as_bytes())
}

/// Decodes hex bytes to data
pub fn decode_bytes(hex: &[u8]) -> Result<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return Err(Error::new(ErrorKind::InvalidInput, "Hex string must have even length"));
    }

    let mut result = Vec::with_capacity(hex.len() / 2);
    for chunk in hex.chunks(2) {
        let high = decode_hex_char(chunk[0])?;
        let low = decode_hex_char(chunk[1])?;
        result.push((high << 4) | low);
    }

    Ok(result)
}

/// Decodes hex to slice
pub fn decode_to_slice(hex: &[u8], output: &mut [u8]) -> Result<usize> {
    if hex.len() % 2 != 0 {
        return Err(Error::new(ErrorKind::InvalidInput, "Hex must have even length"));
    }

    let len = hex.len() / 2;
    if output.len() < len {
        return Err(Error::new(ErrorKind::InvalidInput, "Output buffer too small"));
    }

    for (i, chunk) in hex.chunks(2).enumerate() {
        let high = decode_hex_char(chunk[0])?;
        let low = decode_hex_char(chunk[1])?;
        output[i] = (high << 4) | low;
    }

    Ok(len)
}

fn decode_hex_char(c: u8) -> Result<u8> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid hex character")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_encode() {
        assert_eq!(encode(b"Hello"), "48656c6c6f");
        assert_eq!(encode(b""), "");
        assert_eq!(encode(b"\x00\xff"), "00ff");
    }

    #[test]
    fn test_hex_decode() {
        assert_eq!(decode("48656c6c6f").unwrap(), b"Hello");
        assert_eq!(decode("").unwrap(), b"");
        assert_eq!(decode("00ff").unwrap(), b"\x00\xff");
    }

    #[test]
    fn test_hex_roundtrip() {
        let data = b"The quick brown fox jumps over the lazy dog";
        let encoded = encode(data);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }
}
