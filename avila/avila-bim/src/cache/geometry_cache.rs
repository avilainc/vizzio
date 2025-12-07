//! Geometry cache (Redis)

use crate::bim_core::*;
use std::collections::HashMap;

/// Cache de geometria
pub struct GeometryCache {
    // TODO: Adicionar redis::Client
    memory_cache: HashMap<String, Mesh>,
}

impl GeometryCache {
    pub fn new() -> Self {
        Self {
            memory_cache: HashMap::new(),
        }
    }

    /// Gerar hash de geometria
    pub fn compute_hash(mesh: &Mesh) -> String {
        crate::hash::SimpleHash::hash_mesh_data(&mesh.vertices, &mesh.indices)
    }

    /// Obter mesh do cache
    pub fn get(&self, hash: &str) -> Option<&Mesh> {
        self.memory_cache.get(hash)
    }

    /// Adicionar mesh ao cache
    pub fn set(&mut self, hash: String, mesh: Mesh) {
        self.memory_cache.insert(hash, mesh);
    }

    /// Limpar cache
    pub fn clear(&mut self) {
        self.memory_cache.clear();
    }
}

impl Default for GeometryCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometry_cache() {
        let mut cache = GeometryCache::new();

        let mesh = Mesh {
            vertices: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            normals: vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0],
            indices: vec![0, 1, 2],
            uvs: None,
            colors: None,
        };

        let hash = GeometryCache::compute_hash(&mesh);
        cache.set(hash.clone(), mesh);

        assert!(cache.get(&hash).is_some());
    }
}
