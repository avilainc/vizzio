//! # AVX Quantum Render - QED Path Integral Renderer
//!
//! **Renderização baseada em Eletrodinâmica Quântica (QED)** usando formulação de integrais de caminho.
//!
//! Este módulo implementa renderização de luz usando princípios fundamentais da física quântica,
//! onde a amplitude total é a soma de contribuições de todos os caminhos possíveis que um fóton pode seguir.
//!
//! ## Teoria
//!
//! Na QED, a amplitude para um fóton ir de A → B é:
//!
//! ```text
//! A(A→B) = Σ_caminhos exp(i·S[caminho]/ℏ)
//! ```
//!
//! Onde S é a ação: `S = ∫(n·ℏω - p·v)dt`
//!
//! ## Exemplo Básico
//!
//! ```rust,ignore
//! use avx_quantum_render::*;
//!
//! // Criar cena
//! let mut scene = Scene::new();
//! scene.add_light(Light::point([0.0, 5.0, 0.0], 100.0));
//! scene.add_surface(Surface::lambertian([0.0, 0.0, 0.0], 0.8));
//!
//! // Criar renderer
//! let renderer = QEDRenderer::new(1000); // 1000 samples per pixel
//!
//! // Renderizar
//! let image = renderer.render(&scene, 800, 600);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod amplitude;
pub mod denoise;
pub mod diagnostics;
pub mod export;
pub mod feynman;
pub mod photon;
pub mod renderer;
pub mod scene;
pub mod gpu_kernel;
pub mod backend_selector;
pub mod optimization;
pub mod performance;
pub mod materials;
pub mod textures;
pub mod cameras;
pub mod serialization;
pub mod postprocessing;
pub mod acceleration;
pub mod utils;
pub mod errors;
pub mod cache;
pub mod simd;
pub mod sync;

// Re-exports
pub use amplitude::{ComplexAmplitude, PhaseAccumulator};
pub use optimization::{ParallelConfig, TileConfig};
pub use performance::{PerfMetrics, PerfTimer, BenchmarkResult};
pub use materials::MaterialProperties;
pub use textures::TexCoord;
pub use cameras::AdvancedCamera;
pub use serialization::{ImageBuffer, ColorFormat};
pub use postprocessing::ToneMapper;
pub use acceleration::AABB;
pub use utils::{Vec3, Random, Math};
pub use denoise::{bilateral_filter, gaussian_blur, Image as DenoiseImage};
pub use diagnostics::{RenderMetrics, RenderTimer, SpectralMode};
pub use gpu_kernel::{GpuPathTracingConfig, GpuPathTracingKernel, GpuRenderBuffer};
pub use backend_selector::{BackendType, BackendInfo};
pub use export::{export_ascii, export_png, export_ppm, ExportError, ExportResult};
pub use feynman::{FeynmanDiagram, FeynmanVertex};
pub use photon::{Interaction, InteractionType, PhotonPath, Vertex};
pub use renderer::{QEDRenderer, RenderConfig};
pub use scene::{Light, Material, Scene, Surface};
pub use errors::{RenderError, RenderResult, Validator};
pub use cache::{TextureCache, ObjectPool, Arena, CacheStats};
pub use simd::{SimdVec, SimdColor, ParallelRays};
pub use sync::{SafeScene, RenderQueue, RenderJob, MetricsAccumulator, ConcurrentMetrics};

/// Constante de Planck reduzida (ℏ)
pub const HBAR: f64 = 1.054571817e-34; // J·s

/// Velocidade da luz no vácuo (c)
pub const SPEED_OF_LIGHT: f64 = 299792458.0; // m/s

/// Constante de estrutura fina (α ≈ 1/137)
pub const FINE_STRUCTURE: f64 = 7.2973525693e-3;

/// Carga do elétron (e)
pub const ELECTRON_CHARGE: f64 = 1.602176634e-19; // C

/// Prelude - tipos comumente usados
pub mod prelude {
    pub use crate::{
        bilateral_filter, gaussian_blur, ComplexAmplitude, DenoiseImage, FeynmanDiagram,
        FeynmanVertex, Interaction, InteractionType, Light, Material, PhaseAccumulator, PhotonPath,
        QEDRenderer, RenderConfig, RenderMetrics, RenderTimer, Scene, SpectralMode, Surface,
        Vertex,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert!(HBAR > 0.0);
        assert!(SPEED_OF_LIGHT > 0.0);
        assert!((FINE_STRUCTURE - 1.0 / 137.0).abs() < 0.001);
    }
}
