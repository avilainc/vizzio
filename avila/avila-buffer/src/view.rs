//! Zero-copy buffer views and slicing

use alloc::vec::Vec;
use avila_error::{Error, ErrorKind, Result};

pub struct BufferView<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> BufferView<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    pub fn with_offset(data: &'a [u8], offset: usize) -> Result<Self> {
        if offset > data.len() {
            return Err(Error::new(ErrorKind::OutOfBounds, "Offset out of bounds"));
        }
        Ok(Self { data, offset })
    }

    pub fn slice(&self, start: usize, end: usize) -> Result<BufferView<'a>> {
        if start > end || end > self.remaining() {
            return Err(Error::new(ErrorKind::OutOfBounds, "Invalid slice range"));
        }
        Ok(BufferView::new(&self.data[self.offset + start..self.offset + end]))
    }

    pub fn advance(&mut self, count: usize) -> Result<()> {
        if count > self.remaining() {
            return Err(Error::new(ErrorKind::OutOfBounds, "Advance beyond end"));
        }
        self.offset += count;
        Ok(())
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.offset
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data[self.offset..]
    }

    pub fn peek(&self, count: usize) -> Option<&[u8]> {
        if count > self.remaining() {
            return None;
        }
        Some(&self.data[self.offset..self.offset + count])
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        if self.remaining() < 1 {
            return Err(Error::new(ErrorKind::OutOfBounds, "Not enough data"));
        }
        let value = self.data[self.offset];
        self.offset += 1;
        Ok(value)
    }

    pub fn read_bytes(&mut self, count: usize) -> Result<&'a [u8]> {
        if count > self.remaining() {
            return Err(Error::new(ErrorKind::OutOfBounds, "Not enough data"));
        }
        let slice = &self.data[self.offset..self.offset + count];
        self.offset += count;
        Ok(slice)
    }

    pub fn split_at(&self, mid: usize) -> Result<(BufferView<'a>, BufferView<'a>)> {
        if mid > self.remaining() {
            return Err(Error::new(ErrorKind::OutOfBounds, "Split point out of bounds"));
        }
        let left = BufferView::new(&self.data[self.offset..self.offset + mid]);
        let right = BufferView::new(&self.data[self.offset + mid..]);
        Ok((left, right))
    }
}

pub struct BufferViewMut<'a> {
    data: &'a mut [u8],
    offset: usize,
}

impl<'a> BufferViewMut<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self { data, offset: 0 }
    }

    pub fn write_u8(&mut self, value: u8) -> Result<()> {
        if self.remaining() < 1 {
            return Err(Error::new(ErrorKind::OutOfBounds, "Not enough space"));
        }
        self.data[self.offset] = value;
        self.offset += 1;
        Ok(())
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        if bytes.len() > self.remaining() {
            return Err(Error::new(ErrorKind::OutOfBounds, "Not enough space"));
        }
        self.data[self.offset..self.offset + bytes.len()].copy_from_slice(bytes);
        self.offset += bytes.len();
        Ok(())
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.offset
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data[self.offset..]
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        let offset = self.offset;
        &mut self.data[offset..]
    }

    pub fn fill(&mut self, value: u8) {
        for i in self.offset..self.data.len() {
            self.data[i] = value;
        }
    }
}

pub struct SplitBuffer {
    buffers: Vec<Vec<u8>>,
    current_read: usize,
    current_offset: usize,
}

impl SplitBuffer {
    pub fn new() -> Self {
        Self {
            buffers: Vec::new(),
            current_read: 0,
            current_offset: 0,
        }
    }

    pub fn add_buffer(&mut self, buffer: Vec<u8>) {
        self.buffers.push(buffer);
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let mut written = 0;

        while written < buf.len() && self.current_read < self.buffers.len() {
            let current_buf = &self.buffers[self.current_read];
            let available = current_buf.len() - self.current_offset;
            let to_copy = available.min(buf.len() - written);

            buf[written..written + to_copy].copy_from_slice(
                &current_buf[self.current_offset..self.current_offset + to_copy]
            );

            written += to_copy;
            self.current_offset += to_copy;

            if self.current_offset >= current_buf.len() {
                self.current_read += 1;
                self.current_offset = 0;
            }
        }

        written
    }

    pub fn total_len(&self) -> usize {
        self.buffers.iter().map(|b| b.len()).sum()
    }

    pub fn remaining(&self) -> usize {
        if self.current_read >= self.buffers.len() {
            return 0;
        }

        let current_remaining = self.buffers[self.current_read].len() - self.current_offset;
        let rest: usize = self.buffers[self.current_read + 1..].iter().map(|b| b.len()).sum();
        current_remaining + rest
    }
}

impl Default for SplitBuffer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CowBuffer<'a> {
    data: CowData<'a>,
}

enum CowData<'a> {
    Borrowed(&'a [u8]),
    Owned(Vec<u8>),
}

impl<'a> CowBuffer<'a> {
    pub fn borrowed(data: &'a [u8]) -> Self {
        Self {
            data: CowData::Borrowed(data),
        }
    }

    pub fn owned(data: Vec<u8>) -> Self {
        Self {
            data: CowData::Owned(data),
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        match &self.data {
            CowData::Borrowed(slice) => slice,
            CowData::Owned(vec) => vec.as_slice(),
        }
    }

    pub fn to_mut(&mut self) -> &mut Vec<u8> {
        match &mut self.data {
            CowData::Borrowed(slice) => {
                let owned = slice.to_vec();
                self.data = CowData::Owned(owned);
                match &mut self.data {
                    CowData::Owned(vec) => vec,
                    _ => unreachable!(),
                }
            }
            CowData::Owned(vec) => vec,
        }
    }

    pub fn is_borrowed(&self) -> bool {
        matches!(self.data, CowData::Borrowed(_))
    }

    pub fn into_owned(self) -> Vec<u8> {
        match self.data {
            CowData::Borrowed(slice) => slice.to_vec(),
            CowData::Owned(vec) => vec,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_view() {
        let data = vec![1, 2, 3, 4, 5];
        let mut view = BufferView::new(&data);

        assert_eq!(view.remaining(), 5);
        assert_eq!(view.read_u8().unwrap(), 1);
        assert_eq!(view.remaining(), 4);
    }

    #[test]
    fn test_buffer_view_slice() {
        let data = vec![1, 2, 3, 4, 5];
        let view = BufferView::new(&data);
        let slice = view.slice(1, 4).unwrap();

        assert_eq!(slice.as_slice(), &[2, 3, 4]);
    }

    #[test]
    fn test_buffer_view_mut() {
        let mut data = vec![0u8; 5];
        let mut view = BufferViewMut::new(&mut data);

        view.write_u8(1).unwrap();
        view.write_u8(2).unwrap();

        assert_eq!(data[0], 1);
        assert_eq!(data[1], 2);
    }

    #[test]
    fn test_split_buffer() {
        let mut split = SplitBuffer::new();
        split.add_buffer(vec![1, 2, 3]);
        split.add_buffer(vec![4, 5, 6]);

        let mut buf = vec![0u8; 6];
        let read = split.read(&mut buf);

        assert_eq!(read, 6);
        assert_eq!(buf, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_cow_buffer() {
        let data = vec![1, 2, 3];
        let mut cow = CowBuffer::borrowed(&data);

        assert!(cow.is_borrowed());

        cow.to_mut().push(4);
        assert!(!cow.is_borrowed());
        assert_eq!(cow.as_slice(), &[1, 2, 3, 4]);
    }
}
