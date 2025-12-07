//! Error types for avila-codec

use core::fmt;

/// Result type alias using our Error type
pub type Result<T> = core::result::Result<T, Error>;

/// Error kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// Invalid input data
    InvalidInput,
    /// Unsupported operation
    Unsupported,
    /// Buffer too small
    BufferTooSmall,
    /// Encoding error
    EncodingError,
    /// Decoding error
    DecodingError,
}

/// Error type for codec operations
#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    message: &'static str,
}

impl Error {
    /// Creates a new error with the given kind and message
    pub fn new(kind: ErrorKind, message: &'static str) -> Self {
        Self { kind, message }
    }

    /// Returns the error kind
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Returns the error message
    pub fn message(&self) -> &str {
        self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
