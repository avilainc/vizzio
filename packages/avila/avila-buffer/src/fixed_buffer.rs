//! Fixed-size buffer allocated on the stack
//! No heap allocations - perfect for embedded systems

use avila_error::{Error, ErrorKind, Result};

/// Stack-allocated buffer with compile-time size
///
/// # Examples
///
/// ```
/// use avila_buffer::FixedBuffer;
///
/// let mut buffer: FixedBuffer<1024> = FixedBuffer::new();
/// buffer.write(b"Hello").unwrap();
/// ```
#[derive(Debug)]
pub struct FixedBuffer<const N: usize> {
    data: [u8; N],
    read_pos: usize,
    write_pos: usize,
}

impl<const N: usize> FixedBuffer<N> {
    /// Creates a new fixed buffer
    pub const fn new() -> Self {
        Self {
            data: [0; N],
            read_pos: 0,
            write_pos: 0,
        }
    }

    /// Creates buffer from array
    pub const fn from_array(data: [u8; N]) -> Self {
        Self {
            data,
            read_pos: 0,
            write_pos: N,
        }
    }

    /// Writes bytes to buffer
    pub fn write(&mut self, bytes: &[u8]) -> Result<usize> {
        let available = N - self.write_pos;
        if available < bytes.len() {
            return Err(Error::new(
                ErrorKind::InvalidState,
                "Fixed buffer full",
            ));
        }

        let len = bytes.len();
        self.data[self.write_pos..self.write_pos + len].copy_from_slice(bytes);
        self.write_pos += len;
        Ok(len)
    }

    /// Reads bytes from buffer
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let available = self.write_pos - self.read_pos;
        let to_read = available.min(buf.len());

        if to_read == 0 {
            return Ok(0);
        }

        buf[..to_read].copy_from_slice(&self.data[self.read_pos..self.read_pos + to_read]);
        self.read_pos += to_read;
        Ok(to_read)
    }

    /// Peek at bytes without consuming
    pub fn peek(&self, buf: &mut [u8]) -> Result<usize> {
        let available = self.write_pos - self.read_pos;
        let to_read = available.min(buf.len());

        if to_read == 0 {
            return Ok(0);
        }

        buf[..to_read].copy_from_slice(&self.data[self.read_pos..self.read_pos + to_read]);
        Ok(to_read)
    }

    /// Returns available bytes for reading
    pub const fn available(&self) -> usize {
        self.write_pos - self.read_pos
    }

    /// Returns current length
    pub const fn len(&self) -> usize {
        self.available()
    }

    /// Checks if buffer is empty
    pub const fn is_empty(&self) -> bool {
        self.available() == 0
    }

    /// Returns capacity
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns remaining space for writing
    pub const fn remaining(&self) -> usize {
        N - self.write_pos
    }

    /// Checks if buffer is full
    pub const fn is_full(&self) -> bool {
        self.write_pos == N
    }

    /// Resets positions
    pub fn reset(&mut self) {
        self.read_pos = 0;
        self.write_pos = 0;
    }

    /// Clears buffer
    pub fn clear(&mut self) {
        self.data = [0; N];
        self.read_pos = 0;
        self.write_pos = 0;
    }

    /// Compacts buffer
    pub fn compact(&mut self) {
        if self.read_pos > 0 {
            let available = self.available();
            self.data.copy_within(self.read_pos..self.write_pos, 0);
            self.read_pos = 0;
            self.write_pos = available;
        }
    }

    /// Returns slice of unread data
    pub fn as_slice(&self) -> &[u8] {
        &self.data[self.read_pos..self.write_pos]
    }

    /// Returns mutable slice of unread data
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data[self.read_pos..self.write_pos]
    }

    /// Skip n bytes
    pub fn skip(&mut self, n: usize) -> Result<usize> {
        let available = self.available();
        let to_skip = available.min(n);
        self.read_pos += to_skip;
        Ok(to_skip)
    }

    /// Get read position
    pub const fn read_position(&self) -> usize {
        self.read_pos
    }

    /// Get write position
    pub const fn write_position(&self) -> usize {
        self.write_pos
    }
}

impl<const N: usize> Default for FixedBuffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Clone for FixedBuffer<N> {
    fn clone(&self) -> Self {
        Self {
            data: self.data,
            read_pos: self.read_pos,
            write_pos: self.write_pos,
        }
    }
}

impl<const N: usize> Copy for FixedBuffer<N> {}

impl<const N: usize> PartialEq for FixedBuffer<N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<const N: usize> Eq for FixedBuffer<N> {}

impl<const N: usize> AsRef<[u8]> for FixedBuffer<N> {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<const N: usize> AsMut<[u8]> for FixedBuffer<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_buffer_basic() {
        let mut buf: FixedBuffer<16> = FixedBuffer::new();
        assert_eq!(buf.capacity(), 16);
        assert!(buf.is_empty());

        buf.write(b"Hello").unwrap();
        assert_eq!(buf.len(), 5);
        assert!(!buf.is_full());

        let mut read_buf = [0u8; 5];
        buf.read(&mut read_buf).unwrap();
        assert_eq!(&read_buf, b"Hello");
    }

    #[test]
    fn test_fixed_buffer_full() {
        let mut buf: FixedBuffer<4> = FixedBuffer::new();

        buf.write(b"1234").unwrap();
        assert!(buf.is_full());

        assert!(buf.write(b"5").is_err());
    }

    #[test]
    fn test_fixed_buffer_clone() {
        let mut buf: FixedBuffer<8> = FixedBuffer::new();
        buf.write(b"test").unwrap();

        let cloned = buf.clone();
        assert_eq!(buf.as_slice(), cloned.as_slice());
    }
}
