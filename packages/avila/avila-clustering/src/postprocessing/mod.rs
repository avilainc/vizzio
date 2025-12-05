//! Pós-processamento de resultados de clustering
//!
//! Refinamento, análise e explicação de clusters após a execução dos algoritmos.

pub mod refinement;
pub mod labeling;
pub mod explanation;
pub mod stability;

pub use refinement::*;
pub use labeling::*;
pub use explanation::*;
pub use stability::*;

use ndarray::Array1;

/// Resultado de clustering com metadados
pub struct ClusteringResult {
    pub labels: Array1<usize>,
    pub n_clusters: usize,
    pub confidence: Option<Array1<f64>>,
}
