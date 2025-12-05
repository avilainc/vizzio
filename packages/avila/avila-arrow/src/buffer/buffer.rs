//! Core buffer type

use std::sync::Arc;

/// Immutable buffer of bytes
#[derive(Debug, Clone)]
pub struct Buffer {
    data: Arc<Vec<u8>>,
    offset: usize,
    length: usize,
}

impl Buffer {
    pub fn new(data: Vec<u8>) -> Self {
        let length = data.len();
        Self {
            data: Arc::new(data),
            offset: 0,
            length,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data[self.offset..self.offset + self.length]
    }
}

/// Mutable buffer
#[derive(Debug)]
pub struct MutableBuffer {
    data: Vec<u8>,
}

impl MutableBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, byte: u8) {
        self.data.push(byte);
    }

    pub fn freeze(self) -> Buffer {
        Buffer::new(self.data)
    }
}
