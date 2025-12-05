//! Circular byte buffer - ring buffer for bytes
//! More efficient than generic RingBuffer for byte streams

use avila_error::{Error, ErrorKind, Result};

/// Circular buffer optimized for bytes
///
/// Unlike RingBuffer<u8, N>, this uses a single continuous byte array
/// and manages head/tail pointers efficiently for streaming data.
///
/// # Examples
///
/// ```
/// use avila_buffer::CircularByteBuffer;
///
/// let mut cb = CircularByteBuffer::new(1024);
/// cb.write(b"Hello")?;
///
/// let mut buf = [0u8; 5];
/// cb.read(&mut buf)?;
/// ```
pub struct CircularByteBuffer {
    data: alloc::vec::Vec<u8>,
    head: usize,  // Read position
    tail: usize,  // Write position
    full: bool,   // Distinguishes full from empty when head == tail
}

impl CircularByteBuffer {
    /// Creates a new circular buffer with given capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            data: alloc::vec![0; capacity],
            head: 0,
            tail: 0,
            full: false,
        }
    }

    /// Returns the capacity
    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    /// Returns available bytes to read
    pub fn available(&self) -> usize {
        if self.full {
            self.capacity()
        } else if self.tail >= self.head {
            self.tail - self.head
        } else {
            self.capacity() - self.head + self.tail
        }
    }

    /// Returns free space to write
    pub fn free_space(&self) -> usize {
        self.capacity() - self.available()
    }

    /// Checks if buffer is empty
    pub fn is_empty(&self) -> bool {
        !self.full && self.head == self.tail
    }

    /// Checks if buffer is full
    pub fn is_full(&self) -> bool {
        self.full
    }

    /// Writes bytes to the buffer
    ///
    /// Returns number of bytes actually written (may be less than input if buffer is full)
    pub fn write(&mut self, bytes: &[u8]) -> Result<usize> {
        if bytes.is_empty() {
            return Ok(0);
        }

        let free = self.free_space();
        if free == 0 {
            return Ok(0);
        }

        let to_write = bytes.len().min(free);
        let capacity = self.capacity();

        if self.tail + to_write <= capacity {
            // Write fits in one chunk
            self.data[self.tail..self.tail + to_write].copy_from_slice(&bytes[..to_write]);
            self.tail = (self.tail + to_write) % capacity;
        } else {
            // Write wraps around
            let first_chunk = capacity - self.tail;
            self.data[self.tail..].copy_from_slice(&bytes[..first_chunk]);
            self.data[..to_write - first_chunk].copy_from_slice(&bytes[first_chunk..to_write]);
            self.tail = to_write - first_chunk;
        }

        if self.tail == self.head {
            self.full = true;
        }

        Ok(to_write)
    }

    /// Writes all bytes, returning error if buffer is full
    pub fn write_all(&mut self, bytes: &[u8]) -> Result<()> {
        if bytes.len() > self.free_space() {
            return Err(Error::new(
                ErrorKind::InvalidState,
                "Circular buffer full",
            ));
        }
        self.write(bytes)?;
        Ok(())
    }

    /// Reads bytes from the buffer
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        let available = self.available();
        if available == 0 {
            return Ok(0);
        }

        let to_read = buf.len().min(available);
        let capacity = self.capacity();

        if self.head + to_read <= capacity {
            // Read fits in one chunk
            buf[..to_read].copy_from_slice(&self.data[self.head..self.head + to_read]);
            self.head = (self.head + to_read) % capacity;
        } else {
            // Read wraps around
            let first_chunk = capacity - self.head;
            buf[..first_chunk].copy_from_slice(&self.data[self.head..]);
            buf[first_chunk..to_read].copy_from_slice(&self.data[..to_read - first_chunk]);
            self.head = to_read - first_chunk;
        }

        self.full = false;
        Ok(to_read)
    }

    /// Peeks at bytes without consuming
    pub fn peek(&self, buf: &mut [u8]) -> Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        let available = self.available();
        if available == 0 {
            return Ok(0);
        }

        let to_read = buf.len().min(available);
        let capacity = self.capacity();
        let head = self.head;

        if head + to_read <= capacity {
            buf[..to_read].copy_from_slice(&self.data[head..head + to_read]);
        } else {
            let first_chunk = capacity - head;
            buf[..first_chunk].copy_from_slice(&self.data[head..]);
            buf[first_chunk..to_read].copy_from_slice(&self.data[..to_read - first_chunk]);
        }

        Ok(to_read)
    }

    /// Skips n bytes
    pub fn skip(&mut self, n: usize) -> Result<usize> {
        let available = self.available();
        let to_skip = n.min(available);

        self.head = (self.head + to_skip) % self.capacity();
        self.full = false;

        Ok(to_skip)
    }

    /// Clears the buffer
    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.full = false;
    }

    /// Resets (alias for clear)
    pub fn reset(&mut self) {
        self.clear();
    }

    /// Returns the current fill percentage (0.0 to 1.0)
    pub fn fill_ratio(&self) -> f32 {
        self.available() as f32 / self.capacity() as f32
    }

    /// Consumes all available bytes and returns them as Vec
    pub fn consume_all(&mut self) -> alloc::vec::Vec<u8> {
        let available = self.available();
        let mut result = alloc::vec![0u8; available];
        self.read(&mut result).unwrap();
        result
    }
}

impl Default for CircularByteBuffer {
    fn default() -> Self {
        Self::new(4096)
    }
}

impl core::fmt::Debug for CircularByteBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CircularByteBuffer")
            .field("capacity", &self.capacity())
            .field("available", &self.available())
            .field("free_space", &self.free_space())
            .field("head", &self.head)
            .field("tail", &self.tail)
            .field("full", &self.full)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_basic() {
        let mut cb = CircularByteBuffer::new(10);

        assert_eq!(cb.capacity(), 10);
        assert_eq!(cb.available(), 0);
        assert_eq!(cb.free_space(), 10);
        assert!(cb.is_empty());
        assert!(!cb.is_full());
    }

    #[test]
    fn test_circular_write_read() {
        let mut cb = CircularByteBuffer::new(10);

        assert_eq!(cb.write(b"Hello").unwrap(), 5);
        assert_eq!(cb.available(), 5);

        let mut buf = [0u8; 5];
        assert_eq!(cb.read(&mut buf).unwrap(), 5);
        assert_eq!(&buf, b"Hello");
        assert!(cb.is_empty());
    }

    #[test]
    fn test_circular_wraparound() {
        let mut cb = CircularByteBuffer::new(10);

        // Fill almost to capacity
        cb.write(b"12345678").unwrap();

        // Read some
        let mut buf = [0u8; 5];
        cb.read(&mut buf).unwrap();

        // Write more (should wrap around)
        cb.write(b"ABCDEFG").unwrap();

        // Read all
        let mut buf = [0u8; 10];
        let n = cb.read(&mut buf).unwrap();
        assert_eq!(n, 10);
        assert_eq!(&buf, b"678ABCDEFG");
    }

    #[test]
    fn test_circular_full() {
        let mut cb = CircularByteBuffer::new(5);

        cb.write(b"12345").unwrap();
        assert!(cb.is_full());
        assert_eq!(cb.write(b"X").unwrap(), 0); // Can't write when full
    }

    #[test]
    fn test_circular_peek() {
        let mut cb = CircularByteBuffer::new(10);
        cb.write(b"Hello").unwrap();

        let mut buf = [0u8; 5];
        cb.peek(&mut buf).unwrap();
        assert_eq!(&buf, b"Hello");

        // Data still available
        assert_eq!(cb.available(), 5);
    }

    #[test]
    fn test_circular_skip() {
        let mut cb = CircularByteBuffer::new(10);
        cb.write(b"0123456789").unwrap();

        cb.skip(5).unwrap();

        let mut buf = [0u8; 5];
        cb.read(&mut buf).unwrap();
        assert_eq!(&buf, b"56789");
    }

    #[test]
    fn test_circular_consume_all() {
        let mut cb = CircularByteBuffer::new(10);
        cb.write(b"Hello").unwrap();

        let data = cb.consume_all();
        assert_eq!(data, b"Hello");
        assert!(cb.is_empty());
    }
}
