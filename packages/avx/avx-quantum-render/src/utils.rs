//! Utility functions and helpers

use std::f64::consts::PI;

/// Vector operations
pub struct Vec3;

impl Vec3 {
    pub fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
    }

    pub fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }

    pub fn scale(v: [f64; 3], s: f64) -> [f64; 3] {
        [v[0] * s, v[1] * s, v[2] * s]
    }

    pub fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    pub fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }

    pub fn length(v: [f64; 3]) -> f64 {
        Self::dot(v, v).sqrt()
    }

    pub fn normalize(v: [f64; 3]) -> [f64; 3] {
        let len = Self::length(v);
        if len > 0.0 {
            Self::scale(v, 1.0 / len)
        } else {
            v
        }
    }

    pub fn length_sq(v: [f64; 3]) -> f64 {
        Self::dot(v, v)
    }

    pub fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
        Self::length(Self::sub(a, b))
    }

    pub fn reflect(v: [f64; 3], n: [f64; 3]) -> [f64; 3] {
        Self::sub(v, Self::scale(n, 2.0 * Self::dot(v, n)))
    }

    pub fn refract(v: [f64; 3], n: [f64; 3], eta: f64) -> Option<[f64; 3]> {
        let cos_i = Self::dot(v, n);
        let sin2_t = eta * eta * (1.0 - cos_i * cos_i);

        if sin2_t > 1.0 {
            return None; // Total internal reflection
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        Some(Self::add(
            Self::scale(v, eta),
            Self::scale(n, eta * cos_i - cos_t),
        ))
    }

    pub fn lerp(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
        Self::add(Self::scale(a, 1.0 - t), Self::scale(b, t))
    }
}

/// Random number utilities
pub struct Random;

impl Random {
    /// Random float in [0, 1)
    pub fn uniform() -> f64 {
        rand::random::<f64>()
    }

    /// Random float in [min, max)
    pub fn range(min: f64, max: f64) -> f64 {
        min + Self::uniform() * (max - min)
    }

    /// Random point in unit sphere
    pub fn in_sphere() -> [f64; 3] {
        let theta = 2.0 * PI * Self::uniform();
        let phi = (2.0 * Self::uniform() - 1.0).acos();
        let r = Self::uniform().cbrt();

        [
            r * phi.sin() * theta.cos(),
            r * phi.sin() * theta.sin(),
            r * phi.cos(),
        ]
    }

    /// Random point in unit disk
    pub fn in_disk() -> [f64; 2] {
        let r = Self::uniform().sqrt();
        let theta = 2.0 * PI * Self::uniform();
        [r * theta.cos(), r * theta.sin()]
    }

    /// Random point on unit sphere surface
    pub fn on_sphere() -> [f64; 3] {
        let theta = 2.0 * PI * Self::uniform();
        let phi = (2.0 * Self::uniform() - 1.0).acos();

        [
            phi.sin() * theta.cos(),
            phi.sin() * theta.sin(),
            phi.cos(),
        ]
    }

    /// Cosine-weighted hemisphere sample
    pub fn cosine_hemisphere(normal: [f64; 3]) -> [f64; 3] {
        let u = Self::uniform();
        let v = Self::uniform();

        let r = u.sqrt();
        let theta = 2.0 * PI * v;

        let x = r * theta.cos();
        let y = r * theta.sin();
        let z = (1.0 - u).sqrt();

        // Build orthonormal basis
        let right = if normal[2].abs() < 0.9 {
            Vec3::normalize(Vec3::cross(normal, [0.0, 0.0, 1.0]))
        } else {
            Vec3::normalize(Vec3::cross(normal, [1.0, 0.0, 0.0]))
        };

        let up = Vec3::cross(right, normal);

        Vec3::add(
            Vec3::add(
                Vec3::scale(right, x),
                Vec3::scale(up, y),
            ),
            Vec3::scale(normal, z),
        )
    }
}

/// Numerical utilities
pub struct Math;

impl Math {
    pub const PI: f64 = PI;
    pub const TAU: f64 = 2.0 * PI;
    pub const EPSILON: f64 = 1e-6;

    /// Clamp value to [min, max]
    pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
        x.max(min).min(max)
    }

    /// Linear interpolation
    pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
        a * (1.0 - t) + b * t
    }

    /// Smooth step function
    pub fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
        let t = Self::clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    /// Inverse square root (fast approximation)
    pub fn inv_sqrt(x: f64) -> f64 {
        if x > 0.0 {
            1.0 / x.sqrt()
        } else {
            0.0
        }
    }

    /// Degrees to radians
    pub fn deg_to_rad(deg: f64) -> f64 {
        deg * PI / 180.0
    }

    /// Radians to degrees
    pub fn rad_to_deg(rad: f64) -> f64 {
        rad * 180.0 / PI
    }

    /// Power that preserves sign
    pub fn sign_pow(x: f64, p: f64) -> f64 {
        if x >= 0.0 {
            x.powf(p)
        } else {
            -(-x).powf(p)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_add() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        let result = Vec3::add(a, b);
        assert_eq!(result, [5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_vec3_normalize() {
        let v = [3.0, 4.0, 0.0];
        let n = Vec3::normalize(v);
        let len = Vec3::length(n);
        assert!((len - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec3_reflect() {
        let v = [1.0, -1.0, 0.0];
        let n = [0.0, 1.0, 0.0];
        let r = Vec3::reflect(v, n);
        assert!((r[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_math_clamp() {
        assert_eq!(Math::clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(Math::clamp(-5.0, 0.0, 10.0), 0.0);
        assert_eq!(Math::clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn test_math_deg_to_rad() {
        let rad = Math::deg_to_rad(180.0);
        assert!((rad - PI).abs() < 1e-10);
    }

    #[test]
    fn test_random_uniform() {
        let r = Random::uniform();
        assert!(r >= 0.0 && r < 1.0);
    }

    #[test]
    fn test_random_in_sphere() {
        let p = Random::in_sphere();
        let len = Vec3::length(p);
        assert!(len <= 1.0);
    }
}
