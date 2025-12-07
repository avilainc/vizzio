//! CRC (Cyclic Redundancy Check) checksums
//!
//! Fast error-detection codes.

/// CRC32 lookup table (IEEE polynomial)
const CRC32_TABLE: [u32; 256] = generate_crc32_table();

const fn generate_crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;

    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;

        while j < 8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
            j += 1;
        }

        table[i] = crc;
        i += 1;
    }

    table
}

/// Calculates CRC32 checksum (IEEE polynomial)
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFF;

    for &byte in data {
        let index = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_TABLE[index];
    }

    crc ^ 0xFFFFFFFF
}

/// Calculates CRC32 with initial value
pub fn crc32_with_initial(data: &[u8], initial: u32) -> u32 {
    let mut crc = initial ^ 0xFFFFFFFF;

    for &byte in data {
        let index = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_TABLE[index];
    }

    crc ^ 0xFFFFFFFF
}

/// CRC16 (XMODEM polynomial)
pub fn crc16(data: &[u8]) -> u16 {
    let mut crc = 0u16;

    for &byte in data {
        crc ^= (byte as u16) << 8;

        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }

    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32() {
        assert_eq!(crc32(b""), 0);
        assert_eq!(crc32(b"The quick brown fox jumps over the lazy dog"), 0x414FA339);
        assert_eq!(crc32(b"123456789"), 0xCBF43926);
    }

    #[test]
    fn test_crc32_incremental() {
        let data1 = b"Hello, ";
        let data2 = b"World!";

        let crc1 = crc32(data1);
        let crc_combined = crc32_with_initial(data2, crc1);

        let mut full_data = Vec::new();
        full_data.extend_from_slice(data1);
        full_data.extend_from_slice(data2);
        let crc_full = crc32(&full_data);

        assert_eq!(crc_combined, crc_full);
    }

    #[test]
    fn test_crc16() {
        assert_eq!(crc16(b"123456789"), 0x31C3);
    }
}
