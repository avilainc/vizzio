//! Variable-length integer encoding
//!
//! Compact integer representation for protocols and storage.

use crate::{Error, ErrorKind, Result};
use alloc::vec::Vec;

/// Encodes unsigned integer as LEB128 (Little Endian Base 128)
pub fn encode_leb128_u64(mut value: u64) -> Vec<u8> {
    let mut result = Vec::new();

    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;

        if value != 0 {
            byte |= 0x80;
        }

        result.push(byte);

        if value == 0 {
            break;
        }
    }

    result
}

/// Encodes signed integer as LEB128
pub fn encode_leb128_i64(mut value: i64) -> Vec<u8> {
    let mut result = Vec::new();
    let mut more = true;

    while more {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;

        if (value == 0 && (byte & 0x40) == 0) || (value == -1 && (byte & 0x40) != 0) {
            more = false;
        } else {
            byte |= 0x80;
        }

        result.push(byte);
    }

    result
}

/// Decodes LEB128 unsigned integer
pub fn decode_leb128_u64(data: &[u8]) -> Result<(u64, usize)> {
    let mut result = 0u64;
    let mut shift = 0;
    let mut i = 0;

    loop {
        if i >= data.len() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Incomplete LEB128 integer",
            ));
        }

        let byte = data[i];
        i += 1;

        result |= ((byte & 0x7F) as u64) << shift;

        if byte & 0x80 == 0 {
            break;
        }

        shift += 7;

        if shift >= 64 {
            return Err(Error::new(ErrorKind::InvalidInput, "LEB128 overflow"));
        }
    }

    Ok((result, i))
}

/// Decodes LEB128 signed integer
pub fn decode_leb128_i64(data: &[u8]) -> Result<(i64, usize)> {
    let mut result = 0i64;
    let mut shift = 0;
    let mut i = 0;
    let mut byte;

    loop {
        if i >= data.len() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Incomplete LEB128 integer",
            ));
        }

        byte = data[i];
        i += 1;

        result |= ((byte & 0x7F) as i64) << shift;
        shift += 7;

        if byte & 0x80 == 0 {
            break;
        }

        if shift >= 64 {
            return Err(Error::new(ErrorKind::InvalidInput, "LEB128 overflow"));
        }
    }

    // Sign extend
    if shift < 64 && (byte & 0x40) != 0 {
        result |= !0 << shift;
    }

    Ok((result, i))
}

/// Encodes unsigned integer as protobuf-style varint
pub fn encode_varint_u64(mut value: u64) -> Vec<u8> {
    let mut result = Vec::new();

    while value >= 0x80 {
        result.push((value & 0x7F | 0x80) as u8);
        value >>= 7;
    }

    result.push(value as u8);
    result
}

/// Decodes protobuf-style varint
pub fn decode_varint_u64(data: &[u8]) -> Result<(u64, usize)> {
    decode_leb128_u64(data)
}

/// Encodes signed integer using ZigZag encoding + varint
pub fn encode_zigzag_i64(value: i64) -> Vec<u8> {
    let encoded = ((value << 1) ^ (value >> 63)) as u64;
    encode_varint_u64(encoded)
}

/// Decodes ZigZag encoded signed integer
pub fn decode_zigzag_i64(data: &[u8]) -> Result<(i64, usize)> {
    let (encoded, len) = decode_varint_u64(data)?;
    let decoded = ((encoded >> 1) as i64) ^ (-((encoded & 1) as i64));
    Ok((decoded, len))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leb128_u64() {
        let test_cases = vec![0u64, 1, 127, 128, 255, 300, 16384, u64::MAX];

        for &value in &test_cases {
            let encoded = encode_leb128_u64(value);
            let (decoded, _) = decode_leb128_u64(&encoded).unwrap();
            assert_eq!(decoded, value);
        }
    }

    #[test]
    fn test_leb128_i64() {
        let test_cases = vec![-1i64, 0, 1, -64, 64, -128, 128, i64::MIN, i64::MAX];

        for &value in &test_cases {
            let encoded = encode_leb128_i64(value);
            let (decoded, _) = decode_leb128_i64(&encoded).unwrap();
            assert_eq!(decoded, value);
        }
    }

    #[test]
    fn test_varint_u64() {
        assert_eq!(encode_varint_u64(0), vec![0]);
        assert_eq!(encode_varint_u64(1), vec![1]);
        assert_eq!(encode_varint_u64(127), vec![127]);
        assert_eq!(encode_varint_u64(128), vec![0x80, 0x01]);
        assert_eq!(encode_varint_u64(300), vec![0xAC, 0x02]);
    }

    #[test]
    fn test_zigzag() {
        let test_cases = vec![-1i64, 0, 1, -2, 2, -100, 100, i64::MIN, i64::MAX];

        for &value in &test_cases {
            let encoded = encode_zigzag_i64(value);
            let (decoded, _) = decode_zigzag_i64(&encoded).unwrap();
            assert_eq!(decoded, value);
        }
    }

    #[test]
    fn test_zigzag_values() {
        assert_eq!(encode_zigzag_i64(0), vec![0]);
        assert_eq!(encode_zigzag_i64(-1), vec![1]);
        assert_eq!(encode_zigzag_i64(1), vec![2]);
        assert_eq!(encode_zigzag_i64(-2), vec![3]);
    }
}
