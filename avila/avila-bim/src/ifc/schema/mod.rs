//! IFC Schema definitions

pub mod ifc2x3;
pub mod ifc4;
pub mod ifc4x3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfcVersion {
    Ifc2x3,
    Ifc4,
    Ifc4x3,
}

impl IfcVersion {
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "IFC2X3" => Some(Self::Ifc2x3),
            "IFC4" => Some(Self::Ifc4),
            "IFC4X3" | "IFC4.3" => Some(Self::Ifc4x3),
            _ => None,
        }
    }
}
