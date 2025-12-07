//! Null bitmaps for tracking null values

/// Bitmap for tracking null values
#[derive(Debug, Clone)]
pub struct Bitmap {
    buffer: Vec<u8>,
    length: usize,
}

impl Bitmap {
    pub fn new(length: usize) -> Self {
        let byte_len = (length + 7) / 8;
        Self {
            buffer: vec![0; byte_len],
            length,
        }
    }

    pub fn set(&mut self, index: usize, value: bool) {
        if index >= self.length {
            return;
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        if value {
            self.buffer[byte_index] |= 1 << bit_index;
        } else {
            self.buffer[byte_index] &= !(1 << bit_index);
        }
    }

    pub fn get(&self, index: usize) -> bool {
        if index >= self.length {
            return false;
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        (self.buffer[byte_index] & (1 << bit_index)) != 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}
