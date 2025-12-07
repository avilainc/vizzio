//! glTF Mesh builder

use crate::bim_core::*;

/// Construtor de meshes glTF
pub struct MeshBuilder {
    vertices: Vec<f32>,
    normals: Vec<f32>,
    indices: Vec<u32>,
    uvs: Option<Vec<f32>>,
    colors: Option<Vec<f32>>,
}

impl MeshBuilder {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            indices: Vec::new(),
            uvs: None,
            colors: None,
        }
    }

    /// Adicionar mesh ao buffer
    pub fn add_mesh(&mut self, mesh: &Mesh) {
        let vertex_offset = (self.vertices.len() / 3) as u32;

        // Adicionar vértices
        self.vertices.extend_from_slice(&mesh.vertices);
        self.normals.extend_from_slice(&mesh.normals);

        // Ajustar índices com offset
        for &idx in &mesh.indices {
            self.indices.push(idx + vertex_offset);
        }

        // UVs e cores (opcional)
        if let Some(ref uvs) = mesh.uvs {
            self.uvs.get_or_insert_with(Vec::new).extend_from_slice(uvs);
        }

        if let Some(ref colors) = mesh.colors {
            self.colors.get_or_insert_with(Vec::new).extend_from_slice(colors);
        }
    }

    /// Construir mesh final
    pub fn build(self) -> Mesh {
        Mesh {
            vertices: self.vertices,
            normals: self.normals,
            indices: self.indices,
            uvs: self.uvs,
            colors: self.colors,
        }
    }

    /// Calcular bounding box
    pub fn bounding_box(&self) -> BoundingBox {
        BoundingBox::from_vertices(&self.vertices)
    }
}

impl Default for MeshBuilder {
    fn default() -> Self {
        Self::new()
    }
}
