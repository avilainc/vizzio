//! Quoted-Printable encoding (RFC 2045)
//!
//! Email-safe encoding for text with mostly ASCII characters.

use crate::{Error, ErrorKind, Result};
use alloc::{string::String, vec::Vec};

const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";
const MAX_LINE_LENGTH: usize = 76;

/// Encodes bytes to quoted-printable format
pub fn encode(data: &[u8]) -> String {
    let mut result = String::with_capacity(data.len());
    let mut line_length = 0;

    for &byte in data {
        if needs_encoding(byte) {
            // Check if we need to add soft line break
            if line_length + 3 > MAX_LINE_LENGTH {
                result.push_str("=\r\n");
                line_length = 0;
            }

            result.push('=');
            result.push(HEX_CHARS[(byte >> 4) as usize] as char);
            result.push(HEX_CHARS[(byte & 0x0f) as usize] as char);
            line_length += 3;
        } else {
            // Check for line break
            if byte == b'\r' || byte == b'\n' {
                result.push(byte as char);
                line_length = 0;
            } else {
                if line_length + 1 > MAX_LINE_LENGTH {
                    result.push_str("=\r\n");
                    line_length = 0;
                }

                result.push(byte as char);
                line_length += 1;
            }
        }
    }

    result
}

/// Decodes quoted-printable string to bytes
pub fn decode(encoded: &str) -> Result<Vec<u8>> {
    let mut result = Vec::new();
    let bytes = encoded.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'=' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'\r' {
                // Soft line break: =\r\n
                i += 2;
                if i < bytes.len() && bytes[i] == b'\n' {
                    i += 1;
                }
                continue;
            } else if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                // Soft line break: =\n
                i += 2;
                continue;
            } else if i + 2 < bytes.len() {
                // Encoded byte
                let high = decode_hex_char(bytes[i + 1])?;
                let low = decode_hex_char(bytes[i + 2])?;
                result.push((high << 4) | low);
                i += 3;
            } else {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Incomplete quoted-printable sequence",
                ));
            }
        } else {
            result.push(bytes[i]);
            i += 1;
        }
    }

    Ok(result)
}

fn needs_encoding(byte: u8) -> bool {
    // Encode if not printable ASCII (33-126) or whitespace at end of line
    byte < 33 || byte > 126 || byte == b'='
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
    fn test_encode_simple() {
        assert_eq!(encode(b"hello"), "hello");
        assert_eq!(encode(b""), "");
    }

    #[test]
    fn test_encode_special() {
        assert_eq!(encode(b"hello=world"), "hello=3Dworld");
        assert_eq!(encode(b"test\xFF\xFE"), "test=FF=FE");
    }

    #[test]
    fn test_decode_simple() {
        assert_eq!(decode("hello").unwrap(), b"hello");
        assert_eq!(decode("hello=3Dworld").unwrap(), b"hello=world");
    }

    #[test]
    fn test_roundtrip() {
        let data = b"The quick brown fox jumps over the lazy dog!";
        let encoded = encode(data);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_soft_line_break() {
        // Test decoding soft line breaks
        assert_eq!(decode("hello=\r\nworld").unwrap(), b"helloworld");
        assert_eq!(decode("test=\ndata").unwrap(), b"testdata");
    }

    #[test]
    fn test_binary_data() {
        let data = b"\x00\x01\x02\xFF\xFE\xFD";
        let encoded = encode(data);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }
}
