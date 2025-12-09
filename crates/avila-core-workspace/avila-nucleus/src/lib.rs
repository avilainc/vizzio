//! # Ávila Nucleus
//!
//! Núcleo atômico - operações de baixíssimo nível em bits e bytes.
//! ZERO dependencies externas. Apenas Rust puro e instruções SIMD.
//!
//! ## Filosofia
//! - Stack-allocated apenas
//! - Constant-time operations
//! - SIMD manual (AVX2/AVX-512/NEON)
//! - Zero heap allocations

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(missing_docs, unused_imports, unused_variables, dead_code)]

pub mod bits;
pub mod simd;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
