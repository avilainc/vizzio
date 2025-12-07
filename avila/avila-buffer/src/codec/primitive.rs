//! Primitive type encoding/decoding

use crate::ByteBuffer;
use avila_error::Result;

/// Extension trait for encoding primitive types
pub trait PrimitiveEncoder {
    /// Write a u8
    fn write_u8(&mut self, value: u8) -> Result<()>;

    /// Write a u16 in little-endian
    fn write_u16_le(&mut self, value: u16) -> Result<()>;

    /// Write a u16 in big-endian
    fn write_u16_be(&mut self, value: u16) -> Result<()>;

    /// Write a u32 in little-endian
    fn write_u32_le(&mut self, value: u32) -> Result<()>;

    /// Write a u32 in big-endian
    fn write_u32_be(&mut self, value: u32) -> Result<()>;

    /// Write a u64 in little-endian
    fn write_u64_le(&mut self, value: u64) -> Result<()>;

    /// Write a u64 in big-endian
    fn write_u64_be(&mut self, value: u64) -> Result<()>;
}

/// Extension trait for decoding primitive types
pub trait PrimitiveDecoder {
    /// Read a u8
    fn read_u8(&mut self) -> Result<u8>;

    /// Read a u16 in little-endian
    fn read_u16_le(&mut self) -> Result<u16>;

    /// Read a u16 in big-endian
    fn read_u16_be(&mut self) -> Result<u16>;

    /// Read a u32 in little-endian
    fn read_u32_le(&mut self) -> Result<u32>;

    /// Read a u32 in big-endian
    fn read_u32_be(&mut self) -> Result<u32>;

    /// Read a u64 in little-endian
    fn read_u64_le(&mut self) -> Result<u64>;

    /// Read a u64 in big-endian
    fn read_u64_be(&mut self) -> Result<u64>;
}

// Implementation to be added in Phase 2
impl PrimitiveEncoder for ByteBuffer {
    fn write_u8(&mut self, value: u8) -> Result<()> {
        self.write(&[value]).map(|_| ())
    }

    fn write_u16_le(&mut self, value: u16) -> Result<()> {
        self.write(&value.to_le_bytes()).map(|_| ())
    }

    fn write_u16_be(&mut self, value: u16) -> Result<()> {
        self.write(&value.to_be_bytes()).map(|_| ())
    }

    fn write_u32_le(&mut self, value: u32) -> Result<()> {
        self.write(&value.to_le_bytes()).map(|_| ())
    }

    fn write_u32_be(&mut self, value: u32) -> Result<()> {
        self.write(&value.to_be_bytes()).map(|_| ())
    }

    fn write_u64_le(&mut self, value: u64) -> Result<()> {
        self.write(&value.to_le_bytes()).map(|_| ())
    }

    fn write_u64_be(&mut self, value: u64) -> Result<()> {
        self.write(&value.to_be_bytes()).map(|_| ())
    }
}

impl PrimitiveDecoder for ByteBuffer {
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.read(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u16_le(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.read(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    fn read_u16_be(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.read(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn read_u32_le(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.read(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    fn read_u32_be(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.read(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    fn read_u64_le(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.read(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    fn read_u64_be(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.read(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }
}
