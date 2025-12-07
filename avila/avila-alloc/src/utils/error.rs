//! Error types for allocation failures

use core::fmt;

/// Allocation error type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocError {
    /// Insufficient capacity
    OutOfMemory,
    /// Invalid alignment
    InvalidAlignment,
    /// Invalid size
    InvalidSize,
}

impl fmt::Display for AllocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocError::OutOfMemory => write!(f, "out of memory"),
            AllocError::InvalidAlignment => write!(f, "invalid alignment"),
            AllocError::InvalidSize => write!(f, "invalid size"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AllocError {}
