//! Módulo de I/O - Serialização e persistência

pub mod serialization;
pub mod formats;
pub mod streaming_io;
pub mod cache;

pub use serialization::*;
pub use formats::*;
pub use streaming_io::*;
pub use cache::*;
