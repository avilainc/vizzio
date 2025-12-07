//! SIMD optimizations module

pub mod avx2;
pub mod avx512;
pub mod neon;
pub mod dispatch;
pub mod traits;

pub use avx2::*;
pub use avx512::*;
pub use neon::*;
pub use dispatch::*;
pub use traits::*;
