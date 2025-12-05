//! # avila-nucleus
//!
//! Fundação atômica da pilha criptográfica Ávila: operações de baixíssimo
//! nível em bits, bytes e vetores SIMD sem dependências externas.
//!
//! ## Filosofia
//! - Stack-allocated apenas
//! - Constant-time operations por padrão
//! - SIMD manual (AVX2/AVX-512/NEON)
//! - Zero heap allocations
//!
//! ## Uso rápido
//! ```rust
//! use avila_nucleus::bits::*;
//!
//! let (sum, carry) = adc(0xFFFFFFFFFFFFFFFF, 1, 0);
//! assert_eq!(sum, 0);
//! assert_eq!(carry, 1);
//! ```

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(incomplete_features)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

/// Versão do núcleo exposta em tempo de compilação.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Verificação placeholder para garantir builds constant-time.
#[inline(always)]
pub const fn assert_ct() {}

#[path = "bits_module.rs"]
pub mod bits;
#[path = "simd_module.rs"]
pub mod simd;
