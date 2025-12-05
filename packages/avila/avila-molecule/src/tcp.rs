//! TCP (Transmission Control Protocol) - Conexões confiáveis
//!
//! Este módulo fornece implementação de TCP usando a biblioteca padrão,
//! com buffers gerenciados por `DynamicArray` do avila-atom.
//!
//! ## Características
//!
//! - **Servidor TCP**: Aceita múltiplas conexões
//! - **Cliente TCP**: Conecta a servidores remotos
//! - **Buffer Dinâmico**: Gerenciamento eficiente de memória
//! - **API Síncrona**: Operações bloqueantes para máximo controle
//!
//! ## Exemplo
//!
//! ```rust,no_run
//! use avila_molecule::tcp::{TcpServer, TcpClient};
//! use avila_molecule::NetworkAddress;
//!
//! // Servidor
//! let server = TcpServer::bind(NetworkAddress::new("127.0.0.1", 8080)).unwrap();
//! let (mut conn, addr) = server.accept().unwrap();
//!
//! // Cliente
//! let mut client = TcpClient::connect(NetworkAddress::new("127.0.0.1", 8080)).unwrap();
//! client.send(b"Hello").unwrap();
//! ```

use crate::NetworkAddress;
use avila_error::{Error, ErrorKind, Result};
use std::net::{TcpListener as StdTcpListener, TcpStream as StdTcpStream, SocketAddr};
use std::io::{Read, Write};
use avila_atom::DynamicArray;

/// Servidor TCP
///
/// Escuta em um endereço e porta específicos, aceitando conexões de entrada.
pub struct TcpServer {
    listener: StdTcpListener,
    address: NetworkAddress,
}

impl TcpServer {
    /// Cria novo servidor TCP
    pub fn bind(address: NetworkAddress) -> Result<Self> {
        let listener = StdTcpListener::bind(address.to_string())
            .map_err(|e| Error::new(ErrorKind::Network, format!("Falha ao vincular TCP: {}", e)))?;

        listener.set_nonblocking(false)
            .map_err(|e| Error::new(ErrorKind::Network, format!("Falha ao configurar TCP: {}", e)))?;

        Ok(Self { listener, address })
    }

    /// Aceita nova conexão
    ///
    /// Bloqueia até que uma nova conexão seja estabelecida.
    ///
    /// # Retorna
    ///
    /// Tupla contendo a conexão TCP e o endereço do cliente.
    ///
    /// # Erros
    ///
    /// Retorna erro se falhar ao aceitar a conexão.
    pub fn accept(&self) -> Result<(TcpConnection, SocketAddr)> {
        let (stream, addr) = self.listener
            .accept()
            .map_err(|e| Error::new(ErrorKind::Network, format!("Falha ao aceitar conexão: {}", e)))?;

        Ok((TcpConnection { stream, buffer: DynamicArray::new() }, addr))
    }

    /// Endereço do servidor
    pub fn address(&self) -> &NetworkAddress {
        &self.address
    }
}

/// Conexão TCP estabelecida
///
/// Representa uma conexão TCP bidirecional com buffer interno
/// para operações eficientes de I/O.
pub struct TcpConnection {
    /// Stream TCP subjacente
    stream: StdTcpStream,
    /// Buffer dinâmico para operações com buffer
    buffer: DynamicArray<u8>,
}

impl TcpConnection {
    /// Envia dados através da conexão
    ///
    /// # Argumentos
    ///
    /// * `data` - Bytes a serem enviados
    ///
    /// # Erros
    ///
    /// Retorna erro se a operação de escrita falhar.
    pub fn send(&mut self, data: &[u8]) -> Result<()> {
        self.stream
            .write_all(data)
            .map_err(|e| Error::new(ErrorKind::Io, format!("Falha ao enviar: {}", e)))
    }

    /// Recebe dados
    pub fn receive(&mut self, buffer: &mut [u8]) -> Result<usize> {
        self.stream
            .read(buffer)
            .map_err(|e| Error::new(ErrorKind::Io, format!("Falha ao receber: {}", e)))
    }

    /// Recebe dados em buffer interno
    pub fn receive_buffered(&mut self, size: usize) -> Result<&[u8]> {
        self.buffer.clear();
        let mut temp = vec![0u8; size];
        let n = self.receive(&mut temp)?;
        for i in 0..n {
            self.buffer.push(temp[i]);
        }
        Ok(self.buffer.as_slice())
    }
}

/// Cliente TCP
pub struct TcpClient {
    connection: TcpConnection,
}

impl TcpClient {
    /// Conecta a um servidor
    pub fn connect(address: NetworkAddress) -> Result<Self> {
        let stream = StdTcpStream::connect(address.to_string())
            .map_err(|e| Error::new(ErrorKind::Network, format!("Falha ao conectar: {}", e)))?;

        Ok(Self {
            connection: TcpConnection {
                stream,
                buffer: DynamicArray::new(),
            }
        })
    }

    /// Envia dados
    pub fn send(&mut self, data: &[u8]) -> Result<()> {
        self.connection.send(data)
    }

    /// Recebe dados
    pub fn receive(&mut self, buffer: &mut [u8]) -> Result<usize> {
        self.connection.receive(buffer)
    }

    /// Acesso à conexão
    pub fn connection(&mut self) -> &mut TcpConnection {
        &mut self.connection
    }
}
