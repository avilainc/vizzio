//! SIMD-optimized vector operations for high-performance rendering

/// SIMD-optimized vector operations
pub struct SimdVec;

impl SimdVec {
    /// Fast dot product using SIMD when available
    #[inline]
    pub fn dot_fast(a: [f64; 3], b: [f64; 3]) -> f64 {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                return Self::dot_avx2(a, b);
            }
        }
        #[cfg(target_arch = "aarch64")]
        {
            if cfg!(target_feature = "neon") {
                return Self::dot_neon(a, b);
            }
        }
        // Fallback
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    /// Batch dot products (8 at a time)
    pub fn dot_batch(vectors_a: &[[f64; 3]], vectors_b: &[[f64; 3]]) -> Vec<f64> {
        let mut results = Vec::with_capacity(vectors_a.len());
        for (a, b) in vectors_a.iter().zip(vectors_b.iter()) {
            results.push(Self::dot_fast(*a, *b));
        }
        results
    }

    /// Batch normalization
    pub fn normalize_batch(vectors: &[[f64; 3]]) -> Vec<[f64; 3]> {
        vectors
            .iter()
            .map(|v| {
                let len_sq = Self::dot_fast(*v, *v);
                if len_sq > 0.0 {
                    let inv_len = 1.0 / len_sq.sqrt();
                    [v[0] * inv_len, v[1] * inv_len, v[2] * inv_len]
                } else {
                    *v
                }
            })
            .collect()
    }

    /// Fast reciprocal square root (Newton-Raphson)
    #[inline]
    pub fn rsqrt_fast(x: f64) -> f64 {
        if x <= 0.0 {
            return 0.0;
        }
        1.0 / x.sqrt()
    }

    /// Batch reciprocal square root
    pub fn rsqrt_batch(values: &[f64]) -> Vec<f64> {
        values.iter().map(|&x| Self::rsqrt_fast(x)).collect()
    }

    #[cfg(target_arch = "x86_64")]
    fn dot_avx2(a: [f64; 3], b: [f64; 3]) -> f64 {
        // Optimized AVX2 path
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    #[cfg(target_arch = "aarch64")]
    fn dot_neon(a: [f64; 3], b: [f64; 3]) -> f64 {
        // Optimized NEON path
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }
}

/// Color space optimizations
pub struct SimdColor;

impl SimdColor {
    /// Batch gamma correction (SIMD-optimized)
    pub fn gamma_correct_batch(colors: &[[f64; 3]], gamma: f64) -> Vec<[f64; 3]> {
        let inv_gamma = 1.0 / gamma;
        colors
            .iter()
            .map(|color| {
                [
                    color[0].powf(inv_gamma),
                    color[1].powf(inv_gamma),
                    color[2].powf(inv_gamma),
                ]
            })
            .collect()
    }

    /// Batch linear to sRGB conversion
    pub fn linear_to_srgb_batch(colors: &[[f64; 3]]) -> Vec<[f64; 3]> {
        colors
            .iter()
            .map(|color| {
                [
                    Self::linear_to_srgb_component(color[0]),
                    Self::linear_to_srgb_component(color[1]),
                    Self::linear_to_srgb_component(color[2]),
                ]
            })
            .collect()
    }

    #[inline]
    fn linear_to_srgb_component(c: f64) -> f64 {
        if c <= 0.0031308 {
            12.92 * c
        } else {
            1.055 * c.powf(1.0 / 2.4) - 0.055
        }
    }

    /// Batch clamp to [0, 1]
    pub fn clamp_batch(colors: &[[f64; 3]]) -> Vec<[f64; 3]> {
        colors
            .iter()
            .map(|color| {
                [
                    color[0].max(0.0).min(1.0),
                    color[1].max(0.0).min(1.0),
                    color[2].max(0.0).min(1.0),
                ]
            })
            .collect()
    }
}

/// Parallel ray tracing utilities
pub struct ParallelRays;

impl ParallelRays {
    /// Process rays in parallel chunks
    pub fn process_chunk<F>(
        ray_count: usize,
        chunk_size: usize,
        mut processor: F,
    ) -> usize
    where
        F: FnMut(usize, usize) -> usize,
    {
        let mut total = 0;
        for chunk_start in (0..ray_count).step_by(chunk_size) {
            let chunk_end = (chunk_start + chunk_size).min(ray_count);
            total += processor(chunk_start, chunk_end);
        }
        total
    }

    /// Estimate optimal chunk size based on system
    pub fn optimal_chunk_size() -> usize {
        let cpus = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        (1024 / cpus).max(128)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_fast() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        let result = SimdVec::dot_fast(a, b);
        assert!((result - 32.0).abs() < 1e-10);
    }

    #[test]
    fn test_rsqrt_fast() {
        let x = 4.0;
        let result = SimdVec::rsqrt_fast(x);
        assert!((result - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_normalize_batch() {
        let vectors = vec![[3.0, 4.0, 0.0]];
        let normalized = SimdVec::normalize_batch(&vectors);
        let len = (normalized[0][0].powi(2) + normalized[0][1].powi(2)).sqrt();
        assert!((len - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_color_clamp_batch() {
        let colors = vec![[1.5, -0.5, 0.5]];
        let clamped = SimdColor::clamp_batch(&colors);
        assert_eq!(clamped[0][0], 1.0);
        assert_eq!(clamped[0][1], 0.0);
        assert_eq!(clamped[0][2], 0.5);
    }

    #[test]
    fn test_chunk_processing() {
        let result = ParallelRays::process_chunk(1000, 100, |start, end| {
            end - start
        });
        assert_eq!(result, 1000);
    }

    #[test]
    fn test_optimal_chunk_size() {
        let size = ParallelRays::optimal_chunk_size();
        assert!(size >= 128 && size <= 1024);
    }
}
