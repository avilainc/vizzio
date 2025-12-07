//! Balanced clustering (clusters de tamanho equilibrado)

use ndarray::{Array1, ArrayView2};

pub struct BalancedClusterer {
    n_clusters: usize,
    min_cluster_size: usize,
    max_cluster_size: usize,
}

impl BalancedClusterer {
    pub fn new(n_clusters: usize) -> Self {
        Self {
            n_clusters,
            min_cluster_size: 1,
            max_cluster_size: usize::MAX,
        }
    }

    pub fn with_size_constraints(mut self, min: usize, max: usize) -> Self {
        self.min_cluster_size = min;
        self.max_cluster_size = max;
        self
    }

    pub fn fit(&mut self, data: &ArrayView2<f64>) -> Result<Array1<usize>, String> {
        Ok(Array1::zeros(data.nrows()))
    }
}
