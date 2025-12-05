//! UDP (User Datagram Protocol) - Comunicação sem conexão
//!
//! Este módulo fornece implementação de UDP usando a biblioteca padrão,
//! ideal para comunicação de baixa latência onde perda de pacotes é aceitável.
//!
//! ## Características
//!
//! - **Sem Conexão**: Envia datagramas sem estabelecer conexão
//! - **Baixa Latência**: Overhead mínimo de protocolo
//! - **Buffer Dinâmico**: Gerenciamento eficiente com avila-atom
//! - **Bidirecional**: Envio e recebimento no mesmo socket
//!
//! ## Exemplo
//!
//! ```rust,no_run
//! use avila_molecule::udp::UdpEndpoint;
//! use avila_molecule::NetworkAddress;
//!
//! let endpoint = UdpEndpoint::bind(NetworkAddress::new("127.0.0.1", 9000)).unwrap();
//! let target = NetworkAddress::new("127.0.0.1", 9001);
//! endpoint.send_to(b"Hello UDP", &target).unwrap();
//! ```

use crate::NetworkAddress;
use avila_error::{Error, ErrorKind, Result};
use std::net::{UdpSocket as StdUdpSocket, SocketAddr};
use avila_atom::DynamicArray;

/// Socket UDP para comunicação sem conexão
///
/// Permite enviar e receber datagramas UDP de/para múltiplos destinos.
pub struct UdpEndpoint {
    socket: StdUdpSocket,
    address: NetworkAddress,
    buffer: DynamicArray<u8>,
}

impl UdpEndpoint {
    /// Cria novo endpoint UDP
    pub fn bind(address: NetworkAddress) -> Result<Self> {
        let socket = StdUdpSocket::bind(address.to_string())
            .map_err(|e| Error::new(ErrorKind::Network, format!("Falha ao vincular UDP: {}", e)))?;

        socket.set_nonblocking(false)
            .map_err(|e| Error::new(ErrorKind::Network, format!("Falha ao configurar UDP: {}", e)))?;

        Ok(Self {
            socket,
            address,
            buffer: DynamicArray::new(),
        })
    }

    /// Envia datagrama para destino específico
    ///
    /// # Argumentos
    ///
    /// * `data` - Bytes a serem enviados
    /// * `target` - Endereço de destino
    ///
    /// # Retorna
    ///
    /// Número de bytes enviados
    ///
    /// # Erros
    ///
    /// Retorna erro se falhar ao enviar o datagrama.
    pub fn send_to(&self, data: &[u8], target: &NetworkAddress) -> Result<usize> {
        self.socket
            .send_to(data, target.to_string().as_str())
            .map_err(|e| Error::new(ErrorKind::Network, format!("Falha ao enviar: {}", e)))
    }

    /// Recebe datagrama
    pub fn receive_from(&self, buffer: &mut [u8]) -> Result<(usize, SocketAddr)> {
        self.socket
            .recv_from(buffer)
            .map_err(|e| Error::new(ErrorKind::Network, format!("Falha ao receber: {}", e)))
    }

    /// Recebe datagrama em buffer interno
    pub fn receive_from_buffered(&mut self, size: usize) -> Result<(usize, SocketAddr)> {
        self.buffer.clear();
        let mut temp = vec![0u8; size];
        let (n, addr) = self.receive_from(&mut temp)?;
        for i in 0..n {
            self.buffer.push(temp[i]);
        }
        Ok((n, addr))
    }

    /// Acesso ao buffer interno
    pub fn buffer(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    /// Endereço local
    pub fn address(&self) -> &NetworkAddress {
        &self.address
    }
}
