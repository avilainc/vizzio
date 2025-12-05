//! Error types for cache operations
use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum CacheError {
    /// Cache has reached maximum capacity
    CapacityExceeded,
    /// Key not found in cache
    KeyNotFound,
    /// Invalid configuration
    InvalidConfig,
    /// Serialization error
    SerializationError,
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheError::CapacityExceeded => write!(f, "Cache capacity exceeded"),
            CacheError::KeyNotFound => write!(f, "Key not found in cache"),
            CacheError::InvalidConfig => write!(f, "Invalid cache configuration"),
            CacheError::SerializationError => write!(f, "Serialization error"),
        }
    }
}

pub type CacheResult<T> = Result<T, CacheError>;
