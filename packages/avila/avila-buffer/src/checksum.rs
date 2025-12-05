//! Checksum and hashing utilities
//! Pure Rust implementations without external dependencies

/// CRC-8 checksum calculator
pub struct Crc8 {
    table: [u8; 256],
}

impl Crc8 {
    /// Creates CRC-8 with standard polynomial (0x07)
    pub fn new() -> Self {
        Self::with_polynomial(0x07)
    }

    /// Creates CRC-8 with custom polynomial
    pub fn with_polynomial(poly: u8) -> Self {
        let mut table = [0u8; 256];

        for i in 0..256 {
            let mut crc = i as u8;
            for _ in 0..8 {
                if crc & 0x80 != 0 {
                    crc = (crc << 1) ^ poly;
                } else {
                    crc <<= 1;
                }
            }
            table[i] = crc;
        }

        Self { table }
    }

    /// Calculates CRC-8 checksum
    pub fn checksum(&self, data: &[u8]) -> u8 {
        let mut crc = 0u8;
        for &byte in data {
            crc = self.table[(crc ^ byte) as usize];
        }
        crc
    }

    /// Verifies data with checksum
    pub fn verify(&self, data: &[u8], expected: u8) -> bool {
        self.checksum(data) == expected
    }
}

impl Default for Crc8 {
    fn default() -> Self {
        Self::new()
    }
}

/// CRC-16 checksum calculator (CCITT)
pub struct Crc16 {
    table: [u16; 256],
}

impl Crc16 {
    /// Creates CRC-16 with CCITT polynomial (0x1021)
    pub fn new() -> Self {
        Self::with_polynomial(0x1021)
    }

    /// Creates CRC-16 with custom polynomial
    pub fn with_polynomial(poly: u16) -> Self {
        let mut table = [0u16; 256];

        for i in 0..256 {
            let mut crc = (i as u16) << 8;
            for _ in 0..8 {
                if crc & 0x8000 != 0 {
                    crc = (crc << 1) ^ poly;
                } else {
                    crc <<= 1;
                }
            }
            table[i] = crc;
        }

        Self { table }
    }

    /// Calculates CRC-16 checksum
    pub fn checksum(&self, data: &[u8]) -> u16 {
        let mut crc = 0xFFFFu16;
        for &byte in data {
            let idx = ((crc >> 8) ^ byte as u16) as u8;
            crc = (crc << 8) ^ self.table[idx as usize];
        }
        crc
    }

    /// Verifies data with checksum
    pub fn verify(&self, data: &[u8], expected: u16) -> bool {
        self.checksum(data) == expected
    }
}

impl Default for Crc16 {
    fn default() -> Self {
        Self::new()
    }
}

/// CRC-32 checksum calculator
pub struct Crc32 {
    table: [u32; 256],
}

impl Crc32 {
    /// Creates CRC-32 with IEEE polynomial (0xEDB88320)
    pub fn new() -> Self {
        Self::with_polynomial(0xEDB88320)
    }

    /// Creates CRC-32 with custom polynomial
    pub fn with_polynomial(poly: u32) -> Self {
        let mut table = [0u32; 256];

        for i in 0..256 {
            let mut crc = i as u32;
            for _ in 0..8 {
                if crc & 1 != 0 {
                    crc = (crc >> 1) ^ poly;
                } else {
                    crc >>= 1;
                }
            }
            table[i] = crc;
        }

        Self { table }
    }

    /// Calculates CRC-32 checksum
    pub fn checksum(&self, data: &[u8]) -> u32 {
        let mut crc = 0xFFFFFFFFu32;
        for &byte in data {
            let idx = ((crc ^ byte as u32) & 0xFF) as usize;
            crc = (crc >> 8) ^ self.table[idx];
        }
        !crc
    }

    /// Verifies data with checksum
    pub fn verify(&self, data: &[u8], expected: u32) -> bool {
        self.checksum(data) == expected
    }
}

impl Default for Crc32 {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple checksum algorithms
pub struct SimpleChecksum;

impl SimpleChecksum {
    /// Sum of all bytes (mod 256)
    pub fn sum8(data: &[u8]) -> u8 {
        data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b))
    }

    /// Sum of all bytes (mod 65536)
    pub fn sum16(data: &[u8]) -> u16 {
        data.iter().fold(0u16, |acc, &b| acc.wrapping_add(b as u16))
    }

    /// XOR of all bytes
    pub fn xor(data: &[u8]) -> u8 {
        data.iter().fold(0u8, |acc, &b| acc ^ b)
    }

    /// Fletcher-16 checksum
    pub fn fletcher16(data: &[u8]) -> u16 {
        let mut sum1 = 0u16;
        let mut sum2 = 0u16;

        for &byte in data {
            sum1 = (sum1 + byte as u16) % 255;
            sum2 = (sum2 + sum1) % 255;
        }

        (sum2 << 8) | sum1
    }

    /// Adler-32 checksum
    pub fn adler32(data: &[u8]) -> u32 {
        const MOD: u32 = 65521;
        let mut a = 1u32;
        let mut b = 0u32;

        for &byte in data {
            a = (a + byte as u32) % MOD;
            b = (b + a) % MOD;
        }

        (b << 16) | a
    }
}

/// FNV-1a hash (32-bit)
pub struct Fnv1a32;

impl Fnv1a32 {
    const OFFSET: u32 = 2166136261;
    const PRIME: u32 = 16777619;

    pub fn hash(data: &[u8]) -> u32 {
        let mut hash = Self::OFFSET;
        for &byte in data {
            hash ^= byte as u32;
            hash = hash.wrapping_mul(Self::PRIME);
        }
        hash
    }
}

/// FNV-1a hash (64-bit)
pub struct Fnv1a64;

impl Fnv1a64 {
    const OFFSET: u64 = 14695981039346656037;
    const PRIME: u64 = 1099511628211;

    pub fn hash(data: &[u8]) -> u64 {
        let mut hash = Self::OFFSET;
        for &byte in data {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(Self::PRIME);
        }
        hash
    }
}

/// DJB2 hash
pub struct Djb2;

impl Djb2 {
    pub fn hash(data: &[u8]) -> u32 {
        let mut hash = 5381u32;
        for &byte in data {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u32);
        }
        hash
    }
}

/// SDBM hash
pub struct Sdbm;

impl Sdbm {
    pub fn hash(data: &[u8]) -> u32 {
        let mut hash = 0u32;
        for &byte in data {
            hash = (byte as u32)
                .wrapping_add(hash.wrapping_shl(6))
                .wrapping_add(hash.wrapping_shl(16))
                .wrapping_sub(hash);
        }
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc8() {
        let crc = Crc8::new();
        let data = b"Hello World";
        let checksum = crc.checksum(data);

        assert!(crc.verify(data, checksum));
        assert!(!crc.verify(b"Hello World!", checksum));
    }

    #[test]
    fn test_crc16() {
        let crc = Crc16::new();
        let data = b"Test data";
        let checksum = crc.checksum(data);

        assert!(crc.verify(data, checksum));
    }

    #[test]
    fn test_crc32() {
        let crc = Crc32::new();
        let data = b"The quick brown fox";
        let checksum = crc.checksum(data);

        assert!(crc.verify(data, checksum));
    }

    #[test]
    fn test_simple_checksums() {
        let data = b"ABC";

        // Sum8: A(65) + B(66) + C(67) = 198
        assert_eq!(SimpleChecksum::sum8(data), 198);

        // XOR: 65 ^ 66 ^ 67 = 64
        assert_eq!(SimpleChecksum::xor(data), 64);
    }

    #[test]
    fn test_fletcher16() {
        let data = b"abcde";
        let checksum = SimpleChecksum::fletcher16(data);
        assert!(checksum > 0);
    }

    #[test]
    fn test_adler32() {
        let data = b"Wikipedia";
        let checksum = SimpleChecksum::adler32(data);
        assert_eq!(checksum, 0x11E60398); // Known value
    }

    #[test]
    fn test_fnv1a() {
        let data = b"Test";

        let hash32 = Fnv1a32::hash(data);
        let hash64 = Fnv1a64::hash(data);

        assert!(hash32 > 0);
        assert!(hash64 > 0);
    }

    #[test]
    fn test_djb2() {
        let data = b"hello";
        let hash = Djb2::hash(data);
        assert!(hash > 0);
    }

    #[test]
    fn test_hash_consistency() {
        let data = b"Same data";

        let hash1 = Fnv1a32::hash(data);
        let hash2 = Fnv1a32::hash(data);

        assert_eq!(hash1, hash2);
    }
}
