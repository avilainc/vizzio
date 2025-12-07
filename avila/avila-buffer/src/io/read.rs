//! Read trait implementations for buffers

#[cfg(feature = "std")]
use std::io::{self, Read};

#[cfg(feature = "std")]
impl Read for crate::ByteBuffer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let available = self.available();
        let to_read = available.min(buf.len());

        if to_read == 0 {
            return Ok(0);
        }

        match self.read(&mut buf[..to_read]) {
            Ok(n) => Ok(n),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
        }
    }
}

// Additional read implementations to be added
