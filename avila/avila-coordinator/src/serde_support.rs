//! # Serde Support - Serialization support (optional feature)
//!
//! This module provides serialization/deserialization support using serde.
//! Enable with the "serde" feature flag.

#[cfg(feature = "serde")]
pub mod serde_impl {
    use serde::{Deserialize, Serialize};

    // Re-export types with serde derives when feature is enabled
    // In actual implementation, you would add #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    // to the actual type definitions

    pub use crate::types::{TaskId, TaskError, TaskResult};
    pub use crate::task::{Task, TaskState};
    pub use crate::priority::Priority;
}

#[cfg(not(feature = "serde"))]
pub mod serde_impl {
    // Stub module when serde is not enabled
}
