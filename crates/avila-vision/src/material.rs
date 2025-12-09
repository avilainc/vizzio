//! Material system with PBR properties

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/// Material properties (PBR)
#[derive(Debug, Clone)]
pub struct Material {
    /// Material name
    pub name: String,

    /// Base color (albedo) [R, G, B, A]
    pub base_color: [f32; 4],

    /// Metallic factor (0.0 = dielectric, 1.0 = metal)
    pub metallic: f32,

    /// Roughness factor (0.0 = smooth, 1.0 = rough)
    pub roughness: f32,

    /// Normal map strength
    pub normal_strength: f32,

    /// Emissive color [R, G, B]
    pub emissive: [f32; 3],

    /// Emissive strength
    pub emissive_strength: f32,

    /// Alpha mode
    pub alpha_mode: AlphaMode,

    /// Alpha cutoff (for AlphaMode::Mask)
    pub alpha_cutoff: f32,

    /// Double-sided rendering
    pub double_sided: bool,

    /// Texture IDs
    pub textures: MaterialTextures,
}

/// Texture references
#[derive(Debug, Clone, Default)]
pub struct MaterialTextures {
    /// Base color texture ID
    pub base_color: Option<u32>,

    /// Metallic-roughness texture ID (R=unused, G=roughness, B=metallic)
    pub metallic_roughness: Option<u32>,

    /// Normal map texture ID
    pub normal: Option<u32>,

    /// Occlusion texture ID (R channel)
    pub occlusion: Option<u32>,

    /// Emissive texture ID
    pub emissive: Option<u32>,
}

/// Alpha rendering mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaMode {
    /// Fully opaque
    Opaque,

    /// Binary transparency (alpha cutoff)
    Mask,

    /// Blended transparency
    Blend,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: String::new(),
            base_color: [1.0, 1.0, 1.0, 1.0],
            metallic: 0.0,
            roughness: 0.5,
            normal_strength: 1.0,
            emissive: [0.0, 0.0, 0.0],
            emissive_strength: 0.0,
            alpha_mode: AlphaMode::Opaque,
            alpha_cutoff: 0.5,
            double_sided: false,
            textures: MaterialTextures::default(),
        }
    }
}

impl Material {
    /// Create opaque material with color
    pub fn opaque(color: [f32; 4]) -> Self {
        Self {
            base_color: color,
            ..Default::default()
        }
    }

    /// Create metallic material
    pub fn metallic(color: [f32; 4], metallic: f32, roughness: f32) -> Self {
        Self {
            base_color: color,
            metallic,
            roughness,
            ..Default::default()
        }
    }

    /// Create glass-like material
    pub fn glass(color: [f32; 4], roughness: f32) -> Self {
        Self {
            base_color: [color[0], color[1], color[2], 0.3],
            metallic: 0.0,
            roughness,
            alpha_mode: AlphaMode::Blend,
            double_sided: true,
            ..Default::default()
        }
    }

    /// Create emissive material
    pub fn emissive(base_color: [f32; 4], emissive: [f32; 3], strength: f32) -> Self {
        Self {
            base_color,
            emissive,
            emissive_strength: strength,
            ..Default::default()
        }
    }
}

/// Material library - manages all materials
#[derive(Debug)]
pub struct MaterialLibrary {
    materials: Vec<Material>,
}

impl Default for MaterialLibrary {
    fn default() -> Self {
        let mut lib = Self {
            materials: Vec::new(),
        };

        // Add default materials
        lib.add(Material::opaque([0.8, 0.8, 0.8, 1.0])); // Default gray
        lib.add(Material::metallic([0.7, 0.7, 0.7, 1.0], 1.0, 0.3)); // Metal
        lib.add(Material::glass([0.9, 0.9, 1.0, 0.3], 0.1)); // Glass
        lib.add(Material::emissive([1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0], 1.0)); // Light

        lib
    }
}

impl MaterialLibrary {
    /// Create new empty library
    pub fn new() -> Self {
        Self {
            materials: Vec::new(),
        }
    }

    /// Add material, returns ID
    pub fn add(&mut self, material: Material) -> u32 {
        let id = self.materials.len() as u32;
        self.materials.push(material);
        id
    }

    /// Get material by ID
    pub fn get(&self, id: u32) -> Option<&Material> {
        self.materials.get(id as usize)
    }

    /// Get mutable material by ID
    pub fn get_mut(&mut self, id: u32) -> Option<&mut Material> {
        self.materials.get_mut(id as usize)
    }

    /// Count materials
    pub fn count(&self) -> usize {
        self.materials.len()
    }
}

/// IFC material preset generator
pub struct IfcMaterialPresets;

impl IfcMaterialPresets {
    /// Concrete material
    pub fn concrete() -> Material {
        Material {
            name: String::from("Concrete"),
            base_color: [0.7, 0.7, 0.7, 1.0],
            metallic: 0.0,
            roughness: 0.8,
            ..Default::default()
        }
    }

    /// Steel material
    pub fn steel() -> Material {
        Material {
            name: String::from("Steel"),
            base_color: [0.65, 0.65, 0.7, 1.0],
            metallic: 1.0,
            roughness: 0.4,
            ..Default::default()
        }
    }

    /// Wood material
    pub fn wood() -> Material {
        Material {
            name: String::from("Wood"),
            base_color: [0.6, 0.4, 0.2, 1.0],
            metallic: 0.0,
            roughness: 0.7,
            ..Default::default()
        }
    }

    /// Glass material
    pub fn glass() -> Material {
        Material {
            name: String::from("Glass"),
            base_color: [0.9, 0.9, 1.0, 0.2],
            metallic: 0.0,
            roughness: 0.1,
            alpha_mode: AlphaMode::Blend,
            double_sided: true,
            ..Default::default()
        }
    }

    /// Brick material
    pub fn brick() -> Material {
        Material {
            name: String::from("Brick"),
            base_color: [0.7, 0.3, 0.2, 1.0],
            metallic: 0.0,
            roughness: 0.9,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_creation() {
        let mat = Material::metallic([0.5, 0.5, 0.5, 1.0], 0.8, 0.3);
        assert_eq!(mat.metallic, 0.8);
        assert_eq!(mat.roughness, 0.3);
    }

    #[test]
    fn test_material_library() {
        let mut lib = MaterialLibrary::new();
        let id = lib.add(Material::default());
        assert_eq!(id, 0);
        assert!(lib.get(id).is_some());
    }

    #[test]
    fn test_ifc_presets() {
        let concrete = IfcMaterialPresets::concrete();
        assert_eq!(concrete.roughness, 0.8);

        let steel = IfcMaterialPresets::steel();
        assert_eq!(steel.metallic, 1.0);
    }
}
