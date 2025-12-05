//! SIMD traits for generic operations

/// Trait for SIMD-capable types
pub trait SimdType: Copy {
    type Scalar: Copy;

    fn lanes() -> usize;
    fn splat(value: Self::Scalar) -> Self;
    fn add(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
}

/// Trait for SIMD operations on arrays
pub trait SimdOps<T> {
    fn simd_sum(&self) -> T;
    fn simd_add(&self, other: &Self) -> Vec<T>;
    fn simd_mul(&self, other: &Self) -> Vec<T>;
}
