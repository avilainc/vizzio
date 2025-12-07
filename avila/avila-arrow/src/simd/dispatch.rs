//! Runtime CPU feature detection and dispatch

/// CPU features
#[derive(Debug, Clone, Copy)]
pub struct CpuFeatures {
    pub avx2: bool,
    pub avx512: bool,
    pub neon: bool,
}

impl CpuFeatures {
    /// Detect available CPU features at runtime
    pub fn detect() -> Self {
        Self {
            avx2: is_x86_feature_detected!("avx2"),
            avx512: is_x86_feature_detected!("avx512f"),
            neon: cfg!(target_arch = "aarch64"),
        }
    }
}

/// Get the best available SIMD implementation
pub fn get_simd_level() -> &'static str {
    let features = CpuFeatures::detect();

    if features.avx512 {
        "AVX-512"
    } else if features.avx2 {
        "AVX2"
    } else if features.neon {
        "NEON"
    } else {
        "Scalar"
    }
}
