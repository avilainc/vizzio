//! Iterator implementations for buffers

use crate::ByteBuffer;

/// Iterator over bytes in a ByteBuffer
pub struct ByteBufferIter<'a> {
    buffer: &'a [u8],
    pos: usize,
}

impl<'a> ByteBufferIter<'a> {
    /// Creates a new iterator
    pub fn new(buffer: &'a ByteBuffer) -> Self {
        Self {
            buffer: buffer.as_slice(),
            pos: 0,
        }
    }
}

impl<'a> Iterator for ByteBufferIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.buffer.len() {
            let byte = self.buffer[self.pos];
            self.pos += 1;
            Some(byte)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.buffer.len() - self.pos;
        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for ByteBufferIter<'a> {
    fn len(&self) -> usize {
        self.buffer.len() - self.pos
    }
}

impl<'a> DoubleEndedIterator for ByteBufferIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.pos < self.buffer.len() {
            let byte = self.buffer[self.buffer.len() - 1];
            Some(byte)
        } else {
            None
        }
    }
}

/// Chunked iterator for reading chunks of data
pub struct ChunkIter<'a> {
    buffer: &'a [u8],
    chunk_size: usize,
    pos: usize,
}

impl<'a> ChunkIter<'a> {
    /// Creates a new chunk iterator
    pub fn new(buffer: &'a ByteBuffer, chunk_size: usize) -> Self {
        Self {
            buffer: buffer.as_slice(),
            chunk_size,
            pos: 0,
        }
    }
}

impl<'a> Iterator for ChunkIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.buffer.len() {
            return None;
        }

        let end = (self.pos + self.chunk_size).min(self.buffer.len());
        let chunk = &self.buffer[self.pos..end];
        self.pos = end;
        Some(chunk)
    }
}

/// Window iterator for sliding window operations
pub struct WindowIter<'a> {
    buffer: &'a [u8],
    window_size: usize,
    pos: usize,
}

impl<'a> WindowIter<'a> {
    /// Creates a new window iterator
    pub fn new(buffer: &'a ByteBuffer, window_size: usize) -> Self {
        Self {
            buffer: buffer.as_slice(),
            window_size,
            pos: 0,
        }
    }
}

impl<'a> Iterator for WindowIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos + self.window_size > self.buffer.len() {
            return None;
        }

        let window = &self.buffer[self.pos..self.pos + self.window_size];
        self.pos += 1;
        Some(window)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_buffer_iter() {
        let buffer = ByteBuffer::from(vec![1, 2, 3, 4, 5]);
        let mut iter = ByteBufferIter::new(&buffer);

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.len(), 3);
    }

    #[test]
    fn test_chunk_iter() {
        let buffer = ByteBuffer::from(vec![1, 2, 3, 4, 5, 6, 7]);
        let chunks: Vec<_> = ChunkIter::new(&buffer, 3).collect();

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], &[1, 2, 3]);
        assert_eq!(chunks[1], &[4, 5, 6]);
        assert_eq!(chunks[2], &[7]);
    }

    #[test]
    fn test_window_iter() {
        let buffer = ByteBuffer::from(vec![1, 2, 3, 4, 5]);
        let windows: Vec<_> = WindowIter::new(&buffer, 3).collect();

        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[1], &[2, 3, 4]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }
}
