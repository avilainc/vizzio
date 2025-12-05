//! SIMD-accelerated implementations
//!
//! Hardware-accelerated encoding/decoding using SIMD instructions.

#[cfg(target_arch = "x86_64")]
pub mod avx2;

#[cfg(target_arch = "aarch64")]
pub mod neon;

// Feature detection
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
/// Checks if AVX2 is available on the current CPU
pub fn has_avx2() -> bool {
    #[cfg(target_feature = "avx2")]
    {
        true
    }
    #[cfg(not(target_feature = "avx2"))]
    {
        is_x86_feature_detected!("avx2")
    }
}

#[cfg(target_arch = "aarch64")]
/// Checks if NEON is available on the current CPU
pub fn has_neon() -> bool {
    #[cfg(target_feature = "neon")]
    {
        true
    }
    #[cfg(not(target_feature = "neon"))]
    {
        std::arch::is_aarch64_feature_detected!("neon")
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
/// Checks if SIMD is available (always false on unsupported architectures)
pub fn has_simd() -> bool {
    false
}
