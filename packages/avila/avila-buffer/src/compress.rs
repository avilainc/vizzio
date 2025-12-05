//! Simple compression algorithms
//! No external dependencies - pure Rust implementations

use avila_error::{Error, ErrorKind, Result};

/// Run-Length Encoding (RLE) compression
///
/// Encodes sequences of identical bytes as (count, byte) pairs.
/// Good for data with many repeated values.
///
/// # Format
/// - For runs >= 3: 0xFF, count (1 byte), value
/// - For other bytes: raw value
pub struct RleCompressor;

impl RleCompressor {
    /// Compresses data using RLE
    pub fn compress(input: &[u8]) -> alloc::vec::Vec<u8> {
        use alloc::vec::Vec;

        if input.is_empty() {
            return Vec::new();
        }

        let mut output = Vec::with_capacity(input.len());
        let mut i = 0;

        while i < input.len() {
            let byte = input[i];
            let mut run_length = 1;

            // Count consecutive identical bytes
            while i + run_length < input.len() &&
                  input[i + run_length] == byte &&
                  run_length < 255 {
                run_length += 1;
            }

            if run_length >= 3 {
                // Encode as run: marker, count, value
                output.push(0xFF);
                output.push(run_length as u8);
                output.push(byte);
            } else {
                // Too short to encode, write raw bytes
                for _ in 0..run_length {
                    output.push(byte);
                    // Escape 0xFF by writing it twice
                    if byte == 0xFF {
                        output.push(0xFF);
                    }
                }
            }

            i += run_length;
        }

        output
    }

    /// Decompresses RLE data
    pub fn decompress(input: &[u8]) -> Result<alloc::vec::Vec<u8>> {
        use alloc::vec::Vec;

        let mut output = Vec::new();
        let mut i = 0;

        while i < input.len() {
            if input[i] == 0xFF {
                if i + 1 >= input.len() {
                    // Check if it's escaped 0xFF
                    output.push(0xFF);
                    i += 1;
                } else if input[i + 1] == 0xFF {
                    // Escaped 0xFF
                    output.push(0xFF);
                    i += 2;
                } else if i + 2 < input.len() {
                    // Run encoding: 0xFF, count, value
                    let count = input[i + 1] as usize;
                    let value = input[i + 2];

                    for _ in 0..count {
                        output.push(value);
                    }
                    i += 3;
                } else {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid RLE format",
                    ));
                }
            } else {
                output.push(input[i]);
                i += 1;
            }
        }

        Ok(output)
    }

    /// Calculates compression ratio
    pub fn compression_ratio(original_size: usize, compressed_size: usize) -> f32 {
        if original_size == 0 {
            return 0.0;
        }
        1.0 - (compressed_size as f32 / original_size as f32)
    }
}

/// Delta encoding - stores differences between consecutive values
/// Good for gradually changing values (sensor data, timestamps, etc.)
pub struct DeltaEncoder;

impl DeltaEncoder {
    /// Encodes using delta encoding
    pub fn encode(input: &[u8]) -> alloc::vec::Vec<u8> {
        use alloc::vec::Vec;

        if input.is_empty() {
            return Vec::new();
        }

        let mut output = Vec::with_capacity(input.len());
        output.push(input[0]); // First value as-is

        for i in 1..input.len() {
            let delta = input[i].wrapping_sub(input[i - 1]);
            output.push(delta);
        }

        output
    }

    /// Decodes delta encoded data
    pub fn decode(input: &[u8]) -> alloc::vec::Vec<u8> {
        use alloc::vec::Vec;

        if input.is_empty() {
            return Vec::new();
        }

        let mut output = Vec::with_capacity(input.len());
        output.push(input[0]);

        for i in 1..input.len() {
            let value = output[i - 1].wrapping_add(input[i]);
            output.push(value);
        }

        output
    }
}

/// Simple XOR cipher for data obfuscation
/// NOT cryptographically secure - just for simple obfuscation
pub struct XorCipher;

impl XorCipher {
    /// XOR encrypt/decrypt with key
    pub fn process(data: &[u8], key: &[u8]) -> alloc::vec::Vec<u8> {
        use alloc::vec::Vec;

        if key.is_empty() {
            return data.to_vec();
        }

        let mut result = Vec::with_capacity(data.len());

        for (i, &byte) in data.iter().enumerate() {
            let key_byte = key[i % key.len()];
            result.push(byte ^ key_byte);
        }

        result
    }
}

/// Zero-run compression - good for sparse data
/// Encodes runs of zeros efficiently
pub struct ZeroRunCompressor;

impl ZeroRunCompressor {
    /// Compresses by encoding zero runs
    pub fn compress(input: &[u8]) -> alloc::vec::Vec<u8> {
        use alloc::vec::Vec;

        if input.is_empty() {
            return Vec::new();
        }

        let mut output = Vec::new();
        let mut i = 0;

        while i < input.len() {
            if input[i] == 0 {
                // Count consecutive zeros
                let mut zero_count = 0;
                while i < input.len() && input[i] == 0 && zero_count < 255 {
                    zero_count += 1;
                    i += 1;
                }

                // Encode: 0x00, count
                output.push(0x00);
                output.push(zero_count as u8);
            } else {
                // Non-zero byte
                let byte = input[i];
                output.push(byte);
                // Escape 0x00 by writing marker
                if byte == 0x01 {
                    output.push(0x01); // Escape marker
                }
                i += 1;
            }
        }

        output
    }

    /// Decompresses zero-run data
    pub fn decompress(input: &[u8]) -> Result<alloc::vec::Vec<u8>> {
        use alloc::vec::Vec;

        let mut output = Vec::new();
        let mut i = 0;

        while i < input.len() {
            if input[i] == 0x00 {
                if i + 1 < input.len() {
                    let count = input[i + 1] as usize;
                    for _ in 0..count {
                        output.push(0);
                    }
                    i += 2;
                } else {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid zero-run format",
                    ));
                }
            } else if input[i] == 0x01 && i + 1 < input.len() && input[i + 1] == 0x01 {
                // Escaped 0x01
                output.push(0x01);
                i += 2;
            } else {
                output.push(input[i]);
                i += 1;
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_basic() {
        let input = b"AAABBBCCCCCC";
        let compressed = RleCompressor::compress(input);
        let decompressed = RleCompressor::decompress(&compressed).unwrap();

        assert_eq!(decompressed, input);
        assert!(compressed.len() < input.len());
    }

    #[test]
    fn test_rle_no_runs() {
        let input = b"ABCDEF";
        let compressed = RleCompressor::compress(input);
        let decompressed = RleCompressor::decompress(&compressed).unwrap();

        assert_eq!(decompressed, input);
    }

    #[test]
    fn test_rle_all_same() {
        let input = &[0x42; 100];
        let compressed = RleCompressor::compress(input);
        let decompressed = RleCompressor::decompress(&compressed).unwrap();

        assert_eq!(decompressed, input);
        assert!(compressed.len() < 10); // Very high compression
    }

    #[test]
    fn test_delta_encoding() {
        let input = &[10, 12, 15, 14, 20, 25];
        let encoded = DeltaEncoder::encode(input);
        let decoded = DeltaEncoder::decode(&encoded);

        assert_eq!(decoded, input);
    }

    #[test]
    fn test_xor_cipher() {
        let data = b"Secret message";
        let key = b"key123";

        let encrypted = XorCipher::process(data, key);
        let decrypted = XorCipher::process(&encrypted, key);

        assert_eq!(decrypted, data);
        assert_ne!(encrypted, data); // Should be different
    }

    #[test]
    fn test_zero_run() {
        let input = &[1, 2, 0, 0, 0, 0, 3, 4, 0, 0];
        let compressed = ZeroRunCompressor::compress(input);
        let decompressed = ZeroRunCompressor::decompress(&compressed).unwrap();

        assert_eq!(decompressed, input);
        assert!(compressed.len() < input.len());
    }

    #[test]
    fn test_compression_ratio() {
        let ratio = RleCompressor::compression_ratio(100, 50);
        assert_eq!(ratio, 0.5);
    }
}
