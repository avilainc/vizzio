//! # avila-geometry
//!
//! Advanced geometry processing (NURBS, CSG, etc.)

pub mod nurbs;
pub mod brep;
pub mod tesselation;
pub mod boolean;

pub use nurbs::NurbsCurve;
pub use brep::{BRepTopology, BRepBuilder};
pub use tesselation::Tesselator;
pub use boolean::BooleanOperations;
