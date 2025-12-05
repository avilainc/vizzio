//! # avila-ifc
//!
//! Parser IFC nativo em Rust.
//! Suporta IFC2x3, IFC4 e IFC4.3 (ISO 10303-21 STEP).

pub mod parser;
pub mod schema;
pub mod entities;
pub mod geometry_converter;
pub mod property_extractor;

pub use parser::{IfcParser, IfcParserError};
pub use schema::{IfcSchema, IfcVersion};
