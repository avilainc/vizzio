//! Spatial clustering com restrições geoespaciais

use ndarray::{Array1, ArrayView2};

pub struct SpatialClusterer {
    n_clusters: usize,
    max_distance: f64,
    coordinate_indices: (usize, usize),
}

impl SpatialClusterer {
    pub fn new(n_clusters: usize, max_distance: f64) -> Self {
        Self {
            n_clusters,
            max_distance,
            coordinate_indices: (0, 1),
        }
    }

    pub fn with_coordinate_indices(mut self, x_idx: usize, y_idx: usize) -> Self {
        self.coordinate_indices = (x_idx, y_idx);
        self
    }

    pub fn fit(&mut self, data: &ArrayView2<f64>) -> Result<Array1<usize>, String> {
        Ok(Array1::zeros(data.nrows()))
    }
}
