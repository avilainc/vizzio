//! # Avila Cell Core
//!
//! Core types and traits for the avila-cell ecosystem.
//!
//! This crate provides fundamental abstractions for:
//! - Cell identity and lifecycle
//! - Message passing between cells
//! - State management
//! - Composition patterns

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![warn(clippy::all)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

pub mod cell;
pub mod message;
pub mod state;
pub mod lifecycle;

// Re-export error types
pub use thiserror::Error as ThisError;

/// Result type for avila-cell-core operations
pub type Result<T> = core::result::Result<T, Error>;

/// Error types for avila-cell-core
#[derive(Debug, ThisError)]
pub enum Error {
    /// Invalid state transition
    #[error("Invalid state: {0}")]
    InvalidState(&'static str),
    /// Message error
    #[error("Message error: {0}")]
    Message(String),
    /// Cell error
    #[error("Cell error: {0}")]
    Cell(String),
}

/// Error kind (for compatibility)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// Invalid state
    InvalidState,
    /// Message error
    Message,
    /// Cell error
    Cell,
    /// Invalid input
    InvalidInput,
}

impl Error {
    /// Create a new error
    pub fn new(kind: ErrorKind, message: &'static str) -> Self {
        match kind {
            ErrorKind::InvalidState => Error::InvalidState(message),
            ErrorKind::Message => Error::Message(message.to_string()),
            ErrorKind::Cell => Error::Cell(message.to_string()),
            ErrorKind::InvalidInput => Error::Message(message.to_string()),
        }
    }
}

pub mod prelude {
    //! Common imports for convenience
    pub use crate::cell::{Cell, CellTrait};
    pub use crate::message::{Message, MessageTrait};
    pub use crate::state::{State, StateTrait};
    pub use crate::lifecycle::{Lifecycle, LifecycleStage};
    pub use crate::{Error, ErrorKind, Result};
}
