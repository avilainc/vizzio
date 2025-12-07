//! Bit-level buffer operations
//! For protocols that need bit-precise control

use avila_error::{Error, ErrorKind, Result};

/// Buffer for bit-level operations
///
/// # Examples
///
/// ```
/// use avila_buffer::BitBuffer;
///
/// let mut bb = BitBuffer::new();
/// bb.write_bits(0b101, 3)?;  // Write 3 bits
/// bb.write_bits(0xFF, 8)?;   // Write 8 bits
///
/// let val = bb.read_bits(3)?; // Read 3 bits
/// ```
pub struct BitBuffer {
    data: alloc::vec::Vec<u8>,
    bit_pos: usize,  // Current bit position for reading
    write_bit_pos: usize,  // Current bit position for writing
}

impl BitBuffer {
    /// Creates a new bit buffer
    pub fn new() -> Self {
        Self {
            data: alloc::vec::Vec::new(),
            bit_pos: 0,
            write_bit_pos: 0,
        }
    }

    /// Creates with capacity in bytes
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: alloc::vec::Vec::with_capacity(capacity),
            bit_pos: 0,
            write_bit_pos: 0,
        }
    }

    /// Writes up to 64 bits
    pub fn write_bits(&mut self, value: u64, num_bits: usize) -> Result<()> {
        if num_bits == 0 || num_bits > 64 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "num_bits must be 1-64",
            ));
        }

        // Mask to get only the bits we want
        let mask = if num_bits == 64 {
            u64::MAX
        } else {
            (1u64 << num_bits) - 1
        };
        let value = value & mask;

        for i in 0..num_bits {
            let bit = ((value >> (num_bits - 1 - i)) & 1) != 0;
            self.write_bit(bit)?;
        }

        Ok(())
    }

    /// Writes a single bit
    pub fn write_bit(&mut self, bit: bool) -> Result<()> {
        let byte_pos = self.write_bit_pos / 8;
        let bit_offset = self.write_bit_pos % 8;

        // Ensure we have enough capacity
        if byte_pos >= self.data.len() {
            self.data.push(0);
        }

        if bit {
            self.data[byte_pos] |= 1 << (7 - bit_offset);
        }

        self.write_bit_pos += 1;
        Ok(())
    }

    /// Writes a full byte
    pub fn write_byte(&mut self, byte: u8) -> Result<()> {
        self.write_bits(byte as u64, 8)
    }

    /// Reads up to 64 bits
    pub fn read_bits(&mut self, num_bits: usize) -> Result<u64> {
        if num_bits == 0 || num_bits > 64 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "num_bits must be 1-64",
            ));
        }

        if self.bit_pos + num_bits > self.write_bit_pos {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Not enough bits to read",
            ));
        }

        let mut result = 0u64;

        for _ in 0..num_bits {
            let bit = self.read_bit()?;
            result = (result << 1) | (bit as u64);
        }

        Ok(result)
    }

    /// Reads a single bit
    pub fn read_bit(&mut self) -> Result<bool> {
        if self.bit_pos >= self.write_bit_pos {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No more bits to read",
            ));
        }

        let byte_pos = self.bit_pos / 8;
        let bit_offset = self.bit_pos % 8;

        let bit = (self.data[byte_pos] >> (7 - bit_offset)) & 1;
        self.bit_pos += 1;

        Ok(bit != 0)
    }

    /// Reads a byte (8 bits)
    pub fn read_byte(&mut self) -> Result<u8> {
        self.read_bits(8).map(|v| v as u8)
    }

    /// Returns number of bits written
    pub fn len_bits(&self) -> usize {
        self.write_bit_pos
    }

    /// Returns number of bytes (rounded up)
    pub fn len_bytes(&self) -> usize {
        (self.write_bit_pos + 7) / 8
    }

    /// Checks if empty
    pub fn is_empty(&self) -> bool {
        self.write_bit_pos == 0
    }

    /// Returns available bits to read
    pub fn available_bits(&self) -> usize {
        self.write_bit_pos.saturating_sub(self.bit_pos)
    }

    /// Aligns to byte boundary (pads with zeros)
    pub fn align_to_byte(&mut self) -> Result<()> {
        let remainder = self.write_bit_pos % 8;
        if remainder != 0 {
            let padding = 8 - remainder;
            for _ in 0..padding {
                self.write_bit(false)?;
            }
        }
        Ok(())
    }

    /// Resets read position
    pub fn reset_read(&mut self) {
        self.bit_pos = 0;
    }

    /// Clears the buffer
    pub fn clear(&mut self) {
        self.data.clear();
        self.bit_pos = 0;
        self.write_bit_pos = 0;
    }

    /// Returns underlying byte data
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Peeks at next n bits without consuming
    pub fn peek_bits(&self, num_bits: usize) -> Result<u64> {
        if num_bits == 0 || num_bits > 64 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "num_bits must be 1-64",
            ));
        }

        if self.bit_pos + num_bits > self.write_bit_pos {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Not enough bits available",
            ));
        }

        let mut result = 0u64;
        let mut pos = self.bit_pos;

        for _ in 0..num_bits {
            let byte_pos = pos / 8;
            let bit_offset = pos % 8;
            let bit = (self.data[byte_pos] >> (7 - bit_offset)) & 1;
            result = (result << 1) | (bit as u64);
            pos += 1;
        }

        Ok(result)
    }

    /// Skips n bits
    pub fn skip_bits(&mut self, num_bits: usize) -> Result<()> {
        if self.bit_pos + num_bits > self.write_bit_pos {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Cannot skip beyond written bits",
            ));
        }
        self.bit_pos += num_bits;
        Ok(())
    }
}

impl Default for BitBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for BitBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BitBuffer")
            .field("len_bits", &self.len_bits())
            .field("len_bytes", &self.len_bytes())
            .field("available_bits", &self.available_bits())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_buffer_basic() {
        let mut bb = BitBuffer::new();

        bb.write_bits(0b101, 3).unwrap();
        assert_eq!(bb.len_bits(), 3);

        let val = bb.read_bits(3).unwrap();
        assert_eq!(val, 0b101);
    }

    #[test]
    fn test_bit_buffer_byte() {
        let mut bb = BitBuffer::new();

        bb.write_byte(0xFF).unwrap();
        bb.write_byte(0x00).unwrap();

        assert_eq!(bb.read_byte().unwrap(), 0xFF);
        assert_eq!(bb.read_byte().unwrap(), 0x00);
    }

    #[test]
    fn test_bit_buffer_mixed() {
        let mut bb = BitBuffer::new();

        bb.write_bits(0b11, 2).unwrap();
        bb.write_bits(0b101010, 6).unwrap();

        assert_eq!(bb.read_bits(2).unwrap(), 0b11);
        assert_eq!(bb.read_bits(6).unwrap(), 0b101010);
    }

    #[test]
    fn test_bit_buffer_align() {
        let mut bb = BitBuffer::new();

        bb.write_bits(0b101, 3).unwrap();
        assert_eq!(bb.len_bits(), 3);

        bb.align_to_byte().unwrap();
        assert_eq!(bb.len_bits(), 8);
    }

    #[test]
    fn test_bit_buffer_peek() {
        let mut bb = BitBuffer::new();
        bb.write_bits(0b11001100, 8).unwrap();

        assert_eq!(bb.peek_bits(4).unwrap(), 0b1100);
        assert_eq!(bb.read_bits(4).unwrap(), 0b1100);
        assert_eq!(bb.read_bits(4).unwrap(), 0b1100);
    }

    #[test]
    fn test_bit_buffer_skip() {
        let mut bb = BitBuffer::new();
        bb.write_bits(0b11110000, 8).unwrap();

        bb.skip_bits(4).unwrap();
        assert_eq!(bb.read_bits(4).unwrap(), 0b0000);
    }

    #[test]
    fn test_bit_buffer_64bits() {
        let mut bb = BitBuffer::new();
        let value = 0x123456789ABCDEF0u64;

        bb.write_bits(value, 64).unwrap();
        assert_eq!(bb.read_bits(64).unwrap(), value);
    }
}
