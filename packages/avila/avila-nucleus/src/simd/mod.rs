//! SIMD intrinsics - wrappers de baixo nível
//!
//! Abstrações mínimas sobre intrinsics SIMD para operações vetorizadas.

pub mod detect;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(all(target_arch = "x86_64", feature = "simd"))]
pub mod avx2;

#[cfg(all(target_arch = "x86_64", feature = "simd"))]
pub mod avx512;

pub use detect::*;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(all(target_arch = "x86_64", feature = "simd"))]
pub use avx2::*;

#[cfg(all(target_arch = "x86_64", feature = "simd"))]
pub use avx512::*;

/// Trait genérica para operações SIMD em vetores de `u64`.
pub trait SimdOps {
	/// Tipo concreto do vetor SIMD.
	type Vector;

	/// Carrega dados não alinhados da memória.
	unsafe fn load(ptr: *const u64) -> Self::Vector;

	/// Armazena vetor SIMD em memória.
	unsafe fn store(ptr: *mut u64, vec: Self::Vector);

	/// Soma vetorial lane-wise.
	unsafe fn add(a: Self::Vector, b: Self::Vector) -> Self::Vector;

	/// Subtração vetorial lane-wise.
	unsafe fn sub(a: Self::Vector, b: Self::Vector) -> Self::Vector;

	/// XOR bit a bit.
	unsafe fn xor(a: Self::Vector, b: Self::Vector) -> Self::Vector;

	/// AND bit a bit.
	unsafe fn and(a: Self::Vector, b: Self::Vector) -> Self::Vector;
}
