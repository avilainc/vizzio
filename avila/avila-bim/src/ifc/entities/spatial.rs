//! Spatial structure entities
//! IfcProject, IfcSite, IfcBuilding, IfcBuildingStorey, IfcSpace

use crate::bim_core::*;

/// Converter para elementos espaciais
pub struct SpatialConverter;

impl SpatialConverter {
    /// Converter IfcProject → SpatialNode
    pub fn convert_project(
        guid: IfcGuid,
        name: String,
    ) -> SpatialNode {
        SpatialNode::new(guid, name, SpatialNodeType::Project)
    }

    /// Converter IfcBuilding → SpatialNode
    pub fn convert_building(
        guid: IfcGuid,
        name: String,
    ) -> SpatialNode {
        SpatialNode::new(guid, name, SpatialNodeType::Building)
    }

    /// Converter IfcBuildingStorey → SpatialNode
    pub fn convert_floor(
        guid: IfcGuid,
        name: String,
    ) -> SpatialNode {
        SpatialNode::new(guid, name, SpatialNodeType::Floor)
    }

    /// Converter IfcSpace → SpatialNode
    pub fn convert_space(
        guid: IfcGuid,
        name: String,
    ) -> SpatialNode {
        SpatialNode::new(guid, name, SpatialNodeType::Space)
    }
}
