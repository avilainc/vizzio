//! Transfer learning entre datasets

use ndarray::{Array1, ArrayView2};

pub struct TransferClusterer {
    source_model: Option<Vec<f64>>,
}

impl TransferClusterer {
    pub fn new() -> Self {
        Self { source_model: None }
    }

    pub fn fit_source(&mut self, data: &ArrayView2<f64>) -> Result<(), String> {
        Ok(())
    }

    pub fn transfer_to_target(&self, data: &ArrayView2<f64>) -> Result<Array1<usize>, String> {
        Ok(Array1::zeros(data.nrows()))
    }
}

impl Default for TransferClusterer {
    fn default() -> Self {
        Self::new()
    }
}
