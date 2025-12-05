//! LZ4 compression implementation
//!
//! Fast compression algorithm for cache and networking.
//!
//! This is a simplified LZ4 implementation that follows the basic block format:
//! - Token byte: 4 bits for literal length, 4 bits for match length
//! - Literals data
//! - Match offset (2 bytes, little-endian)
//! - Additional match length bytes if needed

use crate::{Error, ErrorKind, Result};
use alloc::vec::Vec;

/// Hash table size for finding matches
const HASH_SIZE: usize = 4096;

/// Minimum match length
const MIN_MATCH: usize = 4;

/// Maximum literal length in a single token
const MAX_LITERAL_LEN: usize = 15;

/// Maximum match length in a single token
const MAX_MATCH_LEN: usize = 15;

/// Compute hash for 4-byte sequence
#[inline]
fn hash4(data: &[u8], pos: usize) -> usize {
    if pos + 4 > data.len() {
        return 0;
    }
    let val = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
    ((val.wrapping_mul(2654435761)) >> 20) as usize & (HASH_SIZE - 1)
}

/// Find the length of matching bytes
#[inline]
fn match_length(data: &[u8], pos1: usize, pos2: usize, max_len: usize) -> usize {
    let mut len = 0;
    while len < max_len && pos1 + len < data.len() && pos2 + len < data.len() {
        if data[pos1 + len] != data[pos2 + len] {
            break;
        }
        len += 1;
    }
    len
}

/// Encode a variable-length integer (for lengths > 15)
fn encode_length(output: &mut Vec<u8>, mut len: usize) {
    while len >= 255 {
        output.push(255);
        len -= 255;
    }
    output.push(len as u8);
}

/// Decode a variable-length integer
fn decode_length(data: &[u8], pos: &mut usize) -> Result<usize> {
    let mut len = 0usize;
    loop {
        if *pos >= data.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Unexpected end of data"));
        }
        let byte = data[*pos];
        *pos += 1;
        len += byte as usize;
        if byte != 255 {
            break;
        }
    }
    Ok(len)
}

/// Compresses data using LZ4 algorithm
///
/// # Arguments
/// * `data` - Input data to compress
///
/// # Returns
/// Compressed data in LZ4 block format
///
/// # Example
/// ```
/// use avila_codec::compression::lz4;
///
/// let data = b"hello world hello world";
/// let compressed = lz4::compress(data).unwrap();
/// let decompressed = lz4::decompress(&compressed).unwrap();
/// assert_eq!(data, &decompressed[..]);
/// ```
pub fn compress(data: &[u8]) -> Result<Vec<u8>> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    let mut output = Vec::new();
    let mut hash_table = [0usize; HASH_SIZE];

    let mut pos = 0;
    let mut anchor = 0; // Start of current literal run

    while pos + MIN_MATCH <= data.len() {
        // Find a match using hash table
        let h = hash4(data, pos);
        let match_pos = hash_table[h];
        hash_table[h] = pos;

        // Check if we have a valid match
        let mut match_len = 0;
        if match_pos < pos && pos - match_pos < 65536 {
            match_len = match_length(data, pos, match_pos, data.len() - pos);
        }

        if match_len >= MIN_MATCH {
            // We found a match!
            let literal_len = pos - anchor;
            let offset = pos - match_pos;

            // Encode token
            let token_literal = if literal_len >= MAX_LITERAL_LEN {
                MAX_LITERAL_LEN
            } else {
                literal_len
            };
            let token_match = if match_len - MIN_MATCH >= MAX_MATCH_LEN {
                MAX_MATCH_LEN
            } else {
                match_len - MIN_MATCH
            };
            let token = ((token_literal << 4) | token_match) as u8;
            output.push(token);

            // Encode literal length if needed
            if literal_len >= MAX_LITERAL_LEN {
                encode_length(&mut output, literal_len - MAX_LITERAL_LEN);
            }

            // Copy literals
            output.extend_from_slice(&data[anchor..pos]);

            // Encode offset (2 bytes, little-endian)
            output.push((offset & 0xFF) as u8);
            output.push(((offset >> 8) & 0xFF) as u8);

            // Encode match length if needed
            if match_len - MIN_MATCH >= MAX_MATCH_LEN {
                encode_length(&mut output, match_len - MIN_MATCH - MAX_MATCH_LEN);
            }

            pos += match_len;
            anchor = pos;
        } else {
            pos += 1;
        }
    }

    // Encode remaining literals
    let literal_len = data.len() - anchor;
    if literal_len > 0 {
        let token_literal = if literal_len >= MAX_LITERAL_LEN {
            MAX_LITERAL_LEN
        } else {
            literal_len
        };
        let token = (token_literal << 4) as u8;
        output.push(token);

        if literal_len >= MAX_LITERAL_LEN {
            encode_length(&mut output, literal_len - MAX_LITERAL_LEN);
        }

        output.extend_from_slice(&data[anchor..]);
    }

    Ok(output)
}

/// Decompresses LZ4-compressed data
///
/// # Arguments
/// * `data` - LZ4-compressed data
///
/// # Returns
/// Decompressed original data
///
/// # Example
/// ```
/// use avila_codec::compression::lz4;
///
/// let original = b"test data test data";
/// let compressed = lz4::compress(original).unwrap();
/// let decompressed = lz4::decompress(&compressed).unwrap();
/// assert_eq!(original, &decompressed[..]);
/// ```
pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    let mut output = Vec::new();
    let mut pos = 0;

    while pos < data.len() {
        // Read token
        let token = data[pos];
        pos += 1;

        // Extract literal and match lengths from token
        let mut literal_len = ((token >> 4) & 0x0F) as usize;
        let mut match_len = (token & 0x0F) as usize;

        // Decode literal length if needed
        if literal_len == MAX_LITERAL_LEN {
            literal_len += decode_length(data, &mut pos)?;
        }

        // Copy literals
        if pos + literal_len > data.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid literal length"));
        }
        output.extend_from_slice(&data[pos..pos + literal_len]);
        pos += literal_len;

        // Check if we're at the end
        if pos >= data.len() {
            break;
        }

        // Read offset
        if pos + 2 > data.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid offset"));
        }
        let offset = data[pos] as usize | ((data[pos + 1] as usize) << 8);
        pos += 2;

        if offset == 0 || offset > output.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid match offset"));
        }

        // Decode match length if needed
        if match_len == MAX_MATCH_LEN {
            match_len += decode_length(data, &mut pos)?;
        }
        match_len += MIN_MATCH;

        // Copy match
        let match_pos = output.len() - offset;
        for i in 0..match_len {
            let byte = output[match_pos + i];
            output.push(byte);
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let data = b"";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, &decompressed[..]);
    }

    #[test]
    fn test_small_data() {
        let data = b"abc";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, &decompressed[..]);
    }

    #[test]
    fn test_repetitive_data() {
        let data = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, &decompressed[..]);
        // Compressed should be smaller
        assert!(compressed.len() < data.len());
    }

    #[test]
    fn test_hello_world() {
        let data = b"hello world hello world hello world";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, &decompressed[..]);
    }

    #[test]
    fn test_various_sizes() {
        for size in [1, 10, 100, 1000, 10000] {
            let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
            let compressed = compress(&data).unwrap();
            let decompressed = decompress(&compressed).unwrap();
            assert_eq!(data, decompressed);
        }
    }

    #[test]
    fn test_pattern() {
        let mut data = Vec::new();
        for _ in 0..100 {
            data.extend_from_slice(b"pattern");
        }
        let compressed = compress(&data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, decompressed);
        // Should compress well
        assert!(compressed.len() < data.len() / 2);
    }

    #[test]
    fn test_random_like() {
        let data: Vec<u8> = (0..256).map(|i| i as u8).collect();
        let compressed = compress(&data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, decompressed);
    }

    #[test]
    fn test_invalid_compressed_data() {
        let invalid = vec![0xFF, 0xFF, 0xFF];
        assert!(decompress(&invalid).is_err());
    }

    #[test]
    fn test_roundtrip_lorem() {
        let data = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
                     Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, &decompressed[..]);
    }
}
