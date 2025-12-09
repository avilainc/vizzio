//! Level of Detail (LOD) system for dynamic mesh simplification

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

/// LOD levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LodLevel {
    /// Full detail - all vertices
    High = 0,
    /// Medium detail - 50% vertices
    Medium = 1,
    /// Low detail - 25% vertices
    Low = 2,
    /// Minimal detail - bounding box only
    Minimal = 3,
}

/// LOD configuration
#[derive(Debug, Clone)]
pub struct LodConfig {
    /// Distance thresholds for each LOD level
    pub distances: [f32; 4],
    /// Enable automatic LOD transitions
    pub auto_transition: bool,
    /// Fade duration between LODs (seconds)
    pub fade_duration: f32,
}

impl Default for LodConfig {
    fn default() -> Self {
        Self {
            distances: [10.0, 50.0, 150.0, 500.0], // meters
            auto_transition: true,
            fade_duration: 0.3,
        }
    }
}

impl LodConfig {
    /// Calculate LOD level based on distance
    pub fn calculate_lod(&self, distance: f32) -> LodLevel {
        if distance < self.distances[0] {
            LodLevel::High
        } else if distance < self.distances[1] {
            LodLevel::Medium
        } else if distance < self.distances[2] {
            LodLevel::Low
        } else {
            LodLevel::Minimal
        }
    }
}

/// LOD mesh representation
#[derive(Debug, Clone)]
pub struct LodMesh {
    /// Original high-detail mesh
    pub high: MeshData,
    /// Medium detail mesh (optional)
    pub medium: Option<MeshData>,
    /// Low detail mesh (optional)
    pub low: Option<MeshData>,
    /// Minimal representation (bounding box)
    pub minimal: BoundingBox,
}

/// Simplified mesh data
#[derive(Debug, Clone)]
pub struct MeshData {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub normals: Vec<[f32; 3]>,
}

/// Axis-aligned bounding box
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl BoundingBox {
    /// Create from vertices
    pub fn from_vertices(vertices: &[[f32; 3]]) -> Option<Self> {
        if vertices.is_empty() {
            return None;
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

        Some(Self { min, max })
    }

    /// Get center point
    pub fn center(&self) -> [f32; 3] {
        [
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
            (self.min[2] + self.max[2]) * 0.5,
        ]
    }

    /// Get size in each dimension
    pub fn size(&self) -> [f32; 3] {
        [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1],
            self.max[2] - self.min[2],
        ]
    }

    /// Get diagonal length
    pub fn diagonal(&self) -> f32 {
        let size = self.size();
        (size[0] * size[0] + size[1] * size[1] + size[2] * size[2]).sqrt()
    }

    /// As cube vertices (8 corners)
    pub fn as_cube_vertices(&self) -> [[f32; 3]; 8] {
        [
            [self.min[0], self.min[1], self.min[2]],
            [self.max[0], self.min[1], self.min[2]],
            [self.max[0], self.max[1], self.min[2]],
            [self.min[0], self.max[1], self.min[2]],
            [self.min[0], self.min[1], self.max[2]],
            [self.max[0], self.min[1], self.max[2]],
            [self.max[0], self.max[1], self.max[2]],
            [self.min[0], self.max[1], self.max[2]],
        ]
    }
}

impl LodMesh {
    /// Create from high-detail mesh with auto-generated LODs
    pub fn new(vertices: Vec<[f32; 3]>, indices: Vec<u32>, normals: Vec<[f32; 3]>) -> Self {
        let high = MeshData { vertices: vertices.clone(), indices: indices.clone(), normals: normals.clone() };
        let minimal = BoundingBox::from_vertices(&vertices).unwrap_or(BoundingBox {
            min: [0.0, 0.0, 0.0],
            max: [1.0, 1.0, 1.0],
        });

        // Generate medium LOD (50% vertices)
        let medium = if vertices.len() > 100 {
            Some(Self::simplify_mesh(&high, 0.5))
        } else {
            None
        };

        // Generate low LOD (25% vertices)
        let low = if vertices.len() > 400 {
            Some(Self::simplify_mesh(&high, 0.25))
        } else {
            None
        };

        Self { high, medium, low, minimal }
    }

    /// Simplify mesh by reducing vertex count
    fn simplify_mesh(mesh: &MeshData, factor: f32) -> MeshData {
        let target_vertices = ((mesh.vertices.len() as f32) * factor) as usize;
        let step = (mesh.vertices.len() / target_vertices.max(1)).max(1);

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();
        let mut vertex_map = Vec::new();

        // Sample vertices at intervals
        for (i, v) in mesh.vertices.iter().enumerate() {
            if i % step == 0 {
                vertex_map.push((i, vertices.len()));
                vertices.push(*v);
                if i < mesh.normals.len() {
                    normals.push(mesh.normals[i]);
                }
            }
        }

        // Rebuild indices
        for chunk in mesh.indices.chunks(3) {
            if chunk.len() == 3 {
                let mut mapped = [0u32; 3];
                let mut valid = true;

                for (i, &idx) in chunk.iter().enumerate() {
                    if let Some((_, new_idx)) = vertex_map.iter().find(|(old_idx, _)| *old_idx == idx as usize) {
                        mapped[i] = *new_idx as u32;
                    } else {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    indices.extend_from_slice(&mapped);
                }
            }
        }

        MeshData { vertices, indices, normals }
    }

    /// Select LOD level based on distance
    pub fn select_lod(&self, distance: f32, config: &LodConfig) -> LodLevel {
        if distance < config.distances[0] {
            LodLevel::High
        } else if distance < config.distances[1] {
            LodLevel::Medium
        } else if distance < config.distances[2] {
            LodLevel::Low
        } else {
            LodLevel::Minimal
        }
    }

    /// Get mesh data for LOD level
    pub fn get_mesh(&self, level: LodLevel) -> Option<&MeshData> {
        match level {
            LodLevel::High => Some(&self.high),
            LodLevel::Medium => self.medium.as_ref(),
            LodLevel::Low => self.low.as_ref(),
            LodLevel::Minimal => None, // Use bounding box
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box() {
        let vertices = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];

        let bbox = BoundingBox::from_vertices(&vertices).unwrap();
        assert_eq!(bbox.min, [0.0, 0.0, 0.0]);
        assert_eq!(bbox.max, [1.0, 1.0, 0.0]);
        assert_eq!(bbox.center(), [0.5, 0.5, 0.0]);
    }

    #[test]
    fn test_lod_selection() {
        let vertices = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.5, 1.0, 0.0]];
        let indices = vec![0, 1, 2];
        let normals = vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]];

        let lod_mesh = LodMesh::new(vertices, indices, normals);
        let config = LodConfig::default();

        assert_eq!(lod_mesh.select_lod(5.0, &config), LodLevel::High);
        assert_eq!(lod_mesh.select_lod(30.0, &config), LodLevel::Medium);
        assert_eq!(lod_mesh.select_lod(100.0, &config), LodLevel::Low);
        assert_eq!(lod_mesh.select_lod(600.0, &config), LodLevel::Minimal);
    }
}
