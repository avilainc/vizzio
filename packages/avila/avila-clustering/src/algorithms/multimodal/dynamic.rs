//! Dynamic clustering (evolução temporal)

use ndarray::{Array1, ArrayView2};

pub struct DynamicClusterer {
    n_clusters: usize,
    history: Vec<Array1<usize>>,
}

impl DynamicClusterer {
    pub fn new(n_clusters: usize) -> Self {
        Self {
            n_clusters,
            history: Vec::new(),
        }
    }

    pub fn fit_timestep(&mut self, data: &ArrayView2<f64>) -> Result<Array1<usize>, String> {
        let labels = Array1::zeros(data.nrows());
        self.history.push(labels.clone());
        Ok(labels)
    }

    pub fn get_history(&self) -> &[Array1<usize>] {
        &self.history
    }
}
