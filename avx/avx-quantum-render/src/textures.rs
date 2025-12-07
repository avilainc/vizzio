//! Texture sampling and UV mapping

use std::f64::consts::PI;

/// Texture coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TexCoord {
    pub u: f64,
    pub v: f64,
}

impl TexCoord {
    /// Create new texture coordinates
    pub fn new(u: f64, v: f64) -> Self {
        Self { u, v }
    }

    /// Tile coordinates [0, scale)
    pub fn tile(self, scale: u32) -> Self {
        let s = scale as f64;
        Self {
            u: (self.u * s) % 1.0,
            v: (self.v * s) % 1.0,
        }
    }

    /// Repeat wrapping
    pub fn repeat(self) -> Self {
        Self {
            u: self.u.fract(),
            v: self.v.fract(),
        }
    }

    /// Clamp wrapping [0, 1]
    pub fn clamp(self) -> Self {
        Self {
            u: self.u.max(0.0).min(1.0),
            v: self.v.max(0.0).min(1.0),
        }
    }
}

/// Basic texture represented as a function
pub trait Texture: Send + Sync {
    /// Sample texture at coordinates
    fn sample(&self, uv: TexCoord) -> [f64; 4]; // RGBA
}

/// Procedural checkerboard texture
pub struct CheckerTexture {
    scale: f64,
    color1: [f64; 4],
    color2: [f64; 4],
}

impl CheckerTexture {
    pub fn new(scale: f64, color1: [f64; 4], color2: [f64; 4]) -> Self {
        Self { scale, color1, color2 }
    }
}

impl Texture for CheckerTexture {
    fn sample(&self, uv: TexCoord) -> [f64; 4] {
        let x = (uv.u * self.scale).floor() as u32;
        let y = (uv.v * self.scale).floor() as u32;
        if (x + y) % 2 == 0 {
            self.color1
        } else {
            self.color2
        }
    }
}

/// Perlin noise texture
pub struct NoiseTexture {
    scale: f64,
    octaves: u32,
}

impl NoiseTexture {
    pub fn new(scale: f64, octaves: u32) -> Self {
        Self { scale, octaves }
    }

    /// Simple Perlin-like noise (simplified for demonstration)
    fn noise(&self, x: f64, y: f64) -> f64 {
        let xi = x.floor() as i32;
        let yi = y.floor() as i32;
        let xf = x.fract();
        let yf = y.fract();

        // Pseudo-random hash
        let hash = |a: i32, b: i32| {
            let h = (a.wrapping_mul(73856093) ^ b.wrapping_mul(19349663)) as f64;
            (h.sin() * 43758.5453).fract()
        };

        let n00 = hash(xi, yi);
        let n10 = hash(xi + 1, yi);
        let n01 = hash(xi, yi + 1);
        let n11 = hash(xi + 1, yi + 1);

        // Smoothstep interpolation
        let u = xf * xf * (3.0 - 2.0 * xf);
        let v = yf * yf * (3.0 - 2.0 * yf);

        let nx0 = n00 * (1.0 - u) + n10 * u;
        let nx1 = n01 * (1.0 - u) + n11 * u;
        nx0 * (1.0 - v) + nx1 * v
    }
}

impl Texture for NoiseTexture {
    fn sample(&self, uv: TexCoord) -> [f64; 4] {
        let mut value = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;
        let mut max_value = 0.0;

        for _ in 0..self.octaves {
            value += amplitude * self.noise(
                uv.u * self.scale * frequency,
                uv.v * self.scale * frequency,
            );
            max_value += amplitude;
            amplitude *= 0.5;
            frequency *= 2.0;
        }

        let normalized = (value / max_value + 1.0) / 2.0;
        [normalized, normalized, normalized, 1.0]
    }
}

/// Solid color texture
pub struct SolidTexture {
    color: [f64; 4],
}

impl SolidTexture {
    pub fn new(color: [f64; 4]) -> Self {
        Self { color }
    }
}

impl Texture for SolidTexture {
    fn sample(&self, _uv: TexCoord) -> [f64; 4] {
        self.color
    }
}

/// UV mapping for spheres (equirectangular)
pub struct SphericalUV;

impl SphericalUV {
    /// Convert 3D point on unit sphere to UV coordinates
    pub fn from_point(x: f64, y: f64, z: f64) -> TexCoord {
        let theta = z.atan2(x);
        let phi = y.asin();

        let u = (theta + PI) / (2.0 * PI);
        let v = (phi + PI / 2.0) / PI;

        TexCoord::new(u, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tex_coord_creation() {
        let uv = TexCoord::new(0.5, 0.75);
        assert_eq!(uv.u, 0.5);
        assert_eq!(uv.v, 0.75);
    }

    #[test]
    fn test_checker_texture() {
        let checker = CheckerTexture::new(2.0, [1.0, 1.0, 1.0, 1.0], [0.0, 0.0, 0.0, 1.0]);
        let color = checker.sample(TexCoord::new(0.25, 0.25));
        assert_eq!(color[3], 1.0); // alpha channel
    }

    #[test]
    fn test_solid_texture() {
        let solid = SolidTexture::new([0.5, 0.5, 0.5, 1.0]);
        let color1 = solid.sample(TexCoord::new(0.0, 0.0));
        let color2 = solid.sample(TexCoord::new(1.0, 1.0));
        assert_eq!(color1, color2);
    }

    #[test]
    fn test_spherical_uv() {
        let uv = SphericalUV::from_point(1.0, 0.0, 0.0);
        assert!(uv.u >= 0.0 && uv.u <= 1.0);
        assert!(uv.v >= 0.0 && uv.v <= 1.0);
    }
}
