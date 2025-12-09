//! Avila IFC - High-performance IFC parser and processor in Rust
//!
//! This library provides a complete implementation for reading and processing
//! IFC (Industry Foundation Classes) files, similar to ifcopenshell but written in pure Rust.
//!
//! # Features
//!
//! - Fast STEP/ISO-10303-21 parser
//! - Support for IFC2X3 and IFC4 schemas
//! - Entity queries and filtering
//! - Geometry processing (points, directions, transformations)
//! - Spatial structure navigation
//! - Type-safe entity access
//!
//! # Example
//!
//! ```rust,no_run
//! use avila_ifc::IfcFile;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Open an IFC file
//! let ifc_file = IfcFile::open("building.ifc")?;
//!
//! // Get file statistics
//! let stats = ifc_file.statistics();
//! println!("{}", stats);
//!
//! // Get all walls
//! let walls = ifc_file.get_entities_by_type("IFCWALL");
//! println!("Found {} walls", walls.len());
//!
//! // Query entities
//! use avila_ifc::IfcQuery;
//! let columns = IfcQuery::new(&ifc_file)
//!     .entity_type("IFCCOLUMN")
//!     .name_contains("C1")
//!     .execute();
//!
//! // Get spatial structure
//! let project = ifc_file.get_project();
//! let buildings = ifc_file.get_buildings();
//! let storeys = ifc_file.get_building_storeys();
//! # Ok(())
//! # }
//! ```

pub mod entities;
pub mod error;
pub mod file;
pub mod geometry;
pub mod query;
pub mod step_parser;

// Re-export commonly used types
pub use entities::{IfcEntity, EntityTypeRegistry};
pub use error::{IfcError, Result};
pub use file::{IfcFile, FileStatistics};
pub use geometry::{
    BoundingBox, Direction3D, GeometryProcessor, Point3D, Transform3D,
};
pub use query::{IfcQuery, SpatialQuery};
pub use step_parser::{StepEntity, StepFile, StepParser, StepValue};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
