//! # Avila Alert
//!
//! A flexible and ergonomic alert system for Rust applications.
//! Pure Rust implementation with zero external dependencies.
//!
//! ## Features
//!
//! - Multiple alert levels (Trace, Debug, Info, Warning, Error, Critical)
//! - Flexible formatting options (Simple, Detailed, JSON, Compact)
//! - Builder pattern for complex alerts
//! - Handler system for custom alert processing
//! - Thread-safe alert management
//! - Timestamp support (pure Rust implementation)
//! - No external dependencies
//!
//! ## Quick Start
//!
//! ```rust
//! use avila_alert::{Alert, AlertLevel, AlertManager, ConsoleHandler, SimpleFormatter};
//!
//! let manager = AlertManager::new();
//! manager.add_handler(Box::new(ConsoleHandler::new(SimpleFormatter)));
//!
//! manager.dispatch(Alert::info("Application started"));
//! manager.dispatch(Alert::warning("Low memory detected"));
//! ```
//!
//! ## Using the Builder Pattern
//!
//! ```rust
//! use avila_alert::{AlertBuilder, AlertLevel};
//!
//! let alert = AlertBuilder::new()
//!     .level(AlertLevel::Warning)
//!     .message("Authentication failed")
//!     .tag("security")
//!     .tag("auth")
//!     .context("user", "john.doe")
//!     .build()
//!     .unwrap();
//! ```

pub mod types;
pub mod builder;
pub mod formatter;
pub mod handler;
pub mod manager;
pub mod macros;

// Re-exports for convenience
pub use types::{Alert, AlertLevel, Timestamp};
pub use builder::AlertBuilder;
pub use formatter::{AlertFormatter, SimpleFormatter, DetailedFormatter, JsonFormatter, CompactFormatter};
pub use handler::{AlertHandler, ConsoleHandler, CallbackHandler, MultiHandler, FilterHandler, BufferHandler};
pub use manager::AlertManager;
