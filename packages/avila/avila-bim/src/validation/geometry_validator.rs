//! Geometry validator (meshes, degenerate triangles, etc.)

use crate::bim_core::*;

/// Validador de geometria
pub struct GeometryValidator {
    epsilon: f64,
}

impl GeometryValidator {
    pub fn new() -> Self {
        Self { epsilon: 1e-6 }
    }

    /// Validar mesh
    pub fn validate_mesh(&self, mesh: &Mesh) -> Vec<String> {
        let mut errors = Vec::new();

        // Validar vértices
        if mesh.vertices.len() % 3 != 0 {
            errors.push("Vertices array length not multiple of 3".to_string());
        }

        // Validar normais
        if mesh.normals.len() != mesh.vertices.len() {
            errors.push("Normals count mismatch".to_string());
        }

        // Validar índices
        if mesh.indices.len() % 3 != 0 {
            errors.push("Indices array length not multiple of 3".to_string());
        }

        let vertex_count = mesh.vertices.len() / 3;
        for &idx in &mesh.indices {
            if idx >= vertex_count as u32 {
                errors.push(format!("Index out of bounds: {}", idx));
                break;
            }
        }

        // Detectar triângulos degenerados
        errors.extend(self.find_degenerate_triangles(mesh));

        errors
    }

    /// Detectar triângulos degenerados (área próxima de zero)
    fn find_degenerate_triangles(&self, mesh: &Mesh) -> Vec<String> {
        let mut errors = Vec::new();

        for i in (0..mesh.indices.len()).step_by(3) {
            let i0 = mesh.indices[i] as usize * 3;
            let i1 = mesh.indices[i + 1] as usize * 3;
            let i2 = mesh.indices[i + 2] as usize * 3;

            let v0 = [
                mesh.vertices[i0] as f64,
                mesh.vertices[i0 + 1] as f64,
                mesh.vertices[i0 + 2] as f64,
            ];
            let v1 = [
                mesh.vertices[i1] as f64,
                mesh.vertices[i1 + 1] as f64,
                mesh.vertices[i1 + 2] as f64,
            ];
            let v2 = [
                mesh.vertices[i2] as f64,
                mesh.vertices[i2 + 1] as f64,
                mesh.vertices[i2 + 2] as f64,
            ];

            let area = self.triangle_area(v0, v1, v2);
            if area < self.epsilon {
                errors.push(format!("Degenerate triangle at index {}", i / 3));
            }
        }

        errors
    }

    /// Calcular área de triângulo
    fn triangle_area(&self, v0: [f64; 3], v1: [f64; 3], v2: [f64; 3]) -> f64 {
        let edge1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
        let edge2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

        let cross = [
            edge1[1] * edge2[2] - edge1[2] * edge2[1],
            edge1[2] * edge2[0] - edge1[0] * edge2[2],
            edge1[0] * edge2[1] - edge1[1] * edge2[0],
        ];

        let length = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
        length / 2.0
    }
}

impl Default for GeometryValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_mesh() {
        let validator = GeometryValidator::new();

        let mesh = Mesh {
            vertices: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            normals: vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
            indices: vec![0, 1, 2],
            uvs: None,
            colors: None,
        };

        let errors = validator.validate_mesh(&mesh);
        assert!(errors.is_empty());
    }
}
