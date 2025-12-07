//! Write trait implementations for buffers

#[cfg(feature = "std")]
use std::io::{self, Write};

#[cfg(feature = "std")]
impl Write for crate::ByteBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.write(buf) {
            Ok(n) => Ok(n),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// Additional write implementations to be added
