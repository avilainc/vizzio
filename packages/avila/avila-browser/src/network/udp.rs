//! UDP socket abstraction

use std::io;
use std::net::SocketAddr;

/// Async UDP socket wrapper
pub struct UdpSocket {
    inner: std::net::UdpSocket,
}

impl UdpSocket {
    /// Bind to local address
    pub fn bind(addr: SocketAddr) -> io::Result<Self> {
        let inner = std::net::UdpSocket::bind(addr)?;
        Ok(Self { inner })
    }

    /// Send datagram to address
    pub fn send_to(&self, buf: &[u8], target: SocketAddr) -> io::Result<usize> {
        self.inner.send_to(buf, target)
    }

    /// Receive datagram
    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.inner.recv_from(buf)
    }

    /// Connect to remote address (filters received packets)
    pub fn connect(&self, addr: SocketAddr) -> io::Result<()> {
        self.inner.connect(addr)
    }
}
