//! Buffer pooling system for reusing buffers
//! Reduces allocation overhead in high-throughput scenarios

extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::VecDeque;
use crate::ByteBuffer;
use avila_error::{Error, ErrorKind, Result};

/// Buffer pool for efficient buffer reuse
///
/// # Examples
///
/// ```
/// use avila_buffer::BufferPool;
///
/// let pool = BufferPool::new(10, 1024);
/// let buffer = pool.acquire().unwrap();
/// // use buffer...
/// pool.release(buffer);
/// ```
pub struct BufferPool {
    pool: VecDeque<ByteBuffer>,
    capacity: usize,
    buffer_size: usize,
    total_created: usize,
    total_acquired: usize,
    total_released: usize,
}

impl BufferPool {
    /// Creates new buffer pool
    ///
    /// # Arguments
    /// * `capacity` - Maximum number of buffers in pool
    /// * `buffer_size` - Size of each buffer
    pub fn new(capacity: usize, buffer_size: usize) -> Self {
        let mut pool = VecDeque::with_capacity(capacity);

        // Pre-allocate buffers
        for _ in 0..capacity {
            pool.push_back(ByteBuffer::with_capacity(buffer_size));
        }

        Self {
            pool,
            capacity,
            buffer_size,
            total_created: capacity,
            total_acquired: 0,
            total_released: 0,
        }
    }

    /// Creates pool with lazy allocation
    pub fn with_capacity(capacity: usize, buffer_size: usize) -> Self {
        Self {
            pool: VecDeque::with_capacity(capacity),
            capacity,
            buffer_size,
            total_created: 0,
            total_acquired: 0,
            total_released: 0,
        }
    }

    /// Acquires a buffer from the pool
    pub fn acquire(&mut self) -> Result<ByteBuffer> {
        self.total_acquired += 1;

        if let Some(mut buffer) = self.pool.pop_front() {
            buffer.reset();
            Ok(buffer)
        } else if self.total_created < self.capacity {
            self.total_created += 1;
            Ok(ByteBuffer::with_capacity(self.buffer_size))
        } else {
            Err(Error::new(
                ErrorKind::InvalidState,
                "Buffer pool exhausted",
            ))
        }
    }

    /// Releases a buffer back to the pool
    pub fn release(&mut self, mut buffer: ByteBuffer) {
        self.total_released += 1;

        if self.pool.len() < self.capacity {
            buffer.reset();
            self.pool.push_back(buffer);
        }
        // If pool is full, just drop the buffer
    }

    /// Returns number of available buffers
    pub fn available(&self) -> usize {
        self.pool.len()
    }

    /// Returns pool capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns buffer size
    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }

    /// Returns total buffers created
    pub fn total_created(&self) -> usize {
        self.total_created
    }

    /// Returns total acquisitions
    pub fn total_acquired(&self) -> usize {
        self.total_acquired
    }

    /// Returns total releases
    pub fn total_released(&self) -> usize {
        self.total_released
    }

    /// Clears the pool
    pub fn clear(&mut self) {
        self.pool.clear();
        self.total_created = 0;
        self.total_acquired = 0;
        self.total_released = 0;
    }

    /// Shrinks pool to fit
    pub fn shrink_to_fit(&mut self) {
        self.pool.shrink_to_fit();
    }
}

impl Default for BufferPool {
    fn default() -> Self {
        Self::new(10, 1024)
    }
}

/// RAII guard for automatic buffer release
pub struct PooledBuffer<'a> {
    buffer: Option<ByteBuffer>,
    pool: &'a mut BufferPool,
}

impl<'a> PooledBuffer<'a> {
    /// Creates new pooled buffer
    pub fn new(pool: &'a mut BufferPool) -> Result<Self> {
        let buffer = pool.acquire()?;
        Ok(Self {
            buffer: Some(buffer),
            pool,
        })
    }

    /// Get reference to buffer
    pub fn buffer(&self) -> &ByteBuffer {
        self.buffer.as_ref().unwrap()
    }

    /// Get mutable reference to buffer
    pub fn buffer_mut(&mut self) -> &mut ByteBuffer {
        self.buffer.as_mut().unwrap()
    }
}

impl<'a> Drop for PooledBuffer<'a> {
    fn drop(&mut self) {
        if let Some(buffer) = self.buffer.take() {
            self.pool.release(buffer);
        }
    }
}

impl<'a> core::ops::Deref for PooledBuffer<'a> {
    type Target = ByteBuffer;

    fn deref(&self) -> &Self::Target {
        self.buffer()
    }
}

impl<'a> core::ops::DerefMut for PooledBuffer<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.buffer_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_basic() {
        let mut pool = BufferPool::new(3, 128);
        assert_eq!(pool.capacity(), 3);
        assert_eq!(pool.available(), 3);

        let buf1 = pool.acquire().unwrap();
        assert_eq!(pool.available(), 2);

        pool.release(buf1);
        assert_eq!(pool.available(), 3);
    }

    #[test]
    fn test_pool_exhaustion() {
        let mut pool = BufferPool::with_capacity(2, 128);

        let _buf1 = pool.acquire().unwrap();
        let _buf2 = pool.acquire().unwrap();

        assert!(pool.acquire().is_err());
    }

    #[test]
    fn test_pooled_buffer_raii() {
        let mut pool = BufferPool::new(2, 128);

        {
            let mut buf = PooledBuffer::new(&mut pool).unwrap();
            buf.write(b"test").unwrap();
            assert_eq!(pool.available(), 1);
        } // Buffer automatically released here

        assert_eq!(pool.available(), 2);
    }

    #[test]
    fn test_pool_statistics() {
        let mut pool = BufferPool::new(2, 128);

        let buf = pool.acquire().unwrap();
        assert_eq!(pool.total_acquired(), 1);

        pool.release(buf);
        assert_eq!(pool.total_released(), 1);
    }
}
