//! Utility types and traits
//!
//! This module contains helper types used across the library.

pub mod error;
pub mod alignment;
pub mod layout;
pub mod ptr;
pub mod const_assert;
pub mod bits;

pub use error::AllocError;
pub use alignment::{align_up, align_down, is_aligned};
pub use layout::{LayoutExt, MemStats};
pub use ptr::{PtrExt, NonNullExt, SliceExt};
pub use const_assert::*;
pub use bits::{BitOps, Bitmap};
