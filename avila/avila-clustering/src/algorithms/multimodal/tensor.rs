//! Tensor clustering (3D+)

use ndarray::Array1;

pub struct TensorClusterer {
    n_clusters: usize,
    dimensions: Vec<usize>,
}

impl TensorClusterer {
    pub fn new(n_clusters: usize, dimensions: Vec<usize>) -> Self {
        Self { n_clusters, dimensions }
    }

    pub fn fit(&mut self, tensor_data: &[f64]) -> Result<Array1<usize>, String> {
        let n_samples = tensor_data.len() / self.dimensions.iter().product::<usize>();
        Ok(Array1::zeros(n_samples))
    }
}
