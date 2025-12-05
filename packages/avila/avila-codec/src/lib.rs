//! # avila-codec - Encoding/Decoding Utilities
//!
//! Fast and secure encoding/decoding for common formats.
//!
//! ## Features
//!
//! - **Hex Encoding** - Fast hex encode/decode
//! - **Base64** - Standard base64 encoding
//! - **Base58** - Bitcoin-style base58 encoding
//! - **Base32** - RFC 4648 base32 encoding
//! - **Base85** - ASCII85 and Z85 variants
//! - **URL Encoding** - Percent encoding (RFC 3986)
//! - **Multibase** - IPFS-style self-describing encodings
//! - **Checksums** - CRC32, XXHash
//! - **VarInt** - Variable-length integer encoding
//! - **Zero Dependencies** - Pure Rust implementation
//! - **no_std Compatible** - Works in embedded environments
//! - **Constant-Time** - Side-channel resistant operations
//!
//! ## Examples
//!
//! ```rust
//! use avila_codec::{hex, base64, base32};
//!
//! // Hex encoding
//! let data = b"Hello";
//! let encoded = hex::encode(data);
//! assert_eq!(encoded, "48656c6c6f");
//!
//! // Base64 encoding
//! let encoded = base64::encode(data);
//! assert_eq!(encoded, "SGVsbG8=");
//!
//! // Base32 encoding
//! let encoded = base32::encode(data);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

extern crate alloc;

// Internal error module
mod error;
pub use error::{Error, ErrorKind, Result};

// Core encoding modules
pub mod hex;
pub mod base64;
pub mod base58;
pub mod base32;
pub mod base85;
pub mod url;
pub mod multibase;

// Advanced features
pub mod checksum;
pub mod binary;
pub mod compression;
pub mod network;

// Infrastructure
pub mod traits;
pub mod simd;

/// Prelude with commonly used functions
pub mod prelude {
    pub use crate::{base32, base58, base64, base85, hex, multibase, url};
    pub use crate::checksum::{crc, xxhash};
    pub use crate::binary::varint;
    pub use crate::compression;
}
