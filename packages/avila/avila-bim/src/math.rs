//! Vector and Matrix math utilities (Rust puro)

/// Vector 3D operations
pub struct Vec3;

impl Vec3 {
    #[inline]
    pub fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    #[inline]
    pub fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }

    #[inline]
    pub fn length(v: [f64; 3]) -> f64 {
        (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
    }

    #[inline]
    pub fn normalize(v: [f64; 3]) -> [f64; 3] {
        let len = Self::length(v);
        if len < 1e-10 {
            return [0.0, 0.0, 1.0];
        }
        [v[0] / len, v[1] / len, v[2] / len]
    }

    #[inline]
    pub fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
    }

    #[inline]
    pub fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }

    #[inline]
    pub fn scale(v: [f64; 3], s: f64) -> [f64; 3] {
        [v[0] * s, v[1] * s, v[2] * s]
    }

    #[inline]
    pub fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
        Self::length(Self::sub(a, b))
    }
}

/// Matrix 4x4 operations (column-major)
pub struct Mat4;

impl Mat4 {
    #[inline]
    pub fn identity() -> [f64; 16] {
        [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]
    }

    #[inline]
    pub fn translation(x: f64, y: f64, z: f64) -> [f64; 16] {
        [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            x, y, z, 1.0,
        ]
    }

    #[inline]
    pub fn scale(x: f64, y: f64, z: f64) -> [f64; 16] {
        [
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]
    }

    /// Multiplicar duas matrizes 4x4
    pub fn multiply(a: &[f64; 16], b: &[f64; 16]) -> [f64; 16] {
        let mut result = [0.0; 16];

        for row in 0..4 {
            for col in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += a[row + k * 4] * b[k + col * 4];
                }
                result[row + col * 4] = sum;
            }
        }

        result
    }

    /// Transformar ponto 3D por matriz
    pub fn transform_point(matrix: &[f64; 16], point: [f64; 3]) -> [f64; 3] {
        let x = matrix[0] * point[0] + matrix[4] * point[1] + matrix[8] * point[2] + matrix[12];
        let y = matrix[1] * point[0] + matrix[5] * point[1] + matrix[9] * point[2] + matrix[13];
        let z = matrix[2] * point[0] + matrix[6] * point[1] + matrix[10] * point[2] + matrix[14];
        let w = matrix[3] * point[0] + matrix[7] * point[1] + matrix[11] * point[2] + matrix[15];

        if w.abs() > 1e-10 {
            [x / w, y / w, z / w]
        } else {
            [x, y, z]
        }
    }

    /// Extrair translação da matriz
    pub fn extract_translation(matrix: &[f64; 16]) -> [f64; 3] {
        [matrix[12], matrix[13], matrix[14]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_operations() {
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];

        assert_eq!(Vec3::dot(a, b), 0.0);

        let cross = Vec3::cross(a, b);
        assert!((cross[0] - 0.0).abs() < 1e-10);
        assert!((cross[1] - 0.0).abs() < 1e-10);
        assert!((cross[2] - 1.0).abs() < 1e-10);

        assert!((Vec3::length([3.0, 4.0, 0.0]) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_mat4_operations() {
        let identity = Mat4::identity();
        let point = [1.0, 2.0, 3.0];

        let transformed = Mat4::transform_point(&identity, point);
        assert_eq!(transformed, point);

        let translation = Mat4::translation(5.0, 10.0, 15.0);
        let moved = Mat4::transform_point(&translation, point);
        assert_eq!(moved, [6.0, 12.0, 18.0]);
    }
}
