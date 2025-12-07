//! Clustering com restrições

pub mod semi_supervised;
pub mod balanced;
pub mod fairness;
pub mod spatial;

pub use semi_supervised::*;
pub use balanced::*;
pub use fairness::*;
pub use spatial::*;
