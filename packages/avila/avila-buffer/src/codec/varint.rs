//! Variable-length integer encoding (LEB128/varint)
//!
//! LEB128 encoding:
//! - Each byte uses 7 bits for data and 1 bit as continuation flag
//! - Values 0-127 fit in 1 byte
//! - Values 128+ use multiple bytes

use crate::ByteBuffer;
use avila_error::{Error, ErrorKind, Result};

/// Extension trait for variable-length integer encoding
pub trait VarintEncoder {
    /// Write a variable-length unsigned integer (LEB128)
    fn write_varint_u64(&mut self, value: u64) -> Result<usize>;

    /// Write a variable-length signed integer (zigzag encoded)
    fn write_varint_i64(&mut self, value: i64) -> Result<usize>;

    /// Write a variable-length u32
    fn write_varint_u32(&mut self, value: u32) -> Result<usize>;

    /// Write a variable-length i32
    fn write_varint_i32(&mut self, value: i32) -> Result<usize>;
}

/// Extension trait for variable-length integer decoding
pub trait VarintDecoder {
    /// Read a variable-length unsigned integer
    fn read_varint_u64(&mut self) -> Result<u64>;

    /// Read a variable-length signed integer (zigzag encoded)
    fn read_varint_i64(&mut self) -> Result<i64>;

    /// Read a variable-length u32
    fn read_varint_u32(&mut self) -> Result<u32>;

    /// Read a variable-length i32
    fn read_varint_i32(&mut self) -> Result<i32>;
}

/// Zigzag encode a signed integer
#[inline]
fn zigzag_encode_i64(value: i64) -> u64 {
    ((value << 1) ^ (value >> 63)) as u64
}

/// Zigzag decode to signed integer
#[inline]
fn zigzag_decode_u64(value: u64) -> i64 {
    ((value >> 1) as i64) ^ (-((value & 1) as i64))
}

/// Zigzag encode i32
#[inline]
fn zigzag_encode_i32(value: i32) -> u32 {
    ((value << 1) ^ (value >> 31)) as u32
}

/// Zigzag decode to i32
#[inline]
fn zigzag_decode_u32(value: u32) -> i32 {
    ((value >> 1) as i32) ^ (-((value & 1) as i32))
}

impl VarintEncoder for ByteBuffer {
    fn write_varint_u64(&mut self, mut value: u64) -> Result<usize> {
        let mut bytes_written = 0;

        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;

            if value != 0 {
                byte |= 0x80; // Set continuation bit
            }

            self.write(&[byte])?;
            bytes_written += 1;

            if value == 0 {
                break;
            }
        }

        Ok(bytes_written)
    }

    fn write_varint_i64(&mut self, value: i64) -> Result<usize> {
        let encoded = zigzag_encode_i64(value);
        self.write_varint_u64(encoded)
    }

    fn write_varint_u32(&mut self, value: u32) -> Result<usize> {
        self.write_varint_u64(value as u64)
    }

    fn write_varint_i32(&mut self, value: i32) -> Result<usize> {
        let encoded = zigzag_encode_i32(value);
        self.write_varint_u32(encoded)
    }
}

impl VarintDecoder for ByteBuffer {
    fn read_varint_u64(&mut self) -> Result<u64> {
        let mut result = 0u64;
        let mut shift = 0;

        loop {
            if shift >= 64 {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Varint overflow: too many bytes",
                ));
            }

            let mut byte = [0u8; 1];
            if self.read(&mut byte)? == 0 {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Unexpected end of buffer",
                ));
            }

            let b = byte[0];
            result |= ((b & 0x7F) as u64) << shift;

            if (b & 0x80) == 0 {
                return Ok(result);
            }

            shift += 7;
        }
    }

    fn read_varint_i64(&mut self) -> Result<i64> {
        let encoded = self.read_varint_u64()?;
        Ok(zigzag_decode_u64(encoded))
    }

    fn read_varint_u32(&mut self) -> Result<u32> {
        let value = self.read_varint_u64()?;
        if value > u32::MAX as u64 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Varint value exceeds u32::MAX",
            ));
        }
        Ok(value as u32)
    }

    fn read_varint_i32(&mut self) -> Result<i32> {
        let encoded = self.read_varint_u32()?;
        Ok(zigzag_decode_u32(encoded))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint_u64_small() {
        let mut buf = ByteBuffer::new();

        // Small value (fits in 1 byte)
        let written = buf.write_varint_u64(42).unwrap();
        assert_eq!(written, 1);

        let value = buf.read_varint_u64().unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_varint_u64_large() {
        let mut buf = ByteBuffer::new();

        // Large value
        let written = buf.write_varint_u64(300).unwrap();
        assert_eq!(written, 2); // 300 requires 2 bytes

        let value = buf.read_varint_u64().unwrap();
        assert_eq!(value, 300);
    }

    #[test]
    fn test_varint_i64_zigzag() {
        let mut buf = ByteBuffer::new();

        // Test positive
        buf.write_varint_i64(123).unwrap();
        // Test negative
        buf.write_varint_i64(-456).unwrap();
        // Test zero
        buf.write_varint_i64(0).unwrap();

        assert_eq!(buf.read_varint_i64().unwrap(), 123);
        assert_eq!(buf.read_varint_i64().unwrap(), -456);
        assert_eq!(buf.read_varint_i64().unwrap(), 0);
    }

    #[test]
    fn test_varint_u32() {
        let mut buf = ByteBuffer::new();

        buf.write_varint_u32(12345).unwrap();
        assert_eq!(buf.read_varint_u32().unwrap(), 12345);
    }

    #[test]
    fn test_varint_i32() {
        let mut buf = ByteBuffer::new();

        buf.write_varint_i32(-999).unwrap();
        assert_eq!(buf.read_varint_i32().unwrap(), -999);
    }

    #[test]
    fn test_zigzag_encoding() {
        // Positive numbers
        assert_eq!(zigzag_encode_i64(0), 0);
        assert_eq!(zigzag_encode_i64(1), 2);
        assert_eq!(zigzag_encode_i64(2), 4);

        // Negative numbers
        assert_eq!(zigzag_encode_i64(-1), 1);
        assert_eq!(zigzag_encode_i64(-2), 3);

        // Round-trip
        for val in [-1000, -1, 0, 1, 1000] {
            let encoded = zigzag_encode_i64(val);
            assert_eq!(zigzag_decode_u64(encoded), val);
        }
    }

    #[test]
    fn test_varint_max_values() {
        let mut buf = ByteBuffer::new();

        buf.write_varint_u64(u64::MAX).unwrap();
        assert_eq!(buf.read_varint_u64().unwrap(), u64::MAX);
    }
}
