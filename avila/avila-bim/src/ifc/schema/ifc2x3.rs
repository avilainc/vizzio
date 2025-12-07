//! IFC 2x3 Schema definitions

/// IFC 2x3 Entity types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ifc2x3Entity {
    // Spatial Structure
    IfcProject,
    IfcSite,
    IfcBuilding,
    IfcBuildingStorey,
    IfcSpace,

    // Building Elements
    IfcWall,
    IfcWallStandardCase,
    IfcSlab,
    IfcColumn,
    IfcBeam,
    IfcDoor,
    IfcWindow,
    IfcRoof,
    IfcStair,

    // Other
    Unknown(String),
}

impl Ifc2x3Entity {
    pub fn from_string(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "IFCPROJECT" => Self::IfcProject,
            "IFCSITE" => Self::IfcSite,
            "IFCBUILDING" => Self::IfcBuilding,
            "IFCBUILDINGSTOREY" => Self::IfcBuildingStorey,
            "IFCSPACE" => Self::IfcSpace,
            "IFCWALL" => Self::IfcWall,
            "IFCWALLSTANDARDCASE" => Self::IfcWallStandardCase,
            "IFCSLAB" => Self::IfcSlab,
            "IFCCOLUMN" => Self::IfcColumn,
            "IFCBEAM" => Self::IfcBeam,
            "IFCDOOR" => Self::IfcDoor,
            "IFCWINDOW" => Self::IfcWindow,
            "IFCROOF" => Self::IfcRoof,
            "IFCSTAIR" => Self::IfcStair,
            _ => Self::Unknown(s.to_string()),
        }
    }
}
