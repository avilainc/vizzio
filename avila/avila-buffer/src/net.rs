//! Network byte order utilities and packet handling

use alloc::vec::Vec;
use avila_error::{Error, ErrorKind, Result};

pub struct PacketBuilder {
    buffer: Vec<u8>,
}

impl PacketBuilder {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
        }
    }

    pub fn write_u8(&mut self, value: u8) -> &mut Self {
        self.buffer.push(value);
        self
    }

    pub fn write_u16_be(&mut self, value: u16) -> &mut Self {
        self.buffer.extend_from_slice(&value.to_be_bytes());
        self
    }

    pub fn write_u16_le(&mut self, value: u16) -> &mut Self {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        self
    }

    pub fn write_u32_be(&mut self, value: u32) -> &mut Self {
        self.buffer.extend_from_slice(&value.to_be_bytes());
        self
    }

    pub fn write_u32_le(&mut self, value: u32) -> &mut Self {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        self
    }

    pub fn write_u64_be(&mut self, value: u64) -> &mut Self {
        self.buffer.extend_from_slice(&value.to_be_bytes());
        self
    }

    pub fn write_u64_le(&mut self, value: u64) -> &mut Self {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        self
    }

    pub fn write_bytes(&mut self, data: &[u8]) -> &mut Self {
        self.buffer.extend_from_slice(data);
        self
    }

    pub fn write_with_length(&mut self, data: &[u8]) -> &mut Self {
        self.write_u16_be(data.len() as u16);
        self.write_bytes(data);
        self
    }

    pub fn build(self) -> Vec<u8> {
        self.buffer
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

impl Default for PacketBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PacketReader<'a> {
    buffer: &'a [u8],
    position: usize,
}

impl<'a> PacketReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            position: 0,
        }
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        if self.position >= self.buffer.len() {
            return Err(Error::new(ErrorKind::OutOfBounds, "Not enough data"));
        }
        let value = self.buffer[self.position];
        self.position += 1;
        Ok(value)
    }

    pub fn read_u16_be(&mut self) -> Result<u16> {
        if self.position + 2 > self.buffer.len() {
            return Err(Error::new(ErrorKind::OutOfBounds, "Not enough data"));
        }
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&self.buffer[self.position..self.position + 2]);
        self.position += 2;
        Ok(u16::from_be_bytes(bytes))
    }

    pub fn read_u16_le(&mut self) -> Result<u16> {
        if self.position + 2 > self.buffer.len() {
            return Err(Error::new(ErrorKind::OutOfBounds, "Not enough data"));
        }
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&self.buffer[self.position..self.position + 2]);
        self.position += 2;
        Ok(u16::from_le_bytes(bytes))
    }

    pub fn read_u32_be(&mut self) -> Result<u32> {
        if self.position + 4 > self.buffer.len() {
            return Err(Error::new(ErrorKind::OutOfBounds, "Not enough data"));
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.buffer[self.position..self.position + 4]);
        self.position += 4;
        Ok(u32::from_be_bytes(bytes))
    }

    pub fn read_u32_le(&mut self) -> Result<u32> {
        if self.position + 4 > self.buffer.len() {
            return Err(Error::new(ErrorKind::OutOfBounds, "Not enough data"));
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.buffer[self.position..self.position + 4]);
        self.position += 4;
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn read_u64_be(&mut self) -> Result<u64> {
        if self.position + 8 > self.buffer.len() {
            return Err(Error::new(ErrorKind::OutOfBounds, "Not enough data"));
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.buffer[self.position..self.position + 8]);
        self.position += 8;
        Ok(u64::from_be_bytes(bytes))
    }

    pub fn read_u64_le(&mut self) -> Result<u64> {
        if self.position + 8 > self.buffer.len() {
            return Err(Error::new(ErrorKind::OutOfBounds, "Not enough data"));
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.buffer[self.position..self.position + 8]);
        self.position += 8;
        Ok(u64::from_le_bytes(bytes))
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        if self.position + len > self.buffer.len() {
            return Err(Error::new(ErrorKind::OutOfBounds, "Not enough data"));
        }
        let slice = &self.buffer[self.position..self.position + len];
        self.position += len;
        Ok(slice)
    }

    pub fn read_with_length(&mut self) -> Result<&'a [u8]> {
        let len = self.read_u16_be()? as usize;
        self.read_bytes(len)
    }

    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.position
    }

    pub fn is_empty(&self) -> bool {
        self.position >= self.buffer.len()
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

pub struct Frame {
    header: Vec<u8>,
    payload: Vec<u8>,
}

impl Frame {
    pub fn new(header: Vec<u8>, payload: Vec<u8>) -> Self {
        Self { header, payload }
    }

    pub fn header(&self) -> &[u8] {
        &self.header
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut result = self.header;
        result.extend_from_slice(&self.payload);
        result
    }

    pub fn total_len(&self) -> usize {
        self.header.len() + self.payload.len()
    }
}

pub struct FrameBuilder {
    max_payload_size: usize,
}

impl FrameBuilder {
    pub fn new(max_payload_size: usize) -> Self {
        Self { max_payload_size }
    }

    pub fn build_frame(&self, data: &[u8]) -> Vec<Frame> {
        let mut frames = Vec::new();
        let mut offset = 0;

        while offset < data.len() {
            let remaining = data.len() - offset;
            let chunk_size = remaining.min(self.max_payload_size);

            let mut header = Vec::new();
            header.extend_from_slice(&(chunk_size as u16).to_be_bytes());

            let payload = data[offset..offset + chunk_size].to_vec();
            frames.push(Frame::new(header, payload));

            offset += chunk_size;
        }

        frames
    }

    pub fn parse_frame(data: &[u8]) -> Result<(Frame, usize)> {
        if data.len() < 2 {
            return Err(Error::new(ErrorKind::InvalidInput, "Frame too short"));
        }

        let mut reader = PacketReader::new(data);
        let payload_len = reader.read_u16_be()? as usize;

        if reader.remaining() < payload_len {
            return Err(Error::new(ErrorKind::InvalidInput, "Incomplete frame"));
        }

        let header = data[..2].to_vec();
        let payload = reader.read_bytes(payload_len)?.to_vec();
        let total_consumed = 2 + payload_len;

        Ok((Frame::new(header, payload), total_consumed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_builder() {
        let packet = PacketBuilder::new()
            .write_u8(1)
            .write_u16_be(0x1234)
            .write_u32_be(0x12345678)
            .build();

        let mut reader = PacketReader::new(&packet);
        assert_eq!(reader.read_u8().unwrap(), 1);
        assert_eq!(reader.read_u16_be().unwrap(), 0x1234);
        assert_eq!(reader.read_u32_be().unwrap(), 0x12345678);
    }

    #[test]
    fn test_write_with_length() {
        let packet = PacketBuilder::new()
            .write_with_length(b"Hello")
            .build();

        let mut reader = PacketReader::new(&packet);
        let data = reader.read_with_length().unwrap();
        assert_eq!(data, b"Hello");
    }

    #[test]
    fn test_frame_builder() {
        let builder = FrameBuilder::new(10);
        let data = b"Hello World! This is a test.";
        let frames = builder.build_frame(data);

        assert!(frames.len() > 1);

        let mut reconstructed = Vec::new();
        for frame in frames {
            reconstructed.extend_from_slice(frame.payload());
        }

        assert_eq!(reconstructed, data);
    }

    #[test]
    fn test_parse_frame() {
        let frame_data = PacketBuilder::new()
            .write_u16_be(5)
            .write_bytes(b"Hello")
            .build();

        let (frame, consumed) = FrameBuilder::parse_frame(&frame_data).unwrap();
        assert_eq!(consumed, 7);
        assert_eq!(frame.payload(), b"Hello");
    }

    #[test]
    fn test_endianness() {
        let mut builder = PacketBuilder::new();
        builder.write_u16_be(0x1234);
        builder.write_u16_le(0x1234);

        let mut reader = PacketReader::new(&builder.buffer);
        assert_eq!(reader.read_u16_be().unwrap(), 0x1234);
        assert_eq!(reader.read_u16_le().unwrap(), 0x1234);
    }
}
