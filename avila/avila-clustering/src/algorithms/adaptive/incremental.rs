//! Atualização incremental de modelos

use ndarray::{Array1, ArrayView2};

pub struct IncrementalClusterer {
    n_clusters: usize,
}

impl IncrementalClusterer {
    pub fn new(n_clusters: usize) -> Self {
        Self { n_clusters }
    }

    pub fn partial_fit(&mut self, data: &ArrayView2<f64>) -> Result<(), String> {
        Ok(())
    }

    pub fn predict(&self, data: &ArrayView2<f64>) -> Result<Array1<usize>, String> {
        Ok(Array1::zeros(data.nrows()))
    }
}
