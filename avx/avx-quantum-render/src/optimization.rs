//! Optimizations for GPU and parallel CPU rendering

/// Parallel path tracing optimization
#[derive(Debug, Clone, Copy)]
pub struct ParallelConfig {
    /// Number of worker threads
    pub num_threads: usize,
    /// Chunks per thread
    pub chunk_size: usize,
    /// Use SIMD optimizations
    pub use_simd: bool,
}

impl ParallelConfig {
    /// Auto-detect optimal configuration
    pub fn auto() -> Self {
        // Reasonable defaults for most systems
        let num_threads = std::thread::available_parallelism()
            .map(|np| np.get())
            .unwrap_or(4);
        Self {
            num_threads,
            chunk_size: 256,
            use_simd: Self::detect_simd(),
        }
    }

    /// Maximum parallelism
    pub fn aggressive() -> Self {
        let num_threads = std::thread::available_parallelism()
            .map(|np| np.get())
            .unwrap_or(4);
        Self {
            num_threads,
            chunk_size: 64,
            use_simd: true,
        }
    }

    fn detect_simd() -> bool {
        // Check for SIMD support
        #[cfg(target_arch = "x86_64")]
        {
            is_x86_feature_detected!("avx2")
        }
        #[cfg(target_arch = "aarch64")]
        {
            is_aarch64_feature_detected!("neon")
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            false
        }
    }
}

/// Cache-friendly tiling for GPU
#[derive(Debug, Clone, Copy)]
pub struct TileConfig {
    /// Tile width
    pub tile_w: usize,
    /// Tile height
    pub tile_h: usize,
    /// Tiles per row
    pub tiles_x: usize,
}

impl TileConfig {
    /// Optimal for 1024x768 resolution
    pub fn standard() -> Self {
        Self {
            tile_w: 32,
            tile_h: 32,
            tiles_x: 32,
        }
    }

    /// Larger tiles for high bandwidth GPUs
    pub fn large() -> Self {
        Self {
            tile_w: 64,
            tile_h: 64,
            tiles_x: 16,
        }
    }

    /// Small tiles for memory-constrained GPUs
    pub fn small() -> Self {
        Self {
            tile_w: 16,
            tile_h: 16,
            tiles_x: 64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_config() {
        let config = ParallelConfig::auto();
        assert!(config.num_threads > 0);
    }

    #[test]
    fn test_tile_config() {
        let tile = TileConfig::standard();
        assert_eq!(tile.tile_w, 32);
    }
}
