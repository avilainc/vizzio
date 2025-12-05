//! Material cache

use crate::bim_core::*;
use std::collections::HashMap;

/// Cache de materiais
pub struct MaterialCache {
    materials: HashMap<String, Material>,
}

impl MaterialCache {
    pub fn new() -> Self {
        Self {
            materials: HashMap::new(),
        }
    }

    /// Adicionar material
    pub fn add(&mut self, name: String, material: Material) {
        self.materials.insert(name, material);
    }

    /// Obter material
    pub fn get(&self, name: &str) -> Option<&Material> {
        self.materials.get(name)
    }

    /// Obter ou criar material padrão
    pub fn get_or_default(&mut self, name: &str, default_color: [f32; 4]) -> Material {
        self.materials
            .entry(name.to_string())
            .or_insert_with(|| Material {
                name: name.to_string(),
                color: Some(default_color),
                metallic: 0.0,
                roughness: 0.8,
                textures: HashMap::new(),
            })
            .clone()
    }

    /// Limpar cache
    pub fn clear(&mut self) {
        self.materials.clear();
    }
}

impl Default for MaterialCache {
    fn default() -> Self {
        Self::new()
    }
}
