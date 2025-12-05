//! Memory-mapped buffer simulation for zero-copy operations

use alloc::vec::Vec;
use avila_error::{Error, ErrorKind, Result};
use core::ops::{Deref, DerefMut};

pub struct MappedBuffer {
    data: Vec<u8>,
    read_only: bool,
    offset: usize,
    length: usize,
}

impl MappedBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0u8; size],
            read_only: false,
            offset: 0,
            length: size,
        }
    }

    pub fn from_vec(data: Vec<u8>) -> Self {
        let len = data.len();
        Self {
            data,
            read_only: false,
            offset: 0,
            length: len,
        }
    }

    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    pub fn slice(&self, offset: usize, length: usize) -> Result<MappedView> {
        if offset + length > self.length {
            return Err(Error::new(ErrorKind::OutOfBounds, "View out of bounds"));
        }

        Ok(MappedView {
            buffer: self,
            offset: self.offset + offset,
            length,
        })
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data[self.offset..self.offset + self.length]
    }

    pub fn as_mut_slice(&mut self) -> Result<&mut [u8]> {
        if self.read_only {
            return Err(Error::new(ErrorKind::InvalidOperation, "Buffer is read-only"));
        }
        Ok(&mut self.data[self.offset..self.offset + self.length])
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn flush(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn resize(&mut self, new_size: usize) -> Result<()> {
        if self.read_only {
            return Err(Error::new(ErrorKind::InvalidOperation, "Cannot resize read-only buffer"));
        }
        self.data.resize(new_size, 0);
        self.length = new_size;
        Ok(())
    }
}

pub struct MappedView<'a> {
    buffer: &'a MappedBuffer,
    offset: usize,
    length: usize,
}

impl<'a> MappedView<'a> {
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer.data[self.offset..self.offset + self.length]
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl<'a> Deref for MappedView<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

pub struct PageAllocator {
    page_size: usize,
    pages: Vec<Vec<u8>>,
}

impl PageAllocator {
    pub fn new(page_size: usize) -> Self {
        Self {
            page_size,
            pages: Vec::new(),
        }
    }

    pub fn allocate(&mut self, size: usize) -> PageAllocation {
        let pages_needed = (size + self.page_size - 1) / self.page_size;
        let start_page = self.pages.len();

        for _ in 0..pages_needed {
            self.pages.push(vec![0u8; self.page_size]);
        }

        PageAllocation {
            start_page,
            page_count: pages_needed,
            size,
        }
    }

    pub fn get_page(&self, index: usize) -> Option<&[u8]> {
        self.pages.get(index).map(|p| p.as_slice())
    }

    pub fn get_page_mut(&mut self, index: usize) -> Option<&mut [u8]> {
        self.pages.get_mut(index).map(|p| p.as_mut_slice())
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn total_allocated(&self) -> usize {
        self.pages.len() * self.page_size
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PageAllocation {
    pub start_page: usize,
    pub page_count: usize,
    pub size: usize,
}

pub struct Arena {
    buffer: Vec<u8>,
    allocated: usize,
}

impl Arena {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            allocated: 0,
        }
    }

    pub fn allocate(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        let align_offset = (align - (self.allocated % align)) % align;
        let start = self.allocated + align_offset;
        let end = start + size;

        if end > self.buffer.capacity() {
            return None;
        }

        while self.buffer.len() < end {
            self.buffer.push(0);
        }

        self.allocated = end;
        Some(unsafe { self.buffer.as_mut_ptr().add(start) })
    }

    pub fn reset(&mut self) {
        self.allocated = 0;
        self.buffer.clear();
    }

    pub fn allocated_bytes(&self) -> usize {
        self.allocated
    }

    pub fn available_bytes(&self) -> usize {
        self.buffer.capacity() - self.allocated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapped_buffer() {
        let mut buf = MappedBuffer::new(100);
        assert_eq!(buf.len(), 100);

        let slice = buf.as_mut_slice().unwrap();
        slice[0] = 42;

        assert_eq!(buf.as_slice()[0], 42);
    }

    #[test]
    fn test_mapped_view() {
        let buf = MappedBuffer::from_vec(vec![1, 2, 3, 4, 5]);
        let view = buf.slice(1, 3).unwrap();

        assert_eq!(view.as_slice(), &[2, 3, 4]);
    }

    #[test]
    fn test_read_only() {
        let mut buf = MappedBuffer::new(10).read_only();
        assert!(buf.as_mut_slice().is_err());
    }

    #[test]
    fn test_page_allocator() {
        let mut alloc = PageAllocator::new(4096);
        let allocation = alloc.allocate(8192);

        assert_eq!(allocation.page_count, 2);
        assert_eq!(alloc.page_count(), 2);
    }

    #[test]
    fn test_arena() {
        let mut arena = Arena::with_capacity(1024);

        let ptr1 = arena.allocate(100, 8).unwrap();
        let ptr2 = arena.allocate(100, 8).unwrap();

        assert_ne!(ptr1, ptr2);
        assert_eq!(arena.allocated_bytes(), 200);
    }
}
