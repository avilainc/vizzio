//! Stack-based allocators
//!
//! This module contains allocators that use stack memory exclusively.

mod vec;
mod string;
mod box_type;
mod queue;
mod map;
mod ring;

pub use vec::{StackVec, IntoIter};
pub use string::StackString;
pub use box_type::StackBox;
pub use queue::{StackQueue, QueueIter};
pub use map::{StackMap, MapIter};
pub use ring::{StackRing, RingIter};
