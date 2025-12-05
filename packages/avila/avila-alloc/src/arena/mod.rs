//! Arena allocators
//!
//! This module contains bump allocators for temporary allocations.

#[cfg(feature = "std")]
mod std_arena;
mod static_arena;

#[cfg(feature = "std")]
pub use std_arena::Arena;
pub use static_arena::StaticArena;
