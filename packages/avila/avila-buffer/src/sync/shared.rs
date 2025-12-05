//! Shared buffer for concurrent access

#[cfg(feature = "std")]
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use avila_error::{Error, ErrorKind, Result};

/// Thread-safe shared buffer
///
/// # Examples
///
/// ```no_run
/// use avila_buffer::SharedBuffer;
/// use std::thread;
///
/// let shared = SharedBuffer::with_capacity(1024);
/// let clone = shared.clone();
///
/// thread::spawn(move || {
///     clone.write(b"Hello from thread").unwrap();
/// });
/// ```
#[cfg(feature = "std")]
pub struct SharedBuffer {
    inner: Arc<RwLock<crate::ByteBuffer>>,
}

#[cfg(feature = "std")]
impl SharedBuffer {
    /// Create a new shared buffer
    pub fn new(buffer: crate::ByteBuffer) -> Self {
        Self {
            inner: Arc::new(RwLock::new(buffer)),
        }
    }

    /// Create a new shared buffer with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self::new(crate::ByteBuffer::with_capacity(capacity))
    }

    /// Write data to buffer
    pub fn write(&self, data: &[u8]) -> Result<usize> {
        self.inner
            .write()
            .map_err(|_| Error::new(ErrorKind::InvalidState, "Lock poisoned"))?
            .write(data)
    }

    /// Read data from buffer
    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        self.inner
            .write()
            .map_err(|_| Error::new(ErrorKind::InvalidState, "Lock poisoned"))?
            .read(buf)
    }

    /// Peek at data without consuming
    pub fn peek(&self, buf: &mut [u8]) -> Result<usize> {
        self.inner
            .read()
            .map_err(|_| Error::new(ErrorKind::InvalidState, "Lock poisoned"))?
            .peek(buf)
    }

    /// Get available bytes
    pub fn available(&self) -> Result<usize> {
        Ok(self
            .inner
            .read()
            .map_err(|_| Error::new(ErrorKind::InvalidState, "Lock poisoned"))?
            .available())
    }

    /// Get capacity
    pub fn capacity(&self) -> Result<usize> {
        Ok(self
            .inner
            .read()
            .map_err(|_| Error::new(ErrorKind::InvalidState, "Lock poisoned"))?
            .capacity())
    }

    /// Check if empty
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self
            .inner
            .read()
            .map_err(|_| Error::new(ErrorKind::InvalidState, "Lock poisoned"))?
            .is_empty())
    }

    /// Get length
    pub fn len(&self) -> Result<usize> {
        Ok(self
            .inner
            .read()
            .map_err(|_| Error::new(ErrorKind::InvalidState, "Lock poisoned"))?
            .len())
    }

    /// Reset buffer
    pub fn reset(&self) -> Result<()> {
        self.inner
            .write()
            .map_err(|_| Error::new(ErrorKind::InvalidState, "Lock poisoned"))?
            .reset();
        Ok(())
    }

    /// Clear buffer
    pub fn clear(&self) -> Result<()> {
        self.inner
            .write()
            .map_err(|_| Error::new(ErrorKind::InvalidState, "Lock poisoned"))?
            .clear();
        Ok(())
    }

    /// Compact buffer
    pub fn compact(&self) -> Result<()> {
        self.inner
            .write()
            .map_err(|_| Error::new(ErrorKind::InvalidState, "Lock poisoned"))?
            .compact();
        Ok(())
    }

    /// Get read lock for direct access
    pub fn read_lock(&self) -> Result<RwLockReadGuard<'_, crate::ByteBuffer>> {
        self.inner
            .read()
            .map_err(|_| Error::new(ErrorKind::InvalidState, "Lock poisoned"))
    }

    /// Get write lock for direct access
    pub fn write_lock(&self) -> Result<RwLockWriteGuard<'_, crate::ByteBuffer>> {
        self.inner
            .write()
            .map_err(|_| Error::new(ErrorKind::InvalidState, "Lock poisoned"))
    }

    /// Get reference count
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }

    /// Get weak reference count
    pub fn weak_count(&self) -> usize {
        Arc::weak_count(&self.inner)
    }

    /// Execute a closure with read access
    pub fn with_read<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&crate::ByteBuffer) -> R,
    {
        let guard = self.read_lock()?;
        Ok(f(&*guard))
    }

    /// Execute a closure with write access
    pub fn with_write<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut crate::ByteBuffer) -> R,
    {
        let mut guard = self.write_lock()?;
        Ok(f(&mut *guard))
    }
}

#[cfg(feature = "std")]
impl Clone for SharedBuffer {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

#[cfg(feature = "std")]
impl Default for SharedBuffer {
    fn default() -> Self {
        Self::with_capacity(0)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_shared_buffer_basic() {
        let shared = SharedBuffer::with_capacity(128);

        shared.write(b"Hello").unwrap();
        assert_eq!(shared.len().unwrap(), 5);

        let mut buf = [0u8; 5];
        shared.read(&mut buf).unwrap();
        assert_eq!(&buf, b"Hello");
    }

    #[test]
    fn test_shared_buffer_clone() {
        let shared = SharedBuffer::with_capacity(128);
        shared.write(b"Test").unwrap();

        let cloned = shared.clone();
        assert_eq!(cloned.len().unwrap(), 4);
        assert_eq!(shared.strong_count(), 2);
    }

    #[test]
    fn test_shared_buffer_threads() {
        let shared = SharedBuffer::with_capacity(1024);
        let clone = shared.clone();

        let handle = thread::spawn(move || {
            clone.write(b"From thread").unwrap();
        });

        handle.join().unwrap();
        assert_eq!(shared.len().unwrap(), 11);
    }

    #[test]
    fn test_shared_buffer_with_closures() {
        let shared = SharedBuffer::with_capacity(128);

        shared.with_write(|buf| {
            buf.write(b"Closure write").unwrap();
        }).unwrap();

        let len = shared.with_read(|buf| {
            buf.len()
        }).unwrap();

        assert_eq!(len, 13);
    }
}
