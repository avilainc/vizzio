//! Geometry converter orchestrator

use crate::bim_core::*;
use crate::ifc::entities::geometry::GeometryConverter;

/// Orquestrador de conversão de geometria IFC
pub struct IfcGeometryConverter {
    tessellation_quality: f64,
}

impl IfcGeometryConverter {
    pub fn new() -> Self {
        Self {
            tessellation_quality: 1.0,
        }
    }

    /// Definir qualidade de tessellation (0.1 a 2.0)
    pub fn with_quality(mut self, quality: f64) -> Self {
        self.tessellation_quality = quality.clamp(0.1, 2.0);
        self
    }

    /// Converter BimGeometry → Mesh
    pub fn convert(&self, geometry: &BimGeometry) -> Result<Mesh> {
        match geometry {
            BimGeometry::ExtrudedSolid { profile, depth, .. } => {
                GeometryConverter::convert_extruded_solid(*profile, *depth)
            }
            BimGeometry::TriangulatedMesh { vertices, normals, indices } => {
                self.convert_triangulated_mesh(vertices, normals, indices)
            }
            BimGeometry::BooleanOperation { .. } => {
                Err(BimError::InvalidGeometry("Boolean ops not implemented".into()))
            }
        }
    }

    /// Converter mesh já triangulada
    fn convert_triangulated_mesh(
        &self,
        vertices: &[[f64; 3]],
        normals: &[[f64; 3]],
        indices: &[u32],
    ) -> Result<Mesh> {
        let vertices_f32: Vec<f32> = vertices
            .iter()
            .flat_map(|v| v.iter().map(|&x| x as f32))
            .collect();

        let normals_f32: Vec<f32> = normals
            .iter()
            .flat_map(|n| n.iter().map(|&x| x as f32))
            .collect();

        Ok(Mesh {
            vertices: vertices_f32,
            normals: normals_f32,
            indices: indices.to_vec(),
            uvs: None,
            colors: None,
        })
    }
}

impl Default for IfcGeometryConverter {
    fn default() -> Self {
        Self::new()
    }
}
