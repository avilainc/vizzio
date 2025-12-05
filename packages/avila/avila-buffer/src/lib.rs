//! # avila-buffer - High-Performance Buffer Management
//!
//! Zero-copy buffer operations for efficient I/O.
//!
//! # Examples
//!
//! ```
//! use avila_buffer::ByteBuffer;
//!
//! let mut buffer = ByteBuffer::with_capacity(1024);
//! buffer.write(b"Hello, World!").unwrap();
//!
//! let mut output = vec![0u8; 13];
//! buffer.read(&mut output).unwrap();
//! assert_eq!(&output, b"Hello, World!");
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

extern crate alloc;
use alloc::vec::Vec;
use avila_error::{Error, ErrorKind, Result};

// Module declarations
pub mod codec;
pub mod io;
pub mod sync;
pub mod utils;
pub mod iter;

// Public modules
pub mod fixed_buffer;
pub mod pool;

// Re-exports
pub use codec::{PrimitiveDecoder, PrimitiveEncoder, VarintDecoder, VarintEncoder};
pub use utils::BufferMetrics;
pub use fixed_buffer::FixedBuffer;
pub use pool::{BufferPool, PooledBuffer};
pub use iter::{ByteBufferIter, ChunkIter, WindowIter};

#[cfg(feature = "std")]
pub use sync::SharedBuffer;

/// Buffer with read/write cursors
pub struct ByteBuffer {
    data: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
}

impl ByteBuffer {
    /// Creates new buffer with capacity
    ///
    /// # Examples
    ///
    /// ```
    /// use avila_buffer::ByteBuffer;
    ///
    /// let buffer = ByteBuffer::with_capacity(1024);
    /// assert!(buffer.capacity() >= 1024);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            read_pos: 0,
            write_pos: 0,
        }
    }

    /// Creates new empty buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use avila_buffer::ByteBuffer;
    ///
    /// let buffer = ByteBuffer::new();
    /// assert_eq!(buffer.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Creates buffer from existing data
    ///
    /// # Examples
    ///
    /// ```
    /// use avila_buffer::ByteBuffer;
    ///
    /// let data = vec![1, 2, 3, 4, 5];
    /// let buffer = ByteBuffer::from_vec(data);
    /// assert_eq!(buffer.len(), 5);
    /// ```
    pub fn from_vec(data: Vec<u8>) -> Self {
        let len = data.len();
        Self {
            data,
            read_pos: 0,
            write_pos: len,
        }
    }

    /// Writes bytes to buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use avila_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new();
    /// let n = buffer.write(b"Hello").unwrap();
    /// assert_eq!(n, 5);
    /// ```
    pub fn write(&mut self, bytes: &[u8]) -> Result<usize> {
        let space = self.data.capacity() - self.write_pos;
        if space < bytes.len() {
            self.data.reserve(bytes.len() - space);
        }

        if self.write_pos + bytes.len() > self.data.len() {
            self.data.resize(self.write_pos + bytes.len(), 0);
        }

        self.data[self.write_pos..self.write_pos + bytes.len()].copy_from_slice(bytes);
        self.write_pos += bytes.len();
        Ok(bytes.len())
    }

    /// Reads bytes from buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use avila_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::from_vec(vec![1, 2, 3, 4, 5]);
    /// let mut buf = [0u8; 3];
    /// let n = buffer.read(&mut buf).unwrap();
    /// assert_eq!(n, 3);
    /// assert_eq!(buf, [1, 2, 3]);
    /// ```
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

    /// Returns available bytes for reading
    pub fn available(&self) -> usize {
        self.write_pos - self.read_pos
    }

    /// Returns current length (same as available)
    pub fn len(&self) -> usize {
        self.available()
    }

    /// Checks if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.available() == 0
    }

    /// Returns capacity
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Returns remaining capacity for writing
    pub fn remaining(&self) -> usize {
        self.data.capacity() - self.write_pos
    }

    /// Resets read and write positions
    pub fn reset(&mut self) {
        self.read_pos = 0;
        self.write_pos = 0;
    }

    /// Clears the buffer and resets positions
    pub fn clear(&mut self) {
        self.data.clear();
        self.read_pos = 0;
        self.write_pos = 0;
    }

    /// Compacts buffer (moves unread data to start)
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

    /// Peek at bytes without consuming them
    pub fn peek(&self, buf: &mut [u8]) -> Result<usize> {
        let available = self.write_pos - self.read_pos;
        let to_read = available.min(buf.len());

        if to_read == 0 {
            return Ok(0);
        }

        buf[..to_read].copy_from_slice(&self.data[self.read_pos..self.read_pos + to_read]);
        Ok(to_read)
    }

    /// Skip n bytes in the read cursor
    pub fn skip(&mut self, n: usize) -> Result<usize> {
        let available = self.available();
        let to_skip = available.min(n);
        self.read_pos += to_skip;
        Ok(to_skip)
    }

    /// Reserve additional capacity
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// Shrink capacity to fit current data
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    /// Get read position
    pub fn read_position(&self) -> usize {
        self.read_pos
    }

    /// Get write position
    pub fn write_position(&self) -> usize {
        self.write_pos
    }

    /// Set read position (unsafe if position > write_pos)
    pub fn set_read_position(&mut self, pos: usize) -> Result<()> {
        if pos > self.write_pos {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Read position cannot exceed write position",
            ));
        }
        self.read_pos = pos;
        Ok(())
    }

    /// Write all from another buffer
    pub fn write_all(&mut self, other: &ByteBuffer) -> Result<usize> {
        self.write(other.as_slice())
    }

    /// Extend with bytes from an iterator
    pub fn extend<I>(&mut self, iter: I) -> Result<usize>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut count = 0;
        for byte in iter {
            self.write(&[byte])?;
            count += 1;
        }
        Ok(count)
    }

    /// Create an iterator over the bytes
    pub fn iter(&self) -> crate::iter::ByteBufferIter<'_> {
        crate::iter::ByteBufferIter::new(self)
    }

    /// Create an iterator over chunks of specified size
    pub fn chunks(&self, chunk_size: usize) -> crate::iter::ChunkIter<'_> {
        crate::iter::ChunkIter::new(self, chunk_size)
    }

    /// Create a sliding window iterator
    pub fn windows(&self, window_size: usize) -> crate::iter::WindowIter<'_> {
        crate::iter::WindowIter::new(self, window_size)
    }

    /// Find the first occurrence of a byte
    pub fn find(&self, byte: u8) -> Option<usize> {
        self.as_slice().iter().position(|&b| b == byte)
    }

    /// Find the first occurrence of a pattern
    pub fn find_pattern(&self, pattern: &[u8]) -> Option<usize> {
        let data = self.as_slice();
        if pattern.is_empty() || pattern.len() > data.len() {
            return None;
        }

        for i in 0..=(data.len() - pattern.len()) {
            if &data[i..i + pattern.len()] == pattern {
                return Some(i);
            }
        }
        None
    }

    /// Check if buffer starts with pattern
    pub fn starts_with(&self, pattern: &[u8]) -> bool {
        self.as_slice().starts_with(pattern)
    }

    /// Check if buffer ends with pattern
    pub fn ends_with(&self, pattern: &[u8]) -> bool {
        self.as_slice().ends_with(pattern)
    }

    /// Split buffer at position
    pub fn split_at(&self, pos: usize) -> Result<(ByteBuffer, ByteBuffer)> {
        if pos > self.len() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Split position out of bounds",
            ));
        }

        let data = self.as_slice();
        let left = ByteBuffer::from(&data[..pos]);
        let right = ByteBuffer::from(&data[pos..]);
        Ok((left, right))
    }
}

impl Default for ByteBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ByteBuffer {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            read_pos: self.read_pos,
            write_pos: self.write_pos,
        }
    }
}

impl core::fmt::Debug for ByteBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ByteBuffer")
            .field("capacity", &self.capacity())
            .field("read_pos", &self.read_pos)
            .field("write_pos", &self.write_pos)
            .field("available", &self.available())
            .finish()
    }
}

impl PartialEq for ByteBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for ByteBuffer {}

impl From<Vec<u8>> for ByteBuffer {
    fn from(data: Vec<u8>) -> Self {
        Self::from_vec(data)
    }
}

impl From<&[u8]> for ByteBuffer {
    fn from(data: &[u8]) -> Self {
        let mut buffer = Self::with_capacity(data.len());
        buffer.write(data).unwrap();
        buffer
    }
}

impl AsRef<[u8]> for ByteBuffer {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsMut<[u8]> for ByteBuffer {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

/// Ring buffer for circular data
pub struct RingBuffer<T, const N: usize> {
    data: [Option<T>; N],
    head: usize,
    tail: usize,
    size: usize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    /// Creates new ring buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use avila_buffer::RingBuffer;
    ///
    /// let ring: RingBuffer<i32, 10> = RingBuffer::new();
    /// assert!(ring.is_empty());
    /// assert_eq!(ring.capacity(), 10);
    /// ```
    pub const fn new() -> Self {
        Self {
            data: [const { None }; N],
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    /// Pushes item to buffer
    ///
    /// Returns error if buffer is full
    pub fn push(&mut self, item: T) -> Result<()> {
        if self.size == N {
            return Err(Error::new(ErrorKind::InvalidState, "Ring buffer full"));
        }

        self.data[self.tail] = Some(item);
        self.tail = (self.tail + 1) % N;
        self.size += 1;
        Ok(())
    }

    /// Pops item from buffer
    ///
    /// Returns `None` if buffer is empty
    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let item = self.data[self.head].take();
        self.head = (self.head + 1) % N;
        self.size -= 1;
        item
    }

    /// Peeks at the front item without removing it
    pub fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }
        self.data[self.head].as_ref()
    }

    /// Returns current size
    pub const fn len(&self) -> usize {
        self.size
    }

    /// Checks if empty
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Checks if full
    pub const fn is_full(&self) -> bool {
        self.size == N
    }

    /// Returns capacity
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Clears the buffer
    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T, const N: usize> Default for RingBuffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{ByteBuffer, RingBuffer, FixedBuffer, BufferPool, PooledBuffer};
    pub use crate::codec::{PrimitiveDecoder, PrimitiveEncoder, VarintDecoder, VarintEncoder};
    pub use crate::utils::BufferMetrics;

    #[cfg(feature = "std")]
    pub use crate::sync::SharedBuffer;
}#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_buffer_basic() {
        let mut buf = ByteBuffer::with_capacity(16);
        assert_eq!(buf.len(), 0);
        assert!(buf.is_empty());

        buf.write(b"Hello").unwrap();
        assert_eq!(buf.len(), 5);
        assert!(!buf.is_empty());

        let mut read_buf = [0u8; 5];
        let n = buf.read(&mut read_buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&read_buf, b"Hello");
        assert!(buf.is_empty());
    }

    #[test]
    fn test_byte_buffer_compact() {
        let mut buf = ByteBuffer::with_capacity(16);
        buf.write(b"Hello World").unwrap();

        let mut tmp = [0u8; 5];
        buf.read(&mut tmp).unwrap();

        buf.compact();
        assert_eq!(buf.as_slice(), b" World");
    }

    #[test]
    fn test_ring_buffer() {
        let mut ring = RingBuffer::<i32, 4>::new();
        assert!(ring.is_empty());
        assert_eq!(ring.capacity(), 4);

        ring.push(1).unwrap();
        ring.push(2).unwrap();
        ring.push(3).unwrap();

        assert_eq!(ring.len(), 3);
        assert_eq!(ring.peek(), Some(&1));
        assert_eq!(ring.pop(), Some(1));
        assert_eq!(ring.pop(), Some(2));

        ring.push(4).unwrap();
        ring.push(5).unwrap();

        assert_eq!(ring.len(), 3);
        assert_eq!(ring.pop(), Some(3));
        assert_eq!(ring.pop(), Some(4));
        assert_eq!(ring.pop(), Some(5));
        assert!(ring.is_empty());
    }

    #[test]
    fn test_ring_buffer_full() {
        let mut ring = RingBuffer::<i32, 3>::new();

        ring.push(1).unwrap();
        ring.push(2).unwrap();
        ring.push(3).unwrap();

        assert!(ring.is_full());
        assert!(ring.push(4).is_err());
    }

    #[test]
    fn test_primitive_codec() {
        let mut buf = ByteBuffer::new();

        buf.write_u32_le(0x12345678).unwrap();
        buf.write_u16_be(0xABCD).unwrap();

        assert_eq!(buf.read_u32_le().unwrap(), 0x12345678);
        assert_eq!(buf.read_u16_be().unwrap(), 0xABCD);
    }
}
