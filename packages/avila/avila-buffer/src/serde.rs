//! Serialization and deserialization utilities

use alloc::vec::Vec;
use alloc::string::String;
use avila_error::{Error, ErrorKind, Result};

pub trait Serialize {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<()>;
    fn serialized_size(&self) -> usize;
}

pub trait Deserialize: Sized {
    fn deserialize(buffer: &[u8]) -> Result<(Self, usize)>;
}

impl Serialize for u8 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.push(*self);
        Ok(())
    }

    fn serialized_size(&self) -> usize {
        1
    }
}

impl Deserialize for u8 {
    fn deserialize(buffer: &[u8]) -> Result<(Self, usize)> {
        if buffer.is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "Buffer too short"));
        }
        Ok((buffer[0], 1))
    }
}

impl Serialize for u16 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }

    fn serialized_size(&self) -> usize {
        2
    }
}

impl Deserialize for u16 {
    fn deserialize(buffer: &[u8]) -> Result<(Self, usize)> {
        if buffer.len() < 2 {
            return Err(Error::new(ErrorKind::InvalidInput, "Buffer too short"));
        }
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&buffer[..2]);
        Ok((u16::from_le_bytes(bytes), 2))
    }
}

impl Serialize for u32 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }

    fn serialized_size(&self) -> usize {
        4
    }
}

impl Deserialize for u32 {
    fn deserialize(buffer: &[u8]) -> Result<(Self, usize)> {
        if buffer.len() < 4 {
            return Err(Error::new(ErrorKind::InvalidInput, "Buffer too short"));
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&buffer[..4]);
        Ok((u32::from_le_bytes(bytes), 4))
    }
}

impl Serialize for u64 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }

    fn serialized_size(&self) -> usize {
        8
    }
}

impl Deserialize for u64 {
    fn deserialize(buffer: &[u8]) -> Result<(Self, usize)> {
        if buffer.len() < 8 {
            return Err(Error::new(ErrorKind::InvalidInput, "Buffer too short"));
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&buffer[..8]);
        Ok((u64::from_le_bytes(bytes), 8))
    }
}

impl Serialize for String {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<()> {
        let len = self.len() as u32;
        len.serialize(buffer)?;
        buffer.extend_from_slice(self.as_bytes());
        Ok(())
    }

    fn serialized_size(&self) -> usize {
        4 + self.len()
    }
}

impl Deserialize for String {
    fn deserialize(buffer: &[u8]) -> Result<(Self, usize)> {
        let (len, mut consumed) = u32::deserialize(buffer)?;
        let len = len as usize;

        if buffer.len() < consumed + len {
            return Err(Error::new(ErrorKind::InvalidInput, "Buffer too short"));
        }

        let s = String::from_utf8(buffer[consumed..consumed + len].to_vec())
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid UTF-8"))?;

        consumed += len;
        Ok((s, consumed))
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<()> {
        let len = self.len() as u32;
        len.serialize(buffer)?;

        for item in self {
            item.serialize(buffer)?;
        }

        Ok(())
    }

    fn serialized_size(&self) -> usize {
        4 + self.iter().map(|item| item.serialized_size()).sum::<usize>()
    }
}

impl<T: Deserialize> Deserialize for Vec<T> {
    fn deserialize(buffer: &[u8]) -> Result<(Self, usize)> {
        let (len, mut consumed) = u32::deserialize(buffer)?;
        let len = len as usize;

        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            let (item, item_consumed) = T::deserialize(&buffer[consumed..])?;
            vec.push(item);
            consumed += item_consumed;
        }

        Ok((vec, consumed))
    }
}

pub struct SerializeBuffer {
    buffer: Vec<u8>,
}

impl SerializeBuffer {
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

    pub fn write<T: Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut self.buffer)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.buffer
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.buffer
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl Default for SerializeBuffer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DeserializeBuffer<'a> {
    buffer: &'a [u8],
    position: usize,
}

impl<'a> DeserializeBuffer<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            position: 0,
        }
    }

    pub fn read<T: Deserialize>(&mut self) -> Result<T> {
        let (value, consumed) = T::deserialize(&self.buffer[self.position..])?;
        self.position += consumed;
        Ok(value)
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

    pub fn reset(&mut self) {
        self.position = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_primitives() {
        let mut buf = SerializeBuffer::new();

        buf.write(&42u8).unwrap();
        buf.write(&1000u16).unwrap();
        buf.write(&100000u32).unwrap();

        assert!(buf.len() > 0);
    }

    #[test]
    fn test_deserialize_primitives() {
        let mut buf = SerializeBuffer::new();
        buf.write(&42u8).unwrap();
        buf.write(&1000u16).unwrap();

        let mut reader = DeserializeBuffer::new(buf.as_slice());
        assert_eq!(reader.read::<u8>().unwrap(), 42);
        assert_eq!(reader.read::<u16>().unwrap(), 1000);
    }

    #[test]
    fn test_serialize_string() {
        let mut buf = SerializeBuffer::new();
        let s = String::from("Hello");
        buf.write(&s).unwrap();

        let mut reader = DeserializeBuffer::new(buf.as_slice());
        let decoded = reader.read::<String>().unwrap();
        assert_eq!(decoded, "Hello");
    }

    #[test]
    fn test_serialize_vec() {
        let mut buf = SerializeBuffer::new();
        let vec = vec![1u32, 2u32, 3u32];
        buf.write(&vec).unwrap();

        let mut reader = DeserializeBuffer::new(buf.as_slice());
        let decoded = reader.read::<Vec<u32>>().unwrap();
        assert_eq!(decoded, vec);
    }

    #[test]
    fn test_multiple_values() {
        let mut buf = SerializeBuffer::new();
        buf.write(&10u8).unwrap();
        buf.write(&String::from("test")).unwrap();
        buf.write(&vec![1u16, 2u16]).unwrap();

        let mut reader = DeserializeBuffer::new(buf.as_slice());
        assert_eq!(reader.read::<u8>().unwrap(), 10);
        assert_eq!(reader.read::<String>().unwrap(), "test");
        assert_eq!(reader.read::<Vec<u16>>().unwrap(), vec![1, 2]);
    }
}
