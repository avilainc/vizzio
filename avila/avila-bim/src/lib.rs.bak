//! Main library module - Re-export all submodules

pub mod bim_core;
pub mod bim_converter;

// Expanded modules
pub mod ifc;
pub mod gltf;
pub mod db;
pub mod spatial;
pub mod geometry;
pub mod cache;
pub mod validation;

// File parsers
pub mod file_parsers;
pub mod ifc_parser;
pub mod dwg_parser;
pub mod dxf_parser;
pub mod obj_parser;
pub mod stl_parser;
pub mod ply_parser;
pub mod fbx_parser;
pub mod gltf_parser;
pub mod skp_parser;
pub mod rvt_parser;
pub mod nwd_parser;

// Example usage
pub mod example_usage;
pub mod test_dwg;

// Pure Rust utilities
pub mod math;
pub mod mesh_gen;
pub mod mesh_optimizer;
pub mod hash;
pub mod step_tokenizer;
pub mod triangulation;
pub mod intersection;
pub mod convex_hull;
pub mod kdtree;
pub mod curve;
pub mod transform;
pub mod bvh;
pub mod octree;
pub mod polygon_ops;
pub mod clipper;

// Re-export core types
pub use bim_core::*;
pub use bim_converter::*;

// Re-export commonly used types
pub use ifc::{IfcParser, IfcParserError};
pub use gltf::{GltfExporter, ExportOptions};
pub use spatial::{BoundingVolumeHierarchy, Octree, Raycast, CollisionDetector};
pub use geometry::{NurbsCurve, BRepTopology, BRepBuilder, Tesselator};
pub use cache::{GeometryCache, MaterialCache};
pub use validation::{IfcValidator, GeometryValidator};

// Re-export file parser types
pub use file_parsers::{ParserManager, LoadedModel, ModelElement, ElementGeometry, FileFormat, FileParser, ParseError};
pub use dwg_parser::DwgFileParser;
