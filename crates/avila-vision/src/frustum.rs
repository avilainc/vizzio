//! Frustum culling para otimização de renderização

use alloc::vec::Vec;

/// Plano do frustum (ax + by + cz + d = 0)
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    /// Coeficiente A
    pub a: f32,
    /// Coeficiente B
    pub b: f32,
    /// Coeficiente C
    pub c: f32,
    /// Coeficiente D
    pub d: f32,
}

impl Plane {
    /// Cria plano a partir de coeficientes
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        let len = (a * a + b * b + c * c).sqrt();
        Self {
            a: a / len,
            b: b / len,
            c: c / len,
            d: d / len,
        }
    }

    /// Distância de ponto ao plano
    pub fn distance(&self, point: [f32; 3]) -> f32 {
        self.a * point[0] + self.b * point[1] + self.c * point[2] + self.d
    }
}

/// Bounding box axis-aligned
#[derive(Debug, Clone, Copy)]
pub struct AABB {
    /// Mínimo (x, y, z)
    pub min: [f32; 3],
    /// Máximo (x, y, z)
    pub max: [f32; 3],
}

impl AABB {
    /// Cria AABB a partir de vértices
    pub fn from_vertices(vertices: &[[f32; 3]]) -> Self {
        if vertices.is_empty() {
            return Self {
                min: [0.0, 0.0, 0.0],
                max: [0.0, 0.0, 0.0],
            };
        }

        let mut min = vertices[0];
        let mut max = vertices[0];

        for v in vertices.iter().skip(1) {
            min[0] = min[0].min(v[0]);
            min[1] = min[1].min(v[1]);
            min[2] = min[2].min(v[2]);
            max[0] = max[0].max(v[0]);
            max[1] = max[1].max(v[1]);
            max[2] = max[2].max(v[2]);
        }

        Self { min, max }
    }

    /// Centro do AABB
    pub fn center(&self) -> [f32; 3] {
        [
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
            (self.min[2] + self.max[2]) * 0.5,
        ]
    }

    /// Raio (semi-diagonal)
    pub fn radius(&self) -> f32 {
        let dx = self.max[0] - self.min[0];
        let dy = self.max[1] - self.min[1];
        let dz = self.max[2] - self.min[2];
        ((dx * dx + dy * dy + dz * dz) * 0.25).sqrt()
    }
}

/// Frustum de visão (6 planos)
pub struct Frustum {
    /// Planos: left, right, bottom, top, near, far
    planes: [Plane; 6],
}

impl Frustum {
    /// Extrai frustum da matriz view-projection (4x4)
    pub fn from_matrix(mvp: &[f32; 16]) -> Self {
        let mut planes = [Plane::new(0.0, 0.0, 0.0, 0.0); 6];

        // Left plane
        planes[0] = Plane::new(
            mvp[3] + mvp[0],
            mvp[7] + mvp[4],
            mvp[11] + mvp[8],
            mvp[15] + mvp[12],
        );

        // Right plane
        planes[1] = Plane::new(
            mvp[3] - mvp[0],
            mvp[7] - mvp[4],
            mvp[11] - mvp[8],
            mvp[15] - mvp[12],
        );

        // Bottom plane
        planes[2] = Plane::new(
            mvp[3] + mvp[1],
            mvp[7] + mvp[5],
            mvp[11] + mvp[9],
            mvp[15] + mvp[13],
        );

        // Top plane
        planes[3] = Plane::new(
            mvp[3] - mvp[1],
            mvp[7] - mvp[5],
            mvp[11] - mvp[9],
            mvp[15] - mvp[13],
        );

        // Near plane
        planes[4] = Plane::new(
            mvp[3] + mvp[2],
            mvp[7] + mvp[6],
            mvp[11] + mvp[10],
            mvp[15] + mvp[14],
        );

        // Far plane
        planes[5] = Plane::new(
            mvp[3] - mvp[2],
            mvp[7] - mvp[6],
            mvp[11] - mvp[10],
            mvp[15] - mvp[14],
        );

        Self { planes }
    }

    /// Testa se AABB está dentro do frustum (esfera aproximada)
    pub fn contains_aabb(&self, aabb: &AABB) -> bool {
        let center = aabb.center();
        let radius = aabb.radius();

        for plane in &self.planes {
            if plane.distance(center) < -radius {
                return false; // Fora do frustum
            }
        }

        true // Dentro ou intersectando
    }
}

/// Multiplica duas matrizes 4x4
pub fn multiply_matrices(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16] {
    let mut result = [0.0f32; 16];

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i * 4 + j] += a[i * 4 + k] * b[k * 4 + j];
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb_from_vertices() {
        let vertices = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];

        let aabb = AABB::from_vertices(&vertices);
        assert_eq!(aabb.min, [0.0, 0.0, 0.0]);
        assert_eq!(aabb.max, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_aabb_center() {
        let aabb = AABB {
            min: [0.0, 0.0, 0.0],
            max: [2.0, 2.0, 2.0],
        };
        assert_eq!(aabb.center(), [1.0, 1.0, 1.0]);
    }
}
