//! Post-processing effects and tone mapping

/// Tone mapping operators
pub struct ToneMapper;

impl ToneMapper {
    /// Reinhard tone mapping
    pub fn reinhard(color: [f64; 3], exposure: f64) -> [f64; 3] {
        let adjusted = [
            color[0] * exposure,
            color[1] * exposure,
            color[2] * exposure,
        ];

        [
            adjusted[0] / (1.0 + adjusted[0]),
            adjusted[1] / (1.0 + adjusted[1]),
            adjusted[2] / (1.0 + adjusted[2]),
        ]
    }

    /// ACES tone mapping (filmic)
    pub fn aces(color: [f64; 3]) -> [f64; 3] {
        let a = 2.51;
        let b = 0.03;
        let c = 2.43;
        let d = 0.59;
        let e = 0.14;

        let map = |x: f64| {
            (x * (a * x + b)) / (x * (c * x + d) + e)
        };

        [map(color[0]), map(color[1]), map(color[2])]
    }

    /// Gamma correction
    pub fn gamma_correct(color: [f64; 3], gamma: f64) -> [f64; 3] {
        let inv_gamma = 1.0 / gamma;
        [
            color[0].powf(inv_gamma),
            color[1].powf(inv_gamma),
            color[2].powf(inv_gamma),
        ]
    }

    /// Uncharted 2 tone mapping
    pub fn uncharted2(color: [f64; 3]) -> [f64; 3] {
        let white_point: f64 = 11.2;

        let tonemap = |x: f64| -> f64 {
            let a = 0.15;
            let b = 0.50;
            let c = 0.10;
            let d = 0.20;
            let e = 0.02;
            let f = 0.30;

            ((x * (a * x + b)) / (x * (c * x + d) + e)) - (f / (1.0 + white_point).powf(2.0))
        };

        let white = tonemap(white_point);
        [
            tonemap(color[0]) / white,
            tonemap(color[1]) / white,
            tonemap(color[2]) / white,
        ]
    }
}

/// Color space conversion
pub struct ColorSpace;

impl ColorSpace {
    /// Convert sRGB to linear RGB
    pub fn srgb_to_linear(color: [f64; 3]) -> [f64; 3] {
        [
            if color[0] <= 0.04045 {
                color[0] / 12.92
            } else {
                ((color[0] + 0.055) / 1.055).powf(2.4)
            },
            if color[1] <= 0.04045 {
                color[1] / 12.92
            } else {
                ((color[1] + 0.055) / 1.055).powf(2.4)
            },
            if color[2] <= 0.04045 {
                color[2] / 12.92
            } else {
                ((color[2] + 0.055) / 1.055).powf(2.4)
            },
        ]
    }

    /// Convert linear RGB to sRGB
    pub fn linear_to_srgb(color: [f64; 3]) -> [f64; 3] {
        [
            if color[0] <= 0.0031308 {
                12.92 * color[0]
            } else {
                1.055 * color[0].powf(1.0 / 2.4) - 0.055
            },
            if color[1] <= 0.0031308 {
                12.92 * color[1]
            } else {
                1.055 * color[1].powf(1.0 / 2.4) - 0.055
            },
            if color[2] <= 0.0031308 {
                12.92 * color[2]
            } else {
                1.055 * color[2].powf(1.0 / 2.4) - 0.055
            },
        ]
    }

    /// RGB to luminance
    pub fn rgb_to_luminance(color: [f64; 3]) -> f64 {
        0.2126 * color[0] + 0.7152 * color[1] + 0.0722 * color[2]
    }
}

/// Bloom and HDR effects
pub struct BloomEffect {
    threshold: f64,
    strength: f64,
    radius: u32,
}

impl BloomEffect {
    pub fn new(threshold: f64, strength: f64, radius: u32) -> Self {
        Self {
            threshold,
            strength,
            radius,
        }
    }

    /// Extract bloom-eligible pixels
    pub fn extract_bloom(&self, color: [f64; 3]) -> [f64; 3] {
        let lum = ColorSpace::rgb_to_luminance(color);
        if lum > self.threshold {
            let bloom_factor = (lum - self.threshold) * self.strength;
            [
                color[0] * bloom_factor,
                color[1] * bloom_factor,
                color[2] * bloom_factor,
            ]
        } else {
            [0.0, 0.0, 0.0]
        }
    }
}

/// Lens distortion effect
pub struct LensDistortion {
    k1: f64, // Primary distortion
    k2: f64, // Secondary distortion
}

impl LensDistortion {
    pub fn new(k1: f64, k2: f64) -> Self {
        Self { k1, k2 }
    }

    /// Apply barrel/pincushion distortion
    pub fn distort(&self, uv: [f64; 2]) -> [f64; 2] {
        let x = uv[0] - 0.5;
        let y = uv[1] - 0.5;
        let r2 = x * x + y * y;

        let distortion = 1.0 + self.k1 * r2 + self.k2 * r2 * r2;

        [x * distortion + 0.5, y * distortion + 0.5]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reinhard_tonemapping() {
        let color = [2.0, 2.0, 2.0];
        let mapped = ToneMapper::reinhard(color, 1.0);
        assert!(mapped[0] < 1.0);
        assert!(mapped[1] < 1.0);
        assert!(mapped[2] < 1.0);
    }

    #[test]
    fn test_gamma_correction() {
        let color = [0.5, 0.5, 0.5];
        let corrected = ToneMapper::gamma_correct(color, 2.2);
        assert!(corrected[0] > 0.0 && corrected[0] < 1.0);
    }

    #[test]
    fn test_srgb_conversion() {
        let linear = [0.5, 0.5, 0.5];
        let srgb = ColorSpace::linear_to_srgb(linear);
        let back = ColorSpace::srgb_to_linear(srgb);
        assert!((back[0] - linear[0]).abs() < 1e-6);
    }

    #[test]
    fn test_bloom_extraction() {
        let bloom = BloomEffect::new(1.0, 1.0, 5);
        let bright_color = [3.0, 3.0, 3.0];
        let extracted = bloom.extract_bloom(bright_color);
        assert!(extracted[0] > 0.0);
    }

    #[test]
    fn test_lens_distortion() {
        let distortion = LensDistortion::new(0.1, 0.01);
        let uv = [0.5, 0.5];
        let distorted = distortion.distort(uv);
        assert_eq!(distorted[0], 0.5); // No distortion at center
        assert_eq!(distorted[1], 0.5);
    }
}
