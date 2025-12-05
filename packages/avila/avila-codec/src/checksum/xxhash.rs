//! XXHash - Extremely fast non-cryptographic hash
//!
//! High-speed hash algorithm for hash tables and checksums.

const PRIME32_1: u32 = 2654435761;
const PRIME32_2: u32 = 2246822519;
const PRIME32_3: u32 = 3266489917;
const PRIME32_4: u32 = 668265263;
const PRIME32_5: u32 = 374761393;

/// Calculates XXHash32 with default seed (0)
pub fn xxhash32(data: &[u8]) -> u32 {
    xxhash32_with_seed(data, 0)
}

/// Calculates XXHash32 with custom seed
pub fn xxhash32_with_seed(data: &[u8], seed: u32) -> u32 {
    let len = data.len();
    let mut h32: u32;

    if len >= 16 {
        let mut v1 = seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2);
        let mut v2 = seed.wrapping_add(PRIME32_2);
        let mut v3 = seed;
        let mut v4 = seed.wrapping_sub(PRIME32_1);

        let mut i = 0;
        while i + 16 <= len {
            v1 = round32(v1, read_u32_le(&data[i..i + 4]));
            v2 = round32(v2, read_u32_le(&data[i + 4..i + 8]));
            v3 = round32(v3, read_u32_le(&data[i + 8..i + 12]));
            v4 = round32(v4, read_u32_le(&data[i + 12..i + 16]));
            i += 16;
        }

        h32 = v1.rotate_left(1)
            .wrapping_add(v2.rotate_left(7))
            .wrapping_add(v3.rotate_left(12))
            .wrapping_add(v4.rotate_left(18));

        // Process remaining bytes
        h32 = finalize32(h32, &data[i..], len as u32);
    } else {
        h32 = seed.wrapping_add(PRIME32_5);
        h32 = finalize32(h32, data, len as u32);
    }

    h32
}

fn round32(acc: u32, input: u32) -> u32 {
    acc.wrapping_add(input.wrapping_mul(PRIME32_2))
        .rotate_left(13)
        .wrapping_mul(PRIME32_1)
}

fn finalize32(mut h32: u32, data: &[u8], len: u32) -> u32 {
    h32 = h32.wrapping_add(len);

    let mut i = 0;
    while i + 4 <= data.len() {
        h32 = h32
            .wrapping_add(read_u32_le(&data[i..i + 4]).wrapping_mul(PRIME32_3))
            .rotate_left(17)
            .wrapping_mul(PRIME32_4);
        i += 4;
    }

    while i < data.len() {
        h32 = h32
            .wrapping_add((data[i] as u32).wrapping_mul(PRIME32_5))
            .rotate_left(11)
            .wrapping_mul(PRIME32_1);
        i += 1;
    }

    // Avalanche
    h32 ^= h32 >> 15;
    h32 = h32.wrapping_mul(PRIME32_2);
    h32 ^= h32 >> 13;
    h32 = h32.wrapping_mul(PRIME32_3);
    h32 ^= h32 >> 16;

    h32
}

fn read_u32_le(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xxhash32_empty() {
        assert_eq!(xxhash32(b""), 0x02CC5D05);
    }

    #[test]
    fn test_xxhash32_short() {
        assert_eq!(xxhash32(b"a"), 0x550D7456);
        assert_eq!(xxhash32(b"ab"), 0x4999FC53);
        assert_eq!(xxhash32(b"abc"), 0x32D153FF);
    }

    #[test]
    fn test_xxhash32_long() {
        let data = b"The quick brown fox jumps over the lazy dog";
        let hash = xxhash32(data);
        // Should produce consistent hash
        assert_eq!(hash, xxhash32(data));
    }

    #[test]
    fn test_xxhash32_with_seed() {
        let data = b"test";
        let hash1 = xxhash32_with_seed(data, 0);
        let hash2 = xxhash32_with_seed(data, 12345);
        // Different seeds should produce different hashes
        assert_ne!(hash1, hash2);
    }
}
