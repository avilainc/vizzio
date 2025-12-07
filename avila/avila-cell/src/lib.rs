//! # avila-cell
//!
//! **Células Digitais - Protocolos de Email**
//!
//! Assim como células são a primeira forma de vida que pode se comunicar,
//! processar informações e se replicar, os protocolos de email (SMTP, POP3, IMAP)
//! são as primeiras formas de "vida digital" que podem:
//!
//! - Comunicar (enviar/receber mensagens)
//! - Processar (parse, validação, roteamento)
//! - Persistir (armazenamento de mensagens)
//!
//! ## Protocolos Suportados
//!
//! - **SMTP** (Simple Mail Transfer Protocol) - Envio de emails
//! - **POP3** (Post Office Protocol v3) - Recebimento de emails (TODO)
//! - **IMAP** (Internet Message Access Protocol) - Acesso a caixas de email (TODO)
//!
//! ## Filosofia
//!
//! Email é a forma mais fundamental de comunicação digital assíncrona.
//! É o "DNA" da internet - simples, robusto, descentralizado.

#![allow(missing_docs)]
#![warn(clippy::all)]

// Re-export core error types
pub use thiserror::Error as ThisError;

pub mod smtp;
pub mod message;
pub mod mime;
pub mod encoding;
pub mod auth;
pub mod tcp;
pub mod notification;

// TODO: Enable when dependencies are available
// pub mod pop3;
// pub mod imap;
// pub mod queue;
// pub mod template;
// pub mod dkim;
// pub mod pool;
// pub mod classifier;
// pub mod calendar;

// Error types
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Authentication failed: {0}")]
    Auth(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Email address structure
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmailAddress {
    /// Local part (before @)
    pub local: String,
    /// Domain (after @)
    pub domain: String,
    /// Optional display name
    pub display_name: Option<String>,
}

impl EmailAddress {
    /// Create new email address from string
    pub fn new(email: impl AsRef<str>) -> Result<Self> {
        let email = email.as_ref();
        let parts: Vec<&str> = email.split('@').collect();

        if parts.len() != 2 {
            return Err(Box::new(Error::InvalidInput(
                format!("Invalid email format: {}", email)
            )));
        }

        Ok(Self {
            local: parts[0].to_string(),
            domain: parts[1].to_string(),
            display_name: None,
        })
    }

    /// Create email with display name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.display_name = Some(name.into());
        self
    }

    /// Convert to string (email only)
    pub fn to_email_string(&self) -> String {
        format!("{}@{}", self.local, self.domain)
    }

    /// Format for RFC 5322 (with display name if present)
    pub fn to_rfc5322(&self) -> String {
        match &self.display_name {
            Some(name) => format!("\"{}\" <{}@{}>", name, self.local, self.domain),
            None => self.to_email_string(),
        }
    }

    /// Validate email format
    pub fn is_valid(&self) -> bool {
        !self.local.is_empty()
            && !self.domain.is_empty()
            && self.domain.contains('.')
    }
}

impl std::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_rfc5322())
    }
}

impl std::str::FromStr for EmailAddress {
    type Err = Box<dyn std::error::Error + Send + Sync>;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}
