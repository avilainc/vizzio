//! # avila-alloc - Stack-First Memory Allocation
//!
//! Zero-dependency memory allocation library prioritizing stack allocation.
//!
//! ## Features
//!
//! - **Stack-First**: Prefer stack over heap allocation
//! - **Arena Allocators**: Bump allocation for temporary data
//! - **Fixed-Size Pools**: Pre-allocated memory pools
//! - **Zero Dependencies**: Only depends on core/std
//! - **no_std Compatible**: Works in embedded environments
//! - **Type-Safe**: Leverages Rust's type system
//!
//! ## Philosophy
//!
//! Following Avila's zero-dependencies philosophy, this crate provides
//! memory allocation patterns that favor stack allocation and minimize
//! heap pressure. All allocators are deterministic and predictable.
//!
//! ## Examples
//!
//! ```rust
//! use avila_alloc::{StackVec, StackString};
//!
//! // Stack-allocated vector with capacity 32
//! let mut vec = StackVec::<u32, 32>::new();
//! vec.push(1).unwrap();
//! vec.push(2).unwrap();
//! assert_eq!(vec.len(), 2);
//!
//! // Stack-allocated string
//! let mut s = StackString::<64>::new();
//! s.push_str("Hello, World!").unwrap();
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

#[cfg(feature = "std")]
extern crate std;

pub mod stack;
pub mod arena;
pub mod pool;
pub mod utils;

#[cfg(test)]
pub mod test_helpers;

// Re-exports for convenience
pub use stack::{StackVec, StackString, StackBox, StackQueue, StackMap, StackRing};
pub use arena::StaticArena;
#[cfg(feature = "std")]
pub use arena::Arena;
pub use pool::{Pool, PoolSlot, PoolHandle};
pub use utils::AllocError;

/// Prelude module with commonly used types
pub mod prelude {
    pub use crate::stack::{StackVec, StackString, StackBox, StackQueue, StackMap, StackRing};
    pub use crate::arena::StaticArena;
    #[cfg(feature = "std")]
    pub use crate::arena::Arena;
    pub use crate::pool::Pool;
    pub use crate::utils::AllocError;
}

        pool.alloc(2).unwrap();
        assert!(pool.alloc(3).is_err());
    }
}
