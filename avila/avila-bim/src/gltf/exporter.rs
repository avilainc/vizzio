//! glTF/GLB Exporter

use crate::bim_core::*;
use std::io::Cursor;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GltfError {
    #[error("Export error: {0}")]
    ExportError(String),

    #[error("Invalid geometry")]
    InvalidGeometry,

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, GltfError>;

/// Opções de exportação glTF
#[derive(Debug, Clone)]
pub struct ExportOptions {
    pub merge_meshes: bool,
    pub include_normals: bool,
    pub include_uvs: bool,
    pub include_colors: bool,
    pub use_draco_compression: bool,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            merge_meshes: true,
            include_normals: true,
            include_uvs: false,
            include_colors: false,
            use_draco_compression: false,
        }
    }
}

/// Exportador glTF principal
pub struct GltfExporter;

impl GltfExporter {
    pub fn new() -> Self {
        Self
    }

    /// Exportar BimModel → GLB (binário)
    pub fn export_glb(&self, model: &BimModel, options: &ExportOptions) -> Result<Vec<u8>> {
        // TODO: Implementar exportação GLB
        // 1. Construir scene graph
        // 2. Converter meshes → glTF buffers
        // 3. Converter materiais → PBR
        // 4. Serializar JSON + BIN em formato GLB

        Ok(vec![])
    }

    /// Exportar BimModel → glTF JSON + BIN separados
    pub fn export_gltf(&self, model: &BimModel, options: &ExportOptions) -> Result<(String, Vec<u8>)> {
        // TODO: Implementar exportação glTF
        let json = "{}";
        let bin = vec![];
        Ok((json.to_string(), bin))
    }
}

impl Default for GltfExporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_empty_model() {
        let model = BimModel::new("Test", IfcSchema::Ifc4);
        let exporter = GltfExporter::new();
        let options = ExportOptions::default();

        let result = exporter.export_glb(&model, &options);
        assert!(result.is_ok());
    }
}
