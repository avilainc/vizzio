//! Base64 encoding and decoding
//! Pure Rust implementation following RFC 4648

use alloc::vec::Vec;
use alloc::string::String;
use avila_error::{Error, ErrorKind, Result};

/// Standard Base64 alphabet
const STANDARD_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// URL-safe Base64 alphabet
const URL_SAFE_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// Base64 encoder/decoder
pub struct Base64 {
    alphabet: &'static [u8; 64],
    decode_table: [u8; 256],
    padding: bool,
}

impl Base64 {
    /// Creates standard Base64 encoder
    pub fn standard() -> Self {
        Self::new(STANDARD_ALPHABET, true)
    }

    /// Creates URL-safe Base64 encoder
    pub fn url_safe() -> Self {
        Self::new(URL_SAFE_ALPHABET, false)
    }

    /// Creates custom Base64 encoder
    fn new(alphabet: &'static [u8; 64], padding: bool) -> Self {
        let mut decode_table = [0xFF; 256];

        for (i, &byte) in alphabet.iter().enumerate() {
            decode_table[byte as usize] = i as u8;
        }

        Self {
            alphabet,
            decode_table,
            padding,
        }
    }

    /// Encodes bytes to Base64 string
    pub fn encode(&self, input: &[u8]) -> String {
        let mut result = Vec::with_capacity((input.len() + 2) / 3 * 4);

        let mut i = 0;
        while i + 2 < input.len() {
            let b1 = input[i];
            let b2 = input[i + 1];
            let b3 = input[i + 2];

            result.push(self.alphabet[(b1 >> 2) as usize]);
            result.push(self.alphabet[(((b1 & 0x03) << 4) | (b2 >> 4)) as usize]);
            result.push(self.alphabet[(((b2 & 0x0F) << 2) | (b3 >> 6)) as usize]);
            result.push(self.alphabet[(b3 & 0x3F) as usize]);

            i += 3;
        }

        // Handle remaining bytes
        match input.len() - i {
            1 => {
                let b1 = input[i];
                result.push(self.alphabet[(b1 >> 2) as usize]);
                result.push(self.alphabet[((b1 & 0x03) << 4) as usize]);
                if self.padding {
                    result.push(b'=');
                    result.push(b'=');
                }
            }
            2 => {
                let b1 = input[i];
                let b2 = input[i + 1];
                result.push(self.alphabet[(b1 >> 2) as usize]);
                result.push(self.alphabet[(((b1 & 0x03) << 4) | (b2 >> 4)) as usize]);
                result.push(self.alphabet[((b2 & 0x0F) << 2) as usize]);
                if self.padding {
                    result.push(b'=');
                }
            }
            _ => {}
        }

        String::from_utf8(result).unwrap()
    }

    /// Decodes Base64 string to bytes
    pub fn decode(&self, input: &str) -> Result<Vec<u8>> {
        let input = input.as_bytes();
        let mut input_len = input.len();

        // Remove padding
        while input_len > 0 && input[input_len - 1] == b'=' {
            input_len -= 1;
        }

        let output_len = (input_len * 3) / 4;
        let mut result = Vec::with_capacity(output_len);

        let mut i = 0;
        while i + 3 < input_len {
            let b1 = self.decode_byte(input[i])?;
            let b2 = self.decode_byte(input[i + 1])?;
            let b3 = self.decode_byte(input[i + 2])?;
            let b4 = self.decode_byte(input[i + 3])?;

            result.push((b1 << 2) | (b2 >> 4));
            result.push((b2 << 4) | (b3 >> 2));
            result.push((b3 << 6) | b4);

            i += 4;
        }

        // Handle remaining bytes
        match input_len - i {
            2 => {
                let b1 = self.decode_byte(input[i])?;
                let b2 = self.decode_byte(input[i + 1])?;
                result.push((b1 << 2) | (b2 >> 4));
            }
            3 => {
                let b1 = self.decode_byte(input[i])?;
                let b2 = self.decode_byte(input[i + 1])?;
                let b3 = self.decode_byte(input[i + 2])?;
                result.push((b1 << 2) | (b2 >> 4));
                result.push((b2 << 4) | (b3 >> 2));
            }
            1 => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid Base64 length",
                ));
            }
            _ => {}
        }

        Ok(result)
    }

    /// Decodes single byte
    fn decode_byte(&self, byte: u8) -> Result<u8> {
        let value = self.decode_table[byte as usize];
        if value == 0xFF {
            Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid Base64 character",
            ))
        } else {
            Ok(value)
        }
    }

    /// Encodes to Base64 with line wrapping
    pub fn encode_wrapped(&self, input: &[u8], line_length: usize) -> String {
        let encoded = self.encode(input);
        let mut result = String::with_capacity(encoded.len() + encoded.len() / line_length);

        for (i, ch) in encoded.chars().enumerate() {
            if i > 0 && i % line_length == 0 {
                result.push('\n');
            }
            result.push(ch);
        }

        result
    }
}

/// Base64 encoding utilities
pub struct Base64Utils;

impl Base64Utils {
    /// Quick encode with standard alphabet
    pub fn encode(data: &[u8]) -> String {
        Base64::standard().encode(data)
    }

    /// Quick decode with standard alphabet
    pub fn decode(data: &str) -> Result<Vec<u8>> {
        Base64::standard().decode(data)
    }

    /// URL-safe encode
    pub fn encode_url_safe(data: &[u8]) -> String {
        Base64::url_safe().encode(data)
    }

    /// URL-safe decode
    pub fn decode_url_safe(data: &str) -> Result<Vec<u8>> {
        Base64::url_safe().decode(data)
    }
}

/// Base32 encoder (RFC 4648)
pub struct Base32 {
    alphabet: &'static [u8; 32],
    decode_table: [u8; 256],
    padding: bool,
}

impl Base32 {
    const ALPHABET: &'static [u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

    /// Creates standard Base32 encoder
    pub fn new() -> Self {
        let mut decode_table = [0xFF; 256];

        for (i, &byte) in Self::ALPHABET.iter().enumerate() {
            decode_table[byte as usize] = i as u8;
        }

        Self {
            alphabet: Self::ALPHABET,
            decode_table,
            padding: true,
        }
    }

    /// Encodes bytes to Base32
    pub fn encode(&self, input: &[u8]) -> String {
        let mut result = Vec::new();
        let mut bits = 0u64;
        let mut bit_count = 0;

        for &byte in input {
            bits = (bits << 8) | byte as u64;
            bit_count += 8;

            while bit_count >= 5 {
                bit_count -= 5;
                let index = ((bits >> bit_count) & 0x1F) as usize;
                result.push(self.alphabet[index]);
            }
        }

        // Handle remaining bits
        if bit_count > 0 {
            let index = ((bits << (5 - bit_count)) & 0x1F) as usize;
            result.push(self.alphabet[index]);
        }

        // Add padding
        if self.padding {
            while result.len() % 8 != 0 {
                result.push(b'=');
            }
        }

        String::from_utf8(result).unwrap()
    }

    /// Decodes Base32 string
    pub fn decode(&self, input: &str) -> Result<Vec<u8>> {
        let input = input.as_bytes();
        let mut result = Vec::new();
        let mut bits = 0u64;
        let mut bit_count = 0;

        for &byte in input {
            if byte == b'=' {
                break;
            }

            let value = self.decode_table[byte as usize];
            if value == 0xFF {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid Base32 character",
                ));
            }

            bits = (bits << 5) | value as u64;
            bit_count += 5;

            if bit_count >= 8 {
                bit_count -= 8;
                result.push((bits >> bit_count) as u8);
                bits &= (1 << bit_count) - 1;
            }
        }

        Ok(result)
    }
}

impl Default for Base32 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_standard() {
        let b64 = Base64::standard();

        let data = b"Hello, World!";
        let encoded = b64.encode(data);
        let decoded = b64.decode(&encoded).unwrap();

        assert_eq!(decoded, data);
    }

    #[test]
    fn test_base64_empty() {
        let b64 = Base64::standard();
        assert_eq!(b64.encode(b""), "");
        assert_eq!(b64.decode("").unwrap(), b"");
    }

    #[test]
    fn test_base64_all_lengths() {
        let b64 = Base64::standard();

        // 1 byte
        let data1 = b"A";
        let enc1 = b64.encode(data1);
        assert_eq!(b64.decode(&enc1).unwrap(), data1);

        // 2 bytes
        let data2 = b"AB";
        let enc2 = b64.encode(data2);
        assert_eq!(b64.decode(&enc2).unwrap(), data2);

        // 3 bytes
        let data3 = b"ABC";
        let enc3 = b64.encode(data3);
        assert_eq!(b64.decode(&enc3).unwrap(), data3);
    }

    #[test]
    fn test_base64_url_safe() {
        let b64 = Base64::url_safe();

        let data = b"\xFB\xFF\xBF";
        let encoded = b64.encode(data);

        // Should not contain + or /
        assert!(!encoded.contains('+'));
        assert!(!encoded.contains('/'));

        let decoded = b64.decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_base64_utils() {
        let data = b"Test data";
        let encoded = Base64Utils::encode(data);
        let decoded = Base64Utils::decode(&encoded).unwrap();

        assert_eq!(decoded, data);
    }

    #[test]
    fn test_base64_wrapped() {
        let b64 = Base64::standard();
        let data = b"A".repeat(100);
        let wrapped = b64.encode_wrapped(&data, 20);

        // Should contain newlines
        assert!(wrapped.contains('\n'));
    }

    #[test]
    fn test_base32() {
        let b32 = Base32::new();

        let data = b"Hello";
        let encoded = b32.encode(data);
        let decoded = b32.decode(&encoded).unwrap();

        assert_eq!(decoded, data);
    }

    #[test]
    fn test_base32_rfc_example() {
        let b32 = Base32::new();

        // RFC 4648 test vectors
        assert_eq!(b32.encode(b"f"), "MY======");
        assert_eq!(b32.encode(b"fo"), "MZXQ====");
        assert_eq!(b32.encode(b"foo"), "MZXW6===");
        assert_eq!(b32.encode(b"foob"), "MZXW6YQ=");
        assert_eq!(b32.encode(b"fooba"), "MZXW6YTB");
        assert_eq!(b32.encode(b"foobar"), "MZXW6YTBOI======");
    }

    #[test]
    fn test_base64_invalid() {
        let b64 = Base64::standard();
        assert!(b64.decode("!!!").is_err());
    }

    #[test]
    fn test_base32_invalid() {
        let b32 = Base32::new();
        assert!(b32.decode("!!!").is_err());
    }
}
