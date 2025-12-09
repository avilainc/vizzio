#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! # Avila BIM - Building Information Modeling
//!
//! Parser IFC (Industry Foundation Classes) STEP format para extração de geometria 3D.

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

const _: &str = "lib avila sempre";

pub mod ifc;
pub mod geometry;
pub mod geometry_extra;
pub mod tree;

use avila_error::Result;

/// Entidade IFC básica
#[derive(Debug, Clone)]
pub struct IfcEntity {
    /// ID da entidade
    pub id: u32,
    /// Tipo da entidade (IfcWall, IfcSlab, etc)
    pub entity_type: String,
    /// Parâmetros
    pub params: Vec<String>,
}

/// Modelo IFC completo
#[derive(Debug)]
pub struct IfcModel {
    /// Entidades do modelo
    pub entities: Vec<IfcEntity>,
    /// Header do arquivo
    pub header: IfcHeader,
}

/// Header do arquivo IFC
#[derive(Debug, Default)]
pub struct IfcHeader {
    /// Nome do arquivo
    pub file_name: String,
    /// Descrição
    pub description: String,
    /// Schema IFC (IFC2X3, IFC4, etc)
    pub schema: String,
}

/// Geometria triangulada extraída
#[derive(Debug, Clone)]
pub struct IfcGeometry {
    /// ID da entidade IFC
    pub entity_id: u32,
    /// Tipo da entidade (IFCWALL, IFCBEAM, etc)
    pub entity_type: String,
    /// Vértices (x, y, z)
    pub vertices: Vec<[f32; 3]>,
    /// Índices dos triângulos
    pub indices: Vec<u32>,
    /// Normais
    pub normals: Vec<[f32; 3]>,
    /// Cor do material (r, g, b, a)
    pub color: [f32; 4],
    /// Bounding box mínimo (x, y, z)
    pub bbox_min: [f32; 3],
    /// Bounding box máximo (x, y, z)
    pub bbox_max: [f32; 3],
}

impl IfcModel {
    /// Parse IFC STEP file
    pub fn from_step(content: &str) -> Result<Self> {
        ifc::parse_step(content)
    }

    /// Extrai geometria de todas as entidades
    pub fn extract_geometry(&self) -> Result<Vec<IfcGeometry>> {
        geometry::extract_all(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ifc_model_creation() {
        let model = IfcModel {
            entities: Vec::new(),
            header: IfcHeader::default(),
        };
        assert_eq!(model.entities.len(), 0);
    }
}
