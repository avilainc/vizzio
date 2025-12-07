//! Semi-supervised clustering com restrições must-link/cannot-link

use ndarray::{Array1, ArrayView2};

pub struct SemiSupervisedClusterer {
    n_clusters: usize,
    must_link: Vec<(usize, usize)>,
    cannot_link: Vec<(usize, usize)>,
}

impl SemiSupervisedClusterer {
    pub fn new(n_clusters: usize) -> Self {
        Self {
            n_clusters,
            must_link: Vec::new(),
            cannot_link: Vec::new(),
        }
    }

    pub fn add_must_link(&mut self, i: usize, j: usize) {
        self.must_link.push((i, j));
    }

    pub fn add_cannot_link(&mut self, i: usize, j: usize) {
        self.cannot_link.push((i, j));
    }

    pub fn fit(&mut self, data: &ArrayView2<f64>) -> Result<Array1<usize>, String> {
        Ok(Array1::zeros(data.nrows()))
    }
}
