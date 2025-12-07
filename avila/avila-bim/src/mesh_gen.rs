//! Mesh generation utilities (Rust puro)

use crate::bim_core::*;

/// Gerador de geometria primitiva
pub struct MeshGenerator;

impl MeshGenerator {
    /// Gerar caixa (box)
    pub fn box_mesh(width: f64, height: f64, depth: f64) -> Mesh {
        let w = (width / 2.0) as f32;
        let h = (height / 2.0) as f32;
        let d = (depth / 2.0) as f32;

        #[rustfmt::skip]
        let vertices = vec![
            // Front face
            -w, -h,  d,  w, -h,  d,  w,  h,  d, -w,  h,  d,
            // Back face
            -w, -h, -d, -w,  h, -d,  w,  h, -d,  w, -h, -d,
            // Top face
            -w,  h, -d, -w,  h,  d,  w,  h,  d,  w,  h, -d,
            // Bottom face
            -w, -h, -d,  w, -h, -d,  w, -h,  d, -w, -h,  d,
            // Right face
             w, -h, -d,  w,  h, -d,  w,  h,  d,  w, -h,  d,
            // Left face
            -w, -h, -d, -w, -h,  d, -w,  h,  d, -w,  h, -d,
        ];

        #[rustfmt::skip]
        let normals = vec![
            // Front
             0.0,  0.0,  1.0,  0.0,  0.0,  1.0,  0.0,  0.0,  1.0,  0.0,  0.0,  1.0,
            // Back
             0.0,  0.0, -1.0,  0.0,  0.0, -1.0,  0.0,  0.0, -1.0,  0.0,  0.0, -1.0,
            // Top
             0.0,  1.0,  0.0,  0.0,  1.0,  0.0,  0.0,  1.0,  0.0,  0.0,  1.0,  0.0,
            // Bottom
             0.0, -1.0,  0.0,  0.0, -1.0,  0.0,  0.0, -1.0,  0.0,  0.0, -1.0,  0.0,
            // Right
             1.0,  0.0,  0.0,  1.0,  0.0,  0.0,  1.0,  0.0,  0.0,  1.0,  0.0,  0.0,
            // Left
            -1.0,  0.0,  0.0, -1.0,  0.0,  0.0, -1.0,  0.0,  0.0, -1.0,  0.0,  0.0,
        ];

        #[rustfmt::skip]
        let indices = vec![
            0,  1,  2,  0,  2,  3,   // Front
            4,  5,  6,  4,  6,  7,   // Back
            8,  9,  10, 8,  10, 11,  // Top
            12, 13, 14, 12, 14, 15,  // Bottom
            16, 17, 18, 16, 18, 19,  // Right
            20, 21, 22, 20, 22, 23,  // Left
        ];

        Mesh {
            vertices,
            normals,
            indices,
            uvs: None,
            colors: None,
        }
    }

    /// Gerar cilindro
    pub fn cylinder_mesh(radius: f64, height: f64, segments: usize) -> Mesh {
        let r = radius as f32;
        let h = (height / 2.0) as f32;
        let seg = segments.max(3);

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();

        // Círculo inferior
        for i in 0..seg {
            let angle = (i as f32 / seg as f32) * 2.0 * std::f32::consts::PI;
            let x = r * angle.cos();
            let z = r * angle.sin();

            vertices.extend_from_slice(&[x, -h, z]);
            normals.extend_from_slice(&[x / r, 0.0, z / r]);
        }

        // Círculo superior
        for i in 0..seg {
            let angle = (i as f32 / seg as f32) * 2.0 * std::f32::consts::PI;
            let x = r * angle.cos();
            let z = r * angle.sin();

            vertices.extend_from_slice(&[x, h, z]);
            normals.extend_from_slice(&[x / r, 0.0, z / r]);
        }

        // Faces laterais
        for i in 0..seg {
            let next = (i + 1) % seg;
            let bottom_curr = i as u32;
            let bottom_next = next as u32;
            let top_curr = (seg + i) as u32;
            let top_next = (seg + next) as u32;

            indices.extend_from_slice(&[
                bottom_curr, top_curr, top_next,
                bottom_curr, top_next, bottom_next,
            ]);
        }

        Mesh {
            vertices,
            normals,
            indices,
            uvs: None,
            colors: None,
        }
    }

    /// Gerar esfera (UV sphere)
    pub fn sphere_mesh(radius: f64, lat_segments: usize, lon_segments: usize) -> Mesh {
        let r = radius as f32;
        let lat_seg = lat_segments.max(3);
        let lon_seg = lon_segments.max(3);

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();

        // Gerar vértices
        for lat in 0..=lat_seg {
            let theta = (lat as f32 / lat_seg as f32) * std::f32::consts::PI;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            for lon in 0..=lon_seg {
                let phi = (lon as f32 / lon_seg as f32) * 2.0 * std::f32::consts::PI;
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                let x = sin_theta * cos_phi;
                let y = cos_theta;
                let z = sin_theta * sin_phi;

                vertices.extend_from_slice(&[r * x, r * y, r * z]);
                normals.extend_from_slice(&[x, y, z]);
            }
        }

        // Gerar índices
        for lat in 0..lat_seg {
            for lon in 0..lon_seg {
                let first = (lat * (lon_seg + 1) + lon) as u32;
                let second = first + lon_seg as u32 + 1;

                indices.extend_from_slice(&[
                    first, second, first + 1,
                    second, second + 1, first + 1,
                ]);
            }
        }

        Mesh {
            vertices,
            normals,
            indices,
            uvs: None,
            colors: None,
        }
    }

    /// Gerar plano
    pub fn plane_mesh(width: f64, depth: f64) -> Mesh {
        let w = (width / 2.0) as f32;
        let d = (depth / 2.0) as f32;

        let vertices = vec![
            -w, 0.0, -d,
             w, 0.0, -d,
             w, 0.0,  d,
            -w, 0.0,  d,
        ];

        let normals = vec![
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,
        ];

        let indices = vec![0, 1, 2, 0, 2, 3];

        Mesh {
            vertices,
            normals,
            indices,
            uvs: None,
            colors: None,
        }
    }

    /// Calcular normais de mesh (flat shading)
    pub fn compute_flat_normals(mesh: &mut Mesh) {
        let mut normals = vec![0.0; mesh.vertices.len()];

        for i in (0..mesh.indices.len()).step_by(3) {
            let i0 = mesh.indices[i] as usize * 3;
            let i1 = mesh.indices[i + 1] as usize * 3;
            let i2 = mesh.indices[i + 2] as usize * 3;

            let v0 = [mesh.vertices[i0], mesh.vertices[i0 + 1], mesh.vertices[i0 + 2]];
            let v1 = [mesh.vertices[i1], mesh.vertices[i1 + 1], mesh.vertices[i1 + 2]];
            let v2 = [mesh.vertices[i2], mesh.vertices[i2 + 1], mesh.vertices[i2 + 2]];

            let edge1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
            let edge2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

            let normal = [
                edge1[1] * edge2[2] - edge1[2] * edge2[1],
                edge1[2] * edge2[0] - edge1[0] * edge2[2],
                edge1[0] * edge2[1] - edge1[1] * edge2[0],
            ];

            let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
            let normalized = if length > 1e-6 {
                [normal[0] / length, normal[1] / length, normal[2] / length]
            } else {
                [0.0, 1.0, 0.0]
            };

            // Atribuir normal aos três vértices do triângulo
            for &idx in &[i0, i1, i2] {
                normals[idx] = normalized[0];
                normals[idx + 1] = normalized[1];
                normals[idx + 2] = normalized[2];
            }
        }

        mesh.normals = normals;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_mesh() {
        let mesh = MeshGenerator::box_mesh(2.0, 3.0, 4.0);

        assert_eq!(mesh.vertices.len(), 72); // 24 vertices * 3
        assert_eq!(mesh.normals.len(), 72);
        assert_eq!(mesh.indices.len(), 36); // 12 triangles * 3
        assert_eq!(mesh.triangle_count(), 12);
    }

    #[test]
    fn test_cylinder_mesh() {
        let mesh = MeshGenerator::cylinder_mesh(1.0, 2.0, 8);

        assert!(mesh.vertices.len() > 0);
        assert_eq!(mesh.vertices.len(), mesh.normals.len());
        assert!(mesh.indices.len() % 3 == 0);
    }

    #[test]
    fn test_sphere_mesh() {
        let mesh = MeshGenerator::sphere_mesh(1.0, 8, 8);

        assert!(mesh.vertices.len() > 0);
        assert_eq!(mesh.vertices.len(), mesh.normals.len());
        assert!(mesh.indices.len() % 3 == 0);
    }
}
