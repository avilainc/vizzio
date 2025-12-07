//! Instruções SIMD manuais para performance máxima.
//!
//! Reúne rotinas portáteis, bem como implementações otimizadas para AVX2,
//! AVX-512 e NEON quando disponíveis.

#[path = "simd/detect.rs"]
pub mod detect;

#[cfg(target_arch = "x86_64")]
#[path = "simd/x86_64.rs"]
pub mod x86_64;

#[cfg(target_arch = "aarch64")]
#[path = "simd/aarch64.rs"]
pub mod aarch64;

#[cfg(all(target_arch = "x86_64", feature = "avx2"))]
#[path = "simd/avx2.rs"]
pub mod avx2;

#[cfg(all(target_arch = "x86_64", feature = "avx512"))]
#[path = "simd/avx512.rs"]
pub mod avx512;

pub use detect::*;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(all(target_arch = "x86_64", feature = "avx2"))]
pub use avx2::*;

#[cfg(all(target_arch = "x86_64", feature = "avx512"))]
pub use avx512::*;

/// Trait para operações SIMD genéricas.
pub trait SimdOps {
    /// Tipo concreto do vetor SIMD.
    type Vector;

    /// Carrega dados na memória para vetor SIMD.
    unsafe fn load(ptr: *const u64) -> Self::Vector;

    /// Armazena vetor SIMD na memória.
    unsafe fn store(ptr: *mut u64, vec: Self::Vector);

    /// Adição vetorial lane-wise.
    unsafe fn add(a: Self::Vector, b: Self::Vector) -> Self::Vector;

    /// Subtração vetorial lane-wise.
    unsafe fn sub(a: Self::Vector, b: Self::Vector) -> Self::Vector;

    /// XOR vetorial.
    unsafe fn xor(a: Self::Vector, b: Self::Vector) -> Self::Vector;

    /// AND vetorial.
    unsafe fn and(a: Self::Vector, b: Self::Vector) -> Self::Vector;
}
