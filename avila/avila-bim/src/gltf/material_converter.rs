//! Material converter (IFC → glTF PBR)

use crate::bim_core::*;

/// Converter materiais IFC → glTF PBR
pub struct MaterialConverter;

impl MaterialConverter {
    /// Converter Material BIM → glTF PBR material
    pub fn convert_to_pbr(material: &Material) -> GltfMaterial {
        GltfMaterial {
            name: material.name.clone(),
            base_color: material.color.unwrap_or([0.8, 0.8, 0.8, 1.0]),
            metallic: material.metallic,
            roughness: material.roughness,
            emissive: [0.0, 0.0, 0.0],
            alpha_mode: AlphaMode::Opaque,
        }
    }

    /// Inferir PBR de cor
    pub fn from_color(color: [f32; 4]) -> GltfMaterial {
        GltfMaterial {
            name: "Default Material".to_string(),
            base_color: color,
            metallic: 0.0,
            roughness: 0.8,
            emissive: [0.0, 0.0, 0.0],
            alpha_mode: if color[3] < 1.0 {
                AlphaMode::Blend
            } else {
                AlphaMode::Opaque
            },
        }
    }
}

/// Material glTF PBR
#[derive(Debug, Clone)]
pub struct GltfMaterial {
    pub name: String,
    pub base_color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub emissive: [f32; 3],
    pub alpha_mode: AlphaMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaMode {
    Opaque,
    Mask,
    Blend,
}
