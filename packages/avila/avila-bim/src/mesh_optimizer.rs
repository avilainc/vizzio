//! Mesh optimization utilities (Rust puro)

use crate::bim_core::*;
use std::collections::HashMap;

/// Otimizador de meshes
pub struct MeshOptimizer;

impl MeshOptimizer {
    /// Remover vértices duplicados
    pub fn remove_duplicate_vertices(mesh: &Mesh) -> Mesh {
        let mut unique_vertices: HashMap<[u32; 3], u32> = HashMap::new();
        let mut new_vertices = Vec::new();
        let mut new_normals = Vec::new();
        let mut new_indices = Vec::new();
        let mut vertex_map = Vec::new();

        // Processar vértices
        for i in 0..(mesh.vertices.len() / 3) {
            let idx = i * 3;
            let v = [
                mesh.vertices[idx].to_bits(),
                mesh.vertices[idx + 1].to_bits(),
                mesh.vertices[idx + 2].to_bits(),
            ];

            let new_idx = if let Some(&existing_idx) = unique_vertices.get(&v) {
                existing_idx
            } else {
                let new_idx = (new_vertices.len() / 3) as u32;
                unique_vertices.insert(v, new_idx);

                new_vertices.extend_from_slice(&[
                    mesh.vertices[idx],
                    mesh.vertices[idx + 1],
                    mesh.vertices[idx + 2],
                ]);

                new_normals.extend_from_slice(&[
                    mesh.normals[idx],
                    mesh.normals[idx + 1],
                    mesh.normals[idx + 2],
                ]);

                new_idx
            };

            vertex_map.push(new_idx);
        }

        // Remapear índices
        for &idx in &mesh.indices {
            new_indices.push(vertex_map[idx as usize]);
        }

        Mesh {
            vertices: new_vertices,
            normals: new_normals,
            indices: new_indices,
            uvs: mesh.uvs.clone(),
            colors: mesh.colors.clone(),
        }
    }

    /// Remover triângulos degenerados
    pub fn remove_degenerate_triangles(mesh: &Mesh) -> Mesh {
        let mut new_indices = Vec::new();
        const EPSILON: f32 = 1e-6;

        for i in (0..mesh.indices.len()).step_by(3) {
            let i0 = mesh.indices[i] as usize * 3;
            let i1 = mesh.indices[i + 1] as usize * 3;
            let i2 = mesh.indices[i + 2] as usize * 3;

            let v0 = [mesh.vertices[i0], mesh.vertices[i0 + 1], mesh.vertices[i0 + 2]];
            let v1 = [mesh.vertices[i1], mesh.vertices[i1 + 1], mesh.vertices[i1 + 2]];
            let v2 = [mesh.vertices[i2], mesh.vertices[i2 + 1], mesh.vertices[i2 + 2]];

            // Calcular área do triângulo
            let edge1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
            let edge2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

            let cross = [
                edge1[1] * edge2[2] - edge1[2] * edge2[1],
                edge1[2] * edge2[0] - edge1[0] * edge2[2],
                edge1[0] * edge2[1] - edge1[1] * edge2[0],
            ];

            let area = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();

            // Se área > epsilon, manter triângulo
            if area > EPSILON {
                new_indices.push(mesh.indices[i]);
                new_indices.push(mesh.indices[i + 1]);
                new_indices.push(mesh.indices[i + 2]);
            }
        }

        Mesh {
            vertices: mesh.vertices.clone(),
            normals: mesh.normals.clone(),
            indices: new_indices,
            uvs: mesh.uvs.clone(),
            colors: mesh.colors.clone(),
        }
    }

    /// Combinar múltiplas meshes em uma única
    pub fn merge_meshes(meshes: &[Mesh]) -> Mesh {
        let mut merged_vertices = Vec::new();
        let mut merged_normals = Vec::new();
        let mut merged_indices = Vec::new();

        let mut vertex_offset = 0u32;

        for mesh in meshes {
            // Adicionar vértices e normais
            merged_vertices.extend_from_slice(&mesh.vertices);
            merged_normals.extend_from_slice(&mesh.normals);

            // Adicionar índices com offset
            for &idx in &mesh.indices {
                merged_indices.push(idx + vertex_offset);
            }

            vertex_offset += (mesh.vertices.len() / 3) as u32;
        }

        Mesh {
            vertices: merged_vertices,
            normals: merged_normals,
            indices: merged_indices,
            uvs: None,
            colors: None,
        }
    }

    /// Calcular e aplicar smooth normals (média de normais por vértice)
    pub fn compute_smooth_normals(mesh: &mut Mesh) {
        let vertex_count = mesh.vertices.len() / 3;
        let mut vertex_normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 0.0]; vertex_count];
        let mut vertex_counts = vec![0u32; vertex_count];

        // Acumular normais por vértice
        for i in (0..mesh.indices.len()).step_by(3) {
            let i0 = mesh.indices[i] as usize;
            let i1 = mesh.indices[i + 1] as usize;
            let i2 = mesh.indices[i + 2] as usize;

            let v0 = [
                mesh.vertices[i0 * 3],
                mesh.vertices[i0 * 3 + 1],
                mesh.vertices[i0 * 3 + 2],
            ];
            let v1 = [
                mesh.vertices[i1 * 3],
                mesh.vertices[i1 * 3 + 1],
                mesh.vertices[i1 * 3 + 2],
            ];
            let v2 = [
                mesh.vertices[i2 * 3],
                mesh.vertices[i2 * 3 + 1],
                mesh.vertices[i2 * 3 + 2],
            ];

            // Calcular normal da face
            let edge1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
            let edge2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

            let normal = [
                edge1[1] * edge2[2] - edge1[2] * edge2[1],
                edge1[2] * edge2[0] - edge1[0] * edge2[2],
                edge1[0] * edge2[1] - edge1[1] * edge2[0],
            ];

            // Acumular em cada vértice
            for &idx in &[i0, i1, i2] {
                vertex_normals[idx][0] += normal[0];
                vertex_normals[idx][1] += normal[1];
                vertex_normals[idx][2] += normal[2];
                vertex_counts[idx] += 1;
            }
        }

        // Normalizar
        mesh.normals.clear();
        for i in 0..vertex_count {
            let mut normal = vertex_normals[i];
            let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();

            if length > 1e-6 {
                normal[0] /= length;
                normal[1] /= length;
                normal[2] /= length;
            } else {
                normal = [0.0, 1.0, 0.0];
            }

            mesh.normals.extend_from_slice(&normal);
        }
    }

    /// Reverter orientação de faces (flip normals)
    pub fn flip_normals(mesh: &mut Mesh) {
        // Inverter ordem dos índices
        for i in (0..mesh.indices.len()).step_by(3) {
            mesh.indices.swap(i + 1, i + 2);
        }

        // Inverter normais
        for i in 0..mesh.normals.len() {
            mesh.normals[i] = -mesh.normals[i];
        }
    }

    /// Calcular estatísticas da mesh
    pub fn compute_stats(mesh: &Mesh) -> MeshStats {
        let vertex_count = mesh.vertices.len() / 3;
        let triangle_count = mesh.indices.len() / 3;

        // Calcular bounds
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];

        for i in 0..vertex_count {
            for j in 0..3 {
                let v = mesh.vertices[i * 3 + j];
                min[j] = min[j].min(v);
                max[j] = max[j].max(v);
            }
        }

        let bounds = BoundingBox {
            min: [min[0] as f64, min[1] as f64, min[2] as f64],
            max: [max[0] as f64, max[1] as f64, max[2] as f64],
        };

        MeshStats {
            vertex_count,
            triangle_count,
            bounds,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MeshStats {
    pub vertex_count: usize,
    pub triangle_count: usize,
    pub bounds: BoundingBox,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mesh = Mesh {
            vertices: vec![
                0.0, 0.0, 0.0,
                1.0, 0.0, 0.0,
                1.0, 0.0, 0.0, // Duplicado
                0.0, 1.0, 0.0,
            ],
            normals: vec![
                0.0, 0.0, 1.0,
                0.0, 0.0, 1.0,
                0.0, 0.0, 1.0,
                0.0, 0.0, 1.0,
            ],
            indices: vec![0, 1, 3, 1, 2, 3],
            uvs: None,
            colors: None,
        };

        let optimized = MeshOptimizer::remove_duplicate_vertices(&mesh);
        assert_eq!(optimized.vertex_count(), 3); // Removeu 1 duplicado
    }

    #[test]
    fn test_merge_meshes() {
        let mesh1 = crate::mesh_gen::MeshGenerator::box_mesh(1.0, 1.0, 1.0);
        let mesh2 = crate::mesh_gen::MeshGenerator::box_mesh(1.0, 1.0, 1.0);

        let merged = MeshOptimizer::merge_meshes(&[mesh1.clone(), mesh2]);

        assert_eq!(merged.vertex_count(), mesh1.vertex_count() * 2);
        assert_eq!(merged.triangle_count(), mesh1.triangle_count() * 2);
    }
}
