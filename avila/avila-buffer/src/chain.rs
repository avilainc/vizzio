//! Chained buffer system for dynamic growth without reallocation
//! Useful for streaming scenarios where data size is unknown

use alloc::vec::Vec;
use alloc::collections::VecDeque;
use avila_error::{Error, ErrorKind, Result};

/// A segment in the buffer chain
struct BufferSegment {
    data: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
}

impl BufferSegment {
    fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            read_pos: 0,
            write_pos: 0,
        }
    }

    fn with_data(data: Vec<u8>) -> Self {
        let len = data.len();
        Self {
            data,
            read_pos: 0,
            write_pos: len,
        }
    }

    fn available_read(&self) -> usize {
        self.write_pos - self.read_pos
    }

    fn available_write(&self) -> usize {
        self.data.capacity() - self.write_pos
    }

    fn is_fully_read(&self) -> bool {
        self.read_pos >= self.write_pos
    }

    fn write(&mut self, data: &[u8]) -> usize {
        let available = self.available_write();
        let to_write = available.min(data.len());

        self.data.extend_from_slice(&data[..to_write]);
        self.write_pos += to_write;

        to_write
    }

    fn read(&mut self, buf: &mut [u8]) -> usize {
        let available = self.available_read();
        let to_read = available.min(buf.len());

        buf[..to_read].copy_from_slice(&self.data[self.read_pos..self.read_pos + to_read]);
        self.read_pos += to_read;

        to_read
    }
}

/// Chained buffer that grows dynamically without reallocation
///
/// Instead of reallocating when capacity is exceeded, new segments are added.
/// This is efficient for streaming scenarios and prevents large memory moves.
pub struct ChainBuffer {
    segments: VecDeque<BufferSegment>,
    segment_size: usize,
    total_readable: usize,
}

impl ChainBuffer {
    /// Creates a new chain buffer with default segment size (4KB)
    pub fn new() -> Self {
        Self::with_segment_size(4096)
    }

    /// Creates a chain buffer with custom segment size
    pub fn with_segment_size(segment_size: usize) -> Self {
        Self {
            segments: VecDeque::new(),
            segment_size,
            total_readable: 0,
        }
    }

    /// Returns total readable bytes across all segments
    pub fn len(&self) -> usize {
        self.total_readable
    }

    /// Checks if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.total_readable == 0
    }

    /// Returns number of segments in chain
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    /// Writes data to buffer, creating new segments as needed
    pub fn write(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        let mut remaining = data;

        while !remaining.is_empty() {
            // Get or create write segment
            if self.segments.is_empty() ||
               self.segments.back().unwrap().available_write() == 0 {
                self.segments.push_back(BufferSegment::new(self.segment_size));
            }

            let segment = self.segments.back_mut().unwrap();
            let n = segment.write(remaining);

            written += n;
            remaining = &remaining[n..];
        }

        self.total_readable += written;
        written
    }

    /// Reads data from buffer, removing fully read segments
    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let mut total_read = 0;
        let mut remaining = buf;

        while !remaining.is_empty() && !self.segments.is_empty() {
            let segment = self.segments.front_mut().unwrap();
            let n = segment.read(remaining);

            total_read += n;
            remaining = &mut remaining[n..];

            // Remove fully read segment
            if segment.is_fully_read() {
                self.segments.pop_front();
            }

            if n == 0 {
                break; // No more data
            }
        }

        self.total_readable -= total_read;
        total_read
    }

    /// Peeks at data without consuming
    pub fn peek(&self, buf: &mut [u8]) -> usize {
        let mut total_read = 0;
        let mut remaining = buf;

        for segment in &self.segments {
            if remaining.is_empty() {
                break;
            }

            let available = segment.available_read();
            let to_read = available.min(remaining.len());

            remaining[..to_read].copy_from_slice(
                &segment.data[segment.read_pos..segment.read_pos + to_read]
            );

            total_read += to_read;
            remaining = &mut remaining[to_read..];
        }

        total_read
    }

    /// Skips n bytes
    pub fn skip(&mut self, mut n: usize) -> usize {
        let mut skipped = 0;

        while n > 0 && !self.segments.is_empty() {
            let segment = self.segments.front_mut().unwrap();
            let available = segment.available_read();
            let to_skip = available.min(n);

            segment.read_pos += to_skip;
            skipped += to_skip;
            n -= to_skip;

            if segment.is_fully_read() {
                self.segments.pop_front();
            }
        }

        self.total_readable -= skipped;
        skipped
    }

    /// Clears all data
    pub fn clear(&mut self) {
        self.segments.clear();
        self.total_readable = 0;
    }

    /// Appends another buffer's data
    pub fn append(&mut self, other: &mut ChainBuffer) {
        while let Some(segment) = other.segments.pop_front() {
            self.total_readable += segment.available_read();
            self.segments.push_back(segment);
        }
        other.total_readable = 0;
    }

    /// Compacts into single segment
    pub fn compact(&mut self) {
        if self.segments.len() <= 1 {
            return;
        }

        let mut new_data = Vec::with_capacity(self.total_readable);

        while let Some(mut segment) = self.segments.pop_front() {
            let available = segment.available_read();
            let mut buf = vec![0u8; available];
            segment.read(&mut buf);
            new_data.extend_from_slice(&buf);
        }

        self.segments.push_back(BufferSegment::with_data(new_data));
    }

    /// Reads all data into Vec
    pub fn read_all(&mut self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.total_readable);

        while !self.segments.is_empty() {
            let segment = self.segments.front_mut().unwrap();
            let available = segment.available_read();
            let mut buf = vec![0u8; available];
            segment.read(&mut buf);
            result.extend_from_slice(&buf);

            if segment.is_fully_read() {
                self.segments.pop_front();
            }
        }

        self.total_readable = 0;
        result
    }

    /// Peeks all data without consuming
    pub fn peek_all(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.total_readable);

        for segment in &self.segments {
            let available = segment.available_read();
            result.extend_from_slice(&segment.data[segment.read_pos..segment.read_pos + available]);
        }

        result
    }

    /// Memory usage statistics
    pub fn memory_usage(&self) -> MemoryUsage {
        let mut allocated = 0;
        let mut used = 0;

        for segment in &self.segments {
            allocated += segment.data.capacity();
            used += segment.available_read();
        }

        MemoryUsage {
            allocated,
            used,
            segments: self.segments.len(),
            fragmentation: if allocated > 0 {
                1.0 - (used as f32 / allocated as f32)
            } else {
                0.0
            },
        }
    }
}

impl Default for ChainBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory usage statistics
#[derive(Debug, Clone, Copy)]
pub struct MemoryUsage {
    pub allocated: usize,
    pub used: usize,
    pub segments: usize,
    pub fragmentation: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_write_read() {
        let mut chain = ChainBuffer::with_segment_size(8);

        let written = chain.write(b"Hello");
        assert_eq!(written, 5);
        assert_eq!(chain.len(), 5);

        let mut buf = [0u8; 10];
        let read = chain.read(&mut buf);
        assert_eq!(read, 5);
        assert_eq!(&buf[..5], b"Hello");
        assert_eq!(chain.len(), 0);
    }

    #[test]
    fn test_multiple_segments() {
        let mut chain = ChainBuffer::with_segment_size(4);

        chain.write(b"1234");
        chain.write(b"5678");
        chain.write(b"90");

        assert_eq!(chain.len(), 10);
        assert!(chain.segment_count() >= 2);

        let data = chain.peek_all();
        assert_eq!(&data, b"1234567890");
    }

    #[test]
    fn test_peek() {
        let mut chain = ChainBuffer::new();
        chain.write(b"Hello World");

        let mut buf = [0u8; 5];
        chain.peek(&mut buf);
        assert_eq!(&buf, b"Hello");
        assert_eq!(chain.len(), 11); // Not consumed
    }

    #[test]
    fn test_skip() {
        let mut chain = ChainBuffer::new();
        chain.write(b"Hello World");

        chain.skip(6);
        assert_eq!(chain.len(), 5);

        let mut buf = [0u8; 10];
        let read = chain.read(&mut buf);
        assert_eq!(&buf[..read], b"World");
    }

    #[test]
    fn test_append() {
        let mut chain1 = ChainBuffer::new();
        let mut chain2 = ChainBuffer::new();

        chain1.write(b"Hello ");
        chain2.write(b"World");

        chain1.append(&mut chain2);

        assert_eq!(chain1.len(), 11);
        assert_eq!(chain2.len(), 0);

        let data = chain1.peek_all();
        assert_eq!(&data, b"Hello World");
    }

    #[test]
    fn test_compact() {
        let mut chain = ChainBuffer::with_segment_size(4);

        chain.write(b"123");
        chain.write(b"456");
        chain.write(b"789");

        let segments_before = chain.segment_count();
        chain.compact();
        let segments_after = chain.segment_count();

        assert!(segments_after < segments_before);
        assert_eq!(chain.len(), 9);
    }

    #[test]
    fn test_memory_usage() {
        let mut chain = ChainBuffer::with_segment_size(100);
        chain.write(b"Hello");

        let usage = chain.memory_usage();
        assert_eq!(usage.used, 5);
        assert!(usage.allocated >= 5);
        assert!(usage.fragmentation >= 0.0 && usage.fragmentation <= 1.0);
    }

    #[test]
    fn test_read_all() {
        let mut chain = ChainBuffer::new();
        chain.write(b"Test data");

        let data = chain.read_all();
        assert_eq!(&data, b"Test data");
        assert_eq!(chain.len(), 0);
    }
}
