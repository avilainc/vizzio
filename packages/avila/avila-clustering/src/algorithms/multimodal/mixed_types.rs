//! K-prototypes para dados mistos (numérico + categórico)

use ndarray::{Array1, ArrayView2};

pub struct KPrototypes {
    n_clusters: usize,
    categorical_indices: Vec<usize>,
}

impl KPrototypes {
    pub fn new(n_clusters: usize, categorical_indices: Vec<usize>) -> Self {
        Self {
            n_clusters,
            categorical_indices,
        }
    }

    pub fn fit(&mut self, data: &ArrayView2<f64>) -> Result<Array1<usize>, String> {
        Ok(Array1::zeros(data.nrows()))
    }
}
