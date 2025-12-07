//! IFC4 Schema definitions

/// IFC4 Entity types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ifc4Entity {
    // Spatial Structure
    IfcProject,
    IfcSite,
    IfcBuilding,
    IfcBuildingStorey,
    IfcSpace,
    IfcZone,

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
    IfcRailing,
    IfcCurtainWall,

    // MEP Elements
    IfcPipeSegment,
    IfcDuctSegment,
    IfcCableSegment,

    // Other
    Unknown(String),
}

impl Ifc4Entity {
    pub fn from_string(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "IFCPROJECT" => Self::IfcProject,
            "IFCSITE" => Self::IfcSite,
            "IFCBUILDING" => Self::IfcBuilding,
            "IFCBUILDINGSTOREY" => Self::IfcBuildingStorey,
            "IFCSPACE" => Self::IfcSpace,
            "IFCZONE" => Self::IfcZone,
            "IFCWALL" => Self::IfcWall,
            "IFCWALLSTANDARDCASE" => Self::IfcWallStandardCase,
            "IFCSLAB" => Self::IfcSlab,
            "IFCCOLUMN" => Self::IfcColumn,
            "IFCBEAM" => Self::IfcBeam,
            "IFCDOOR" => Self::IfcDoor,
            "IFCWINDOW" => Self::IfcWindow,
            "IFCROOF" => Self::IfcRoof,
            "IFCSTAIR" => Self::IfcStair,
            "IFCRAILING" => Self::IfcRailing,
            "IFCCURTAINWALL" => Self::IfcCurtainWall,
            "IFCPIPESEGMENT" => Self::IfcPipeSegment,
            "IFCDUCTSEGMENT" => Self::IfcDuctSegment,
            "IFCCABLESEGMENT" => Self::IfcCableSegment,
            _ => Self::Unknown(s.to_string()),
        }
    }
}
