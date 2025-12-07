//! Protocol buffer-like encoding (simplified)

use alloc::vec::Vec;
use avila_error::{Error, ErrorKind, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireType {
    Varint = 0,
    Fixed64 = 1,
    LengthDelimited = 2,
    Fixed32 = 5,
}

impl WireType {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(WireType::Varint),
            1 => Ok(WireType::Fixed64),
            2 => Ok(WireType::LengthDelimited),
            5 => Ok(WireType::Fixed32),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid wire type")),
        }
    }
}

pub struct ProtoEncoder {
    buffer: Vec<u8>,
}

impl ProtoEncoder {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }

    pub fn encode_tag(&mut self, field_number: u32, wire_type: WireType) {
        let tag = (field_number << 3) | (wire_type as u32);
        self.encode_varint(tag as u64);
    }

    pub fn encode_varint(&mut self, mut value: u64) {
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            self.buffer.push(byte);
            if value == 0 {
                break;
            }
        }
    }

    pub fn encode_fixed32(&mut self, value: u32) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }

    pub fn encode_fixed64(&mut self, value: u64) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }

    pub fn encode_bytes(&mut self, data: &[u8]) {
        self.encode_varint(data.len() as u64);
        self.buffer.extend_from_slice(data);
    }

    pub fn encode_field_varint(&mut self, field: u32, value: u64) {
        self.encode_tag(field, WireType::Varint);
        self.encode_varint(value);
    }

    pub fn encode_field_fixed32(&mut self, field: u32, value: u32) {
        self.encode_tag(field, WireType::Fixed32);
        self.encode_fixed32(value);
    }

    pub fn encode_field_fixed64(&mut self, field: u32, value: u64) {
        self.encode_tag(field, WireType::Fixed64);
        self.encode_fixed64(value);
    }

    pub fn encode_field_bytes(&mut self, field: u32, data: &[u8]) {
        self.encode_tag(field, WireType::LengthDelimited);
        self.encode_bytes(data);
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.buffer
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.buffer
    }
}

impl Default for ProtoEncoder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ProtoDecoder<'a> {
    buffer: &'a [u8],
    position: usize,
}

impl<'a> ProtoDecoder<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            position: 0,
        }
    }

    pub fn decode_tag(&mut self) -> Result<(u32, WireType)> {
        let tag = self.decode_varint()? as u32;
        let field_number = tag >> 3;
        let wire_type = WireType::from_u8((tag & 0x07) as u8)?;
        Ok((field_number, wire_type))
    }

    pub fn decode_varint(&mut self) -> Result<u64> {
        let mut result = 0u64;
        let mut shift = 0;

        loop {
            if self.position >= self.buffer.len() {
                return Err(Error::new(ErrorKind::InvalidInput, "Unexpected end"));
            }

            let byte = self.buffer[self.position];
            self.position += 1;

            result |= ((byte & 0x7F) as u64) << shift;

            if byte & 0x80 == 0 {
                break;
            }

            shift += 7;
            if shift >= 64 {
                return Err(Error::new(ErrorKind::InvalidInput, "Varint too long"));
            }
        }

        Ok(result)
    }

    pub fn decode_fixed32(&mut self) -> Result<u32> {
        if self.position + 4 > self.buffer.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Not enough data"));
        }

        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.buffer[self.position..self.position + 4]);
        self.position += 4;

        Ok(u32::from_le_bytes(bytes))
    }

    pub fn decode_fixed64(&mut self) -> Result<u64> {
        if self.position + 8 > self.buffer.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Not enough data"));
        }

        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.buffer[self.position..self.position + 8]);
        self.position += 8;

        Ok(u64::from_le_bytes(bytes))
    }

    pub fn decode_bytes(&mut self) -> Result<&'a [u8]> {
        let len = self.decode_varint()? as usize;

        if self.position + len > self.buffer.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Not enough data"));
        }

        let bytes = &self.buffer[self.position..self.position + len];
        self.position += len;

        Ok(bytes)
    }

    pub fn skip_field(&mut self, wire_type: WireType) -> Result<()> {
        match wire_type {
            WireType::Varint => {
                self.decode_varint()?;
            }
            WireType::Fixed64 => {
                self.decode_fixed64()?;
            }
            WireType::LengthDelimited => {
                self.decode_bytes()?;
            }
            WireType::Fixed32 => {
                self.decode_fixed32()?;
            }
        }
        Ok(())
    }

    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.position
    }

    pub fn is_empty(&self) -> bool {
        self.position >= self.buffer.len()
    }
}

pub struct Message {
    fields: Vec<(u32, Field)>,
}

pub enum Field {
    Varint(u64),
    Fixed32(u32),
    Fixed64(u64),
    Bytes(Vec<u8>),
}

impl Message {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
        }
    }

    pub fn add_varint(&mut self, field: u32, value: u64) {
        self.fields.push((field, Field::Varint(value)));
    }

    pub fn add_fixed32(&mut self, field: u32, value: u32) {
        self.fields.push((field, Field::Fixed32(value)));
    }

    pub fn add_fixed64(&mut self, field: u32, value: u64) {
        self.fields.push((field, Field::Fixed64(value)));
    }

    pub fn add_bytes(&mut self, field: u32, value: Vec<u8>) {
        self.fields.push((field, Field::Bytes(value)));
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoder = ProtoEncoder::new();

        for (field_num, field) in &self.fields {
            match field {
                Field::Varint(v) => encoder.encode_field_varint(*field_num, *v),
                Field::Fixed32(v) => encoder.encode_field_fixed32(*field_num, *v),
                Field::Fixed64(v) => encoder.encode_field_fixed64(*field_num, *v),
                Field::Bytes(v) => encoder.encode_field_bytes(*field_num, v),
            }
        }

        encoder.into_vec()
    }

    pub fn decode(data: &[u8]) -> Result<Self> {
        let mut decoder = ProtoDecoder::new(data);
        let mut message = Message::new();

        while !decoder.is_empty() {
            let (field_num, wire_type) = decoder.decode_tag()?;

            match wire_type {
                WireType::Varint => {
                    let value = decoder.decode_varint()?;
                    message.add_varint(field_num, value);
                }
                WireType::Fixed32 => {
                    let value = decoder.decode_fixed32()?;
                    message.add_fixed32(field_num, value);
                }
                WireType::Fixed64 => {
                    let value = decoder.decode_fixed64()?;
                    message.add_fixed64(field_num, value);
                }
                WireType::LengthDelimited => {
                    let value = decoder.decode_bytes()?.to_vec();
                    message.add_bytes(field_num, value);
                }
            }
        }

        Ok(message)
    }

    pub fn get_varint(&self, field: u32) -> Option<u64> {
        self.fields.iter()
            .find(|(f, _)| *f == field)
            .and_then(|(_, v)| match v {
                Field::Varint(val) => Some(*val),
                _ => None,
            })
    }

    pub fn get_bytes(&self, field: u32) -> Option<&[u8]> {
        self.fields.iter()
            .find(|(f, _)| *f == field)
            .and_then(|(_, v)| match v {
                Field::Bytes(val) => Some(val.as_slice()),
                _ => None,
            })
    }
}

impl Default for Message {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_varint() {
        let mut encoder = ProtoEncoder::new();
        encoder.encode_varint(150);

        let mut decoder = ProtoDecoder::new(encoder.as_slice());
        assert_eq!(decoder.decode_varint().unwrap(), 150);
    }

    #[test]
    fn test_encode_tag() {
        let mut encoder = ProtoEncoder::new();
        encoder.encode_tag(1, WireType::Varint);
        encoder.encode_varint(42);

        let mut decoder = ProtoDecoder::new(encoder.as_slice());
        let (field, wire_type) = decoder.decode_tag().unwrap();

        assert_eq!(field, 1);
        assert_eq!(wire_type, WireType::Varint);
        assert_eq!(decoder.decode_varint().unwrap(), 42);
    }

    #[test]
    fn test_message() {
        let mut msg = Message::new();
        msg.add_varint(1, 100);
        msg.add_bytes(2, b"Hello".to_vec());

        let encoded = msg.encode();
        let decoded = Message::decode(&encoded).unwrap();

        assert_eq!(decoded.get_varint(1), Some(100));
        assert_eq!(decoded.get_bytes(2), Some(b"Hello".as_ref()));
    }

    #[test]
    fn test_fixed_types() {
        let mut encoder = ProtoEncoder::new();
        encoder.encode_field_fixed32(1, 0x12345678);
        encoder.encode_field_fixed64(2, 0x123456789ABCDEF0);

        let mut decoder = ProtoDecoder::new(encoder.as_slice());

        let (field1, _) = decoder.decode_tag().unwrap();
        assert_eq!(field1, 1);
        assert_eq!(decoder.decode_fixed32().unwrap(), 0x12345678);

        let (field2, _) = decoder.decode_tag().unwrap();
        assert_eq!(field2, 2);
        assert_eq!(decoder.decode_fixed64().unwrap(), 0x123456789ABCDEF0);
    }
}
