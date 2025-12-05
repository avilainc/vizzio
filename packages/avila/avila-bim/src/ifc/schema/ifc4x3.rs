//! IFC 4.3 Schema definitions

/// IFC 4.3 Entity types (includes IFC4 + extensions)
pub use super::ifc4::Ifc4Entity as Ifc4x3Entity;

// IFC 4.3 adds infrastructure elements
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfrastructureEntity {
    IfcRoad,
    IfcRoadway,
    IfcBridge,
    IfcBridgepart,
    IfcRailway,
    IfcMarineFacility,
}
