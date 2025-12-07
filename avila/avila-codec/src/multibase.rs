//! Multibase encoding/decoding (IPFS style)
//!
//! Self-describing base encodings with prefix characters.

use crate::{base32, base58, base64, hex, Error, ErrorKind, Result};
use alloc::{string::String, vec::Vec};

/// Multibase encoding type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    /// Base16 (hexadecimal)
    Base16,
    /// Base16 uppercase
    Base16Upper,
    /// Base32 (RFC 4648)
    Base32,
    /// Base32 uppercase
    Base32Upper,
    /// Base58 (Bitcoin)
    Base58Btc,
    /// Base64 (RFC 4648)
    Base64,
    /// Base64 URL-safe
    Base64Url,
}

impl Encoding {
    /// Returns the multibase prefix character for this encoding
    pub fn prefix(&self) -> char {
        match self {
            Encoding::Base16 => 'f',
            Encoding::Base16Upper => 'F',
            Encoding::Base32 => 'b',
            Encoding::Base32Upper => 'B',
            Encoding::Base58Btc => 'z',
            Encoding::Base64 => 'm',
            Encoding::Base64Url => 'u',
        }
    }

    /// Parses encoding from multibase prefix character
    pub fn from_prefix(prefix: char) -> Result<Self> {
        match prefix {
            'f' => Ok(Encoding::Base16),
            'F' => Ok(Encoding::Base16Upper),
            'b' => Ok(Encoding::Base32),
            'B' => Ok(Encoding::Base32Upper),
            'z' => Ok(Encoding::Base58Btc),
            'm' => Ok(Encoding::Base64),
            'u' => Ok(Encoding::Base64Url),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                "Unknown multibase prefix",
            )),
        }
    }
}

/// Encodes bytes to multibase string with specified encoding
pub fn encode(data: &[u8], encoding: Encoding) -> String {
    let mut result = String::new();
    result.push(encoding.prefix());

    let encoded = match encoding {
        Encoding::Base16 => hex::encode(data),
        Encoding::Base16Upper => hex::encode(data).to_uppercase(),
        Encoding::Base32 => base32::encode(data),
        Encoding::Base32Upper => base32::encode(data).to_uppercase(),
        Encoding::Base58Btc => base58::encode(data),
        Encoding::Base64 => base64::encode(data),
        Encoding::Base64Url => {
            // URL-safe base64: replace + with -, / with _, remove padding
            base64::encode(data)
                .replace('+', "-")
                .replace('/', "_")
                .trim_end_matches('=')
                .to_string()
        }
    };

    result.push_str(&encoded);
    result
}

/// Decodes multibase string (auto-detects encoding from prefix)
pub fn decode(encoded: &str) -> Result<Vec<u8>> {
    if encoded.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Empty multibase string",
        ));
    }

    let mut chars = encoded.chars();
    let prefix = chars.next().unwrap();
    let encoding = Encoding::from_prefix(prefix)?;
    let data = chars.as_str();

    match encoding {
        Encoding::Base16 | Encoding::Base16Upper => hex::decode(data),
        Encoding::Base32 | Encoding::Base32Upper => base32::decode(data),
        Encoding::Base58Btc => base58::decode(data),
        Encoding::Base64 => base64::decode(data),
        Encoding::Base64Url => {
            // Convert URL-safe back to standard base64
            let mut standard = data.replace('-', "+").replace('_', "/");
            // Add padding
            match standard.len() % 4 {
                2 => standard.push_str("=="),
                3 => standard.push('='),
                _ => {}
            }
            base64::decode(&standard)
        }
    }
}

/// Encodes bytes to multibase base58btc (most common IPFS encoding)
pub fn encode_base58btc(data: &[u8]) -> String {
    encode(data, Encoding::Base58Btc)
}

/// Encodes bytes to multibase base32 (case-insensitive)
pub fn encode_base32(data: &[u8]) -> String {
    encode(data, Encoding::Base32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let data = b"Hello, World!";

        let encoded = encode(data, Encoding::Base58Btc);
        assert!(encoded.starts_with('z'));
        assert_eq!(decode(&encoded).unwrap(), data);

        let encoded = encode(data, Encoding::Base32);
        assert!(encoded.starts_with('b'));
        assert_eq!(decode(&encoded).unwrap(), data);

        let encoded = encode(data, Encoding::Base16);
        assert!(encoded.starts_with('f'));
        assert_eq!(decode(&encoded).unwrap(), data);
    }

    #[test]
    fn test_prefix_detection() {
        assert_eq!(Encoding::from_prefix('z').unwrap(), Encoding::Base58Btc);
        assert_eq!(Encoding::from_prefix('b').unwrap(), Encoding::Base32);
        assert_eq!(Encoding::from_prefix('f').unwrap(), Encoding::Base16);
        assert!(Encoding::from_prefix('x').is_err());
    }

    #[test]
    fn test_base58btc() {
        let data = b"test data";
        let encoded = encode_base58btc(data);
        assert!(encoded.starts_with('z'));
        assert_eq!(decode(&encoded).unwrap(), data);
    }

    #[test]
    fn test_roundtrip_all_encodings() {
        let data = b"The quick brown fox";

        for encoding in [
            Encoding::Base16,
            Encoding::Base32,
            Encoding::Base58Btc,
            Encoding::Base64,
        ] {
            let encoded = encode(data, encoding);
            let decoded = decode(&encoded).unwrap();
            assert_eq!(decoded, data);
        }
    }
}
