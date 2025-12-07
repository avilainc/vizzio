//! LZ77-style compression (simplified)

use alloc::vec::Vec;
use avila_error::{Error, ErrorKind, Result};

const WINDOW_SIZE: usize = 4096;
const LOOKAHEAD_SIZE: usize = 18;
const MIN_MATCH: usize = 3;

pub struct Lz77Compressor {
    window_size: usize,
    lookahead_size: usize,
    min_match: usize,
}

impl Lz77Compressor {
    pub fn new() -> Self {
        Self {
            window_size: WINDOW_SIZE,
            lookahead_size: LOOKAHEAD_SIZE,
            min_match: MIN_MATCH,
        }
    }

    pub fn compress(&self, input: &[u8]) -> Vec<u8> {
        let mut output = Vec::new();
        let mut pos = 0;

        while pos < input.len() {
            let (match_dist, match_len) = self.find_longest_match(input, pos);

            if match_len >= self.min_match {
                output.push(0xFF);
                output.extend_from_slice(&(match_dist as u16).to_le_bytes());
                output.push(match_len as u8);
                pos += match_len;
            } else {
                output.push(input[pos]);
                if input[pos] == 0xFF {
                    output.push(0x00);
                }
                pos += 1;
            }
        }

        output
    }

    fn find_longest_match(&self, input: &[u8], pos: usize) -> (usize, usize) {
        let window_start = pos.saturating_sub(self.window_size);
        let lookahead_end = (pos + self.lookahead_size).min(input.len());

        let mut best_dist = 0;
        let mut best_len = 0;

        for search_pos in window_start..pos {
            let mut match_len = 0;

            while pos + match_len < lookahead_end
                && input[search_pos + match_len] == input[pos + match_len]
            {
                match_len += 1;
            }

            if match_len > best_len {
                best_len = match_len;
                best_dist = pos - search_pos;
            }
        }

        (best_dist, best_len)
    }

    pub fn decompress(&self, input: &[u8]) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        let mut pos = 0;

        while pos < input.len() {
            if input[pos] == 0xFF {
                if pos + 3 >= input.len() {
                    return Err(Error::new(ErrorKind::InvalidInput, "Truncated match"));
                }

                let dist = u16::from_le_bytes([input[pos + 1], input[pos + 2]]) as usize;
                let len = input[pos + 3] as usize;

                if dist == 0 && len == 0 {
                    output.push(0xFF);
                    pos += 4;
                    continue;
                }

                if dist > output.len() {
                    return Err(Error::new(ErrorKind::InvalidInput, "Invalid distance"));
                }

                let copy_start = output.len() - dist;
                for i in 0..len {
                    let byte = output[copy_start + i];
                    output.push(byte);
                }

                pos += 4;
            } else {
                output.push(input[pos]);
                pos += 1;
            }
        }

        Ok(output)
    }
}

impl Default for Lz77Compressor {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LzssCompressor;

impl LzssCompressor {
    pub fn compress(input: &[u8]) -> Vec<u8> {
        let mut output = Vec::new();
        let mut pos = 0;
        let window_size = 4096;
        let lookahead = 18;

        while pos < input.len() {
            let (dist, len) = Self::find_match(input, pos, window_size, lookahead);

            if len >= 3 {
                output.push(0x01);
                output.extend_from_slice(&(dist as u16).to_le_bytes());
                output.push(len as u8);
                pos += len;
            } else {
                output.push(0x00);
                output.push(input[pos]);
                pos += 1;
            }
        }

        output
    }

    fn find_match(input: &[u8], pos: usize, window: usize, lookahead: usize) -> (usize, usize) {
        let start = pos.saturating_sub(window);
        let end = (pos + lookahead).min(input.len());
        let mut best = (0, 0);

        for i in start..pos {
            let mut len = 0;
            while pos + len < end && input[i + len] == input[pos + len] {
                len += 1;
            }
            if len > best.1 {
                best = (pos - i, len);
            }
        }

        best
    }

    pub fn decompress(input: &[u8]) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        let mut pos = 0;

        while pos < input.len() {
            let flag = input[pos];
            pos += 1;

            if flag == 0x01 {
                if pos + 3 > input.len() {
                    return Err(Error::new(ErrorKind::InvalidInput, "Truncated"));
                }

                let dist = u16::from_le_bytes([input[pos], input[pos + 1]]) as usize;
                let len = input[pos + 2] as usize;
                pos += 3;

                let copy_start = output.len() - dist;
                for i in 0..len {
                    let byte = output[copy_start + i];
                    output.push(byte);
                }
            } else {
                if pos >= input.len() {
                    return Err(Error::new(ErrorKind::InvalidInput, "Truncated"));
                }
                output.push(input[pos]);
                pos += 1;
            }
        }

        Ok(output)
    }
}

pub struct SnappyLike;

impl SnappyLike {
    pub fn compress(input: &[u8]) -> Vec<u8> {
        let mut output = Vec::new();

        output.extend_from_slice(&(input.len() as u32).to_le_bytes());

        let mut pos = 0;
        while pos < input.len() {
            let (dist, len) = Self::find_match(input, pos);

            if len >= 4 {
                let copy_offset = pos - dist;
                output.push(0b01000000 | ((len - 4) as u8));
                output.extend_from_slice(&(copy_offset as u16).to_le_bytes());
                pos += len;
            } else {
                let literal_len = (input.len() - pos).min(60);
                output.push(0b00000000 | (literal_len as u8));
                output.extend_from_slice(&input[pos..pos + literal_len]);
                pos += literal_len;
            }
        }

        output
    }

    fn find_match(input: &[u8], pos: usize) -> (usize, usize) {
        let window_start = pos.saturating_sub(32768);
        let lookahead_end = (pos + 64).min(input.len());
        let mut best = (0, 0);

        for i in window_start..pos {
            let mut len = 0;
            while pos + len < lookahead_end && input[i + len] == input[pos + len] {
                len += 1;
                if len >= 64 {
                    break;
                }
            }
            if len > best.1 {
                best = (pos - i, len);
            }
        }

        best
    }

    pub fn decompress(input: &[u8]) -> Result<Vec<u8>> {
        if input.len() < 4 {
            return Err(Error::new(ErrorKind::InvalidInput, "Too short"));
        }

        let decompressed_len = u32::from_le_bytes([input[0], input[1], input[2], input[3]]) as usize;
        let mut output = Vec::with_capacity(decompressed_len);
        let mut pos = 4;

        while pos < input.len() {
            let tag = input[pos];
            pos += 1;

            if tag & 0b11000000 == 0b01000000 {
                let len = ((tag & 0b00111111) as usize) + 4;
                if pos + 2 > input.len() {
                    return Err(Error::new(ErrorKind::InvalidInput, "Truncated"));
                }
                let offset = u16::from_le_bytes([input[pos], input[pos + 1]]) as usize;
                pos += 2;

                for i in 0..len {
                    let byte = output[offset + i];
                    output.push(byte);
                }
            } else {
                let len = (tag & 0b00111111) as usize;
                if pos + len > input.len() {
                    return Err(Error::new(ErrorKind::InvalidInput, "Truncated"));
                }
                output.extend_from_slice(&input[pos..pos + len]);
                pos += len;
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lz77() {
        let compressor = Lz77Compressor::new();
        let data = b"ABABABABAB";

        let compressed = compressor.compress(data);
        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_lzss() {
        let data = b"Hello Hello Hello";
        let compressed = LzssCompressor::compress(data);
        let decompressed = LzssCompressor::decompress(&compressed).unwrap();

        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_snappy_like() {
        let data = b"AAAAAABBBBBBCCCCCC";
        let compressed = SnappyLike::compress(data);
        let decompressed = SnappyLike::decompress(&compressed).unwrap();

        assert_eq!(decompressed, data);
    }
}
