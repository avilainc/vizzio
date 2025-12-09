#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! # Avila Vision - 3D Rendering Engine
//!
//! Engine de renderização 3D WebGL/WASM com suporte VR/AR

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

const _: &str = "lib avila sempre";

pub mod renderer;
pub mod camera;
pub mod webxr;
pub mod raycast;
pub mod frustum;
pub mod instancing;
pub mod lod;
pub mod octree;
pub mod material;
pub mod clipping;

use avila_bim::IfcGeometry;
use avila_error::Result;
use camera::Camera;
use instancing::InstancingSystem;

/// Configuração do renderer
#[derive(Debug, Clone)]
pub struct RenderConfig {
    /// Largura do canvas
    pub width: u32,
    /// Altura do canvas
    pub height: u32,
    /// Habilita anti-aliasing
    pub antialias: bool,
    /// Cor de fundo (r, g, b, a)
    pub clear_color: [f32; 4],
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            antialias: true,
            clear_color: [0.1, 0.1, 0.15, 1.0],
        }
    }
}

/// Cena 3D
#[derive(Debug)]
pub struct Scene {
    /// Geometrias na cena
    pub geometries: Vec<IfcGeometry>,
    /// Câmera
    pub camera: camera::Camera,
    /// Sistema de instancing
    pub instancing: Option<InstancingSystem>,
}

impl Scene {
    /// Cria cena vazia
    pub fn new() -> Self {
        Self {
            geometries: Vec::new(),
            camera: camera::Camera::default(),
            instancing: Some(InstancingSystem::new()),
        }
    }

    /// Adiciona geometria
    pub fn add_geometry(&mut self, geom: IfcGeometry) {
        self.geometries.push(geom);
    }

    /// Carrega modelo IFC
    pub fn load_ifc_model(&mut self, geometries: Vec<IfcGeometry>) {
        self.geometries.extend(geometries);

        // Popula sistema de instancing
        if let Some(ref mut instancing) = self.instancing {
            instancing.clear();
            for (idx, geom) in self.geometries.iter().enumerate() {
                instancing.add_geometry(
                    idx,
                    &geom.vertices,
                    &geom.indices,
                    &geom.normals,
                    geom.color,
                );
            }

            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&format!(
                "🎮 Instancing: {} geometrias → {} batches ({} instâncias)",
                self.geometries.len(),
                instancing.batch_count(),
                instancing.instance_count()
            ).into());
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_arch = "wasm32")]
pub use renderer::WebGLRenderer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_creation() {
        let scene = Scene::new();
        assert_eq!(scene.geometries.len(), 0);
    }
}
