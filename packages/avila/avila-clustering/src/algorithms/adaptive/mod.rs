//! Algoritmos adaptativos de clustering
//!
//! Algoritmos que se auto-ajustam e otimizam parâmetros automaticamente.

pub mod auto_cluster;
pub mod parameter_tuning;
pub mod incremental;
pub mod transfer;

pub use auto_cluster::*;
pub use parameter_tuning::*;
pub use incremental::*;
pub use transfer::*;
