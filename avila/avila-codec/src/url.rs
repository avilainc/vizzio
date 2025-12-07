//! URL encoding/decoding (percent encoding)
//!
//! Implements RFC 3986 percent encoding for URLs.

use crate::{Error, ErrorKind, Result};
use alloc::{string::String, vec::Vec};

const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";

/// Encodes string to URL-safe percent-encoded string
pub fn encode(input: &str) -> String {
    encode_bytes(input.as_bytes())
}

/// Encodes bytes to URL-safe percent-encoded string
pub fn encode_bytes(data: &[u8]) -> String {
    let mut result = String::with_capacity(data.len() * 3);

    for &byte in data {
        if is_unreserved(byte) {
            result.push(byte as char);
        } else {
            result.push('%');
            result.push(HEX_CHARS[(byte >> 4) as usize] as char);
            result.push(HEX_CHARS[(byte & 0x0f) as usize] as char);
        }
    }

    result
}

/// Encodes string for use in URL path segments
pub fn encode_path(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 3);

    for &byte in input.as_bytes() {
        if is_path_char(byte) {
            result.push(byte as char);
        } else {
            result.push('%');
            result.push(HEX_CHARS[(byte >> 4) as usize] as char);
            result.push(HEX_CHARS[(byte & 0x0f) as usize] as char);
        }
    }

    result
}

/// Encodes string for use in query parameters (application/x-www-form-urlencoded)
pub fn encode_query(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 3);

    for &byte in input.as_bytes() {
        match byte {
            b' ' => result.push('+'),
            b if is_unreserved(b) || b == b'*' || b == b'-' || b == b'.' || b == b'_' => {
                result.push(b as char)
            }
            _ => {
                result.push('%');
                result.push(HEX_CHARS[(byte >> 4) as usize] as char);
                result.push(HEX_CHARS[(byte & 0x0f) as usize] as char);
            }
        }
    }

    result
}

/// Decodes percent-encoded URL string
pub fn decode(encoded: &str) -> Result<String> {
    let bytes = decode_bytes(encoded.as_bytes())?;
    String::from_utf8(bytes)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid UTF-8 in decoded URL"))
}

/// Decodes percent-encoded bytes
pub fn decode_bytes(encoded: &[u8]) -> Result<Vec<u8>> {
    let mut result = Vec::with_capacity(encoded.len());
    let mut i = 0;

    while i < encoded.len() {
        match encoded[i] {
            b'%' => {
                if i + 2 >= encoded.len() {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Incomplete percent encoding",
                    ));
                }

                let high = decode_hex_char(encoded[i + 1])?;
                let low = decode_hex_char(encoded[i + 2])?;
                result.push((high << 4) | low);
                i += 3;
            }
            b'+' => {
                result.push(b' ');
                i += 1;
            }
            byte => {
                result.push(byte);
                i += 1;
            }
        }
    }

    Ok(result)
}

fn is_unreserved(byte: u8) -> bool {
    matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~')
}

fn is_path_char(byte: u8) -> bool {
    is_unreserved(byte) || matches!(byte, b'/' | b':' | b'@' | b'!' | b'$' | b'&' | b'\'' | b'(' | b')' | b'*' | b'+' | b',' | b';' | b'=')
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
    fn test_encode() {
        assert_eq!(encode("hello world"), "hello%20world");
        assert_eq!(encode("hello+world"), "hello%2Bworld");
        assert_eq!(encode("hello@example.com"), "hello%40example.com");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("hello%20world").unwrap(), "hello world");
        assert_eq!(decode("hello+world").unwrap(), "hello world");
        assert_eq!(decode("hello%40example.com").unwrap(), "hello@example.com");
    }

    #[test]
    fn test_roundtrip() {
        let text = "The quick brown fox jumps!";
        let encoded = encode(text);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, text);
    }

    #[test]
    fn test_query_encoding() {
        assert_eq!(encode_query("hello world"), "hello+world");
        assert_eq!(encode_query("key=value&foo=bar"), "key%3Dvalue%26foo%3Dbar");
    }
}
