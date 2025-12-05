//! Argument validation utilities
//!
//! Provides common validators and validation logic for command-line arguments.

mod validators;
mod groups;

pub use validators::*;
pub use groups::validate_groups;
