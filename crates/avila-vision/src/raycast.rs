//! Raycasting para seleção de objetos 3D

use alloc::vec::Vec;
use avila_bim::IfcGeometry;

/// Ray 3D (origem + direção)
#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: [f32; 3],
    pub direction: [f32; 3],
}

/// Resultado de interseção ray-triângulo
#[derive(Debug, Clone)]
pub struct RayHit {
    /// Índice da geometria atingida
    pub geometry_index: usize,
    /// Índice do triângulo atingido
    pub triangle_index: usize,
    /// Distância do ray origin
    pub distance: f32,
    /// Ponto de interseção
    pub point: [f32; 3],
}

impl Ray {
    /// Cria ray da câmera através de um ponto na tela
    ///
    /// # Arguments
    /// * `screen_x` - Coordenada X normalizada (-1 a 1)
    /// * `screen_y` - Coordenada Y normalizada (-1 a 1)
    /// * `view_matrix` - Matriz de view da câmera
    /// * `proj_matrix` - Matriz de projeção
    pub fn from_screen(
        screen_x: f32,
        screen_y: f32,
        view_matrix: &[f32; 16],
        proj_matrix: &[f32; 16],
    ) -> Self {
        // Inverte matrizes para obter ray no world space
        let inv_view = invert_matrix(view_matrix);
        let inv_proj = invert_matrix(proj_matrix);

        // Ponto no near plane (NDC space)
        let near_ndc = [screen_x, screen_y, -1.0, 1.0];

        // Transforma para view space
        let near_view = transform_point(&inv_proj, &near_ndc);

        // Transforma para world space
        let near_world = transform_point(&inv_view, &near_view);

        // Origem é a posição da câmera (última coluna da view matrix invertida)
        let origin = [inv_view[12], inv_view[13], inv_view[14]];

        // Direção é do origin para o ponto near
        let mut direction = [
            near_world[0] - origin[0],
            near_world[1] - origin[1],
            near_world[2] - origin[2],
        ];

        // Normaliza direção
        let len = (direction[0] * direction[0] +
                   direction[1] * direction[1] +
                   direction[2] * direction[2]).sqrt();
        if len > 0.0 {
            direction[0] /= len;
            direction[1] /= len;
            direction[2] /= len;
        }

        Self { origin, direction }
    }

    /// Testa interseção com lista de geometrias
    pub fn intersect_geometries(&self, geometries: &[IfcGeometry]) -> Option<RayHit> {
        let mut closest_hit: Option<RayHit> = None;
        let mut closest_distance = f32::MAX;

        for (geom_idx, geom) in geometries.iter().enumerate() {
            // Testa cada triângulo da geometria
            for tri_idx in (0..geom.indices.len()).step_by(3) {
                let i0 = geom.indices[tri_idx] as usize;
                let i1 = geom.indices[tri_idx + 1] as usize;
                let i2 = geom.indices[tri_idx + 2] as usize;

                if i0 >= geom.vertices.len() || i1 >= geom.vertices.len() || i2 >= geom.vertices.len() {
                    continue;
                }

                let v0 = geom.vertices[i0];
                let v1 = geom.vertices[i1];
                let v2 = geom.vertices[i2];

                if let Some(distance) = self.intersect_triangle(&v0, &v1, &v2) {
                    if distance < closest_distance {
                        closest_distance = distance;
                        let point = [
                            self.origin[0] + self.direction[0] * distance,
                            self.origin[1] + self.direction[1] * distance,
                            self.origin[2] + self.direction[2] * distance,
                        ];
                        closest_hit = Some(RayHit {
                            geometry_index: geom_idx,
                            triangle_index: tri_idx / 3,
                            distance,
                            point,
                        });
                    }
                }
            }
        }

        closest_hit
    }

    /// Algoritmo Möller-Trumbore para interseção ray-triângulo
    fn intersect_triangle(&self, v0: &[f32; 3], v1: &[f32; 3], v2: &[f32; 3]) -> Option<f32> {
        const EPSILON: f32 = 0.000001;

        // Arestas do triângulo
        let edge1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
        let edge2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

        // Produto cruzado: direction × edge2
        let h = cross(&self.direction, &edge2);
        let a = dot(&edge1, &h);

        // Ray paralelo ao triângulo
        if a > -EPSILON && a < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = [
            self.origin[0] - v0[0],
            self.origin[1] - v0[1],
            self.origin[2] - v0[2],
        ];
        let u = f * dot(&s, &h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = cross(&s, &edge1);
        let v = f * dot(&self.direction, &q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // Calcula t (distância ao longo do ray)
        let t = f * dot(&edge2, &q);

        if t > EPSILON {
            Some(t)
        } else {
            None
        }
    }
}

/// Produto vetorial
fn cross(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Produto escalar
fn dot(a: &[f32; 3], b: &[f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Transforma ponto 4D por matriz 4x4
fn transform_point(matrix: &[f32; 16], point: &[f32; 4]) -> [f32; 4] {
    [
        matrix[0] * point[0] + matrix[4] * point[1] + matrix[8] * point[2] + matrix[12] * point[3],
        matrix[1] * point[0] + matrix[5] * point[1] + matrix[9] * point[2] + matrix[13] * point[3],
        matrix[2] * point[0] + matrix[6] * point[1] + matrix[10] * point[2] + matrix[14] * point[3],
        matrix[3] * point[0] + matrix[7] * point[1] + matrix[11] * point[2] + matrix[15] * point[3],
    ]
}

/// Inverte matriz 4x4 (simplificado para transform matrices)
fn invert_matrix(m: &[f32; 16]) -> [f32; 16] {
    // Para matrizes de transformação ortogonais, a inversa é a transposta da rotação
    // e negação da translação
    let mut inv = [0.0; 16];

    // Transpõe a parte 3x3 de rotação
    inv[0] = m[0]; inv[4] = m[1]; inv[8] = m[2];
    inv[1] = m[4]; inv[5] = m[5]; inv[9] = m[6];
    inv[2] = m[8]; inv[6] = m[9]; inv[10] = m[10];

    // Inverte translação
    inv[12] = -(m[12] * inv[0] + m[13] * inv[4] + m[14] * inv[8]);
    inv[13] = -(m[12] * inv[1] + m[13] * inv[5] + m[14] * inv[9]);
    inv[14] = -(m[12] * inv[2] + m[13] * inv[6] + m[14] * inv[10]);

    inv[3] = 0.0; inv[7] = 0.0; inv[11] = 0.0; inv[15] = 1.0;

    inv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_triangle_intersection() {
        let ray = Ray {
            origin: [0.0, 0.0, -5.0],
            direction: [0.0, 0.0, 1.0],
        };

        let v0 = [-1.0, -1.0, 0.0];
        let v1 = [1.0, -1.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];

        let result = ray.intersect_triangle(&v0, &v1, &v2);
        assert!(result.is_some());
        assert!((result.unwrap() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_ray_miss() {
        let ray = Ray {
            origin: [10.0, 10.0, -5.0],
            direction: [0.0, 0.0, 1.0],
        };

        let v0 = [-1.0, -1.0, 0.0];
        let v1 = [1.0, -1.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];

        let result = ray.intersect_triangle(&v0, &v1, &v2);
        assert!(result.is_none());
    }
}
