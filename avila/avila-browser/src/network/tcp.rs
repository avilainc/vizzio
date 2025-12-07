//! TCP stream abstraction

use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::time::Duration;

/// Async TCP stream wrapper
pub struct TcpStream {
    inner: std::net::TcpStream,
}

impl TcpStream {
    /// Connect to a remote address
    pub fn connect(addr: SocketAddr, timeout: Duration) -> io::Result<Self> {
        // TODO: Implement async TCP connection with timeout
        let inner = std::net::TcpStream::connect_timeout(&addr, timeout)?;
        Ok(Self { inner })
    }

    /// Set read timeout
    pub fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.inner.set_read_timeout(timeout)
    }

    /// Set write timeout
    pub fn set_write_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.inner.set_write_timeout(timeout)
    }

    /// Enable TCP keepalive
    pub fn set_keepalive(&self, keepalive: bool) -> io::Result<()> {
        // TODO: Implement TCP keepalive configuration
        Ok(())
    }
}

impl Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
