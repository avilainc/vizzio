//! Product entities (IfcWall, IfcSlab, IfcColumn, etc.)

use crate::bim_core::*;

/// Converter para elementos de produto
pub struct ProductConverter;

impl ProductConverter {
    /// Converter IfcWall → BimElement
    pub fn convert_wall(
        guid: IfcGuid,
        name: Option<String>,
    ) -> BimElement {
        let mut element = BimElement::new("IfcWall");
        element.guid = guid;
        element.name = name;
        element
    }

    /// Converter IfcSlab → BimElement
    pub fn convert_slab(
        guid: IfcGuid,
        name: Option<String>,
    ) -> BimElement {
        let mut element = BimElement::new("IfcSlab");
        element.guid = guid;
        element.name = name;
        element
    }

    /// Converter IfcColumn → BimElement
    pub fn convert_column(
        guid: IfcGuid,
        name: Option<String>,
    ) -> BimElement {
        let mut element = BimElement::new("IfcColumn");
        element.guid = guid;
        element.name = name;
        element
    }

    /// Converter IfcBeam → BimElement
    pub fn convert_beam(
        guid: IfcGuid,
        name: Option<String>,
    ) -> BimElement {
        let mut element = BimElement::new("IfcBeam");
        element.guid = guid;
        element.name = name;
        element
    }
}
