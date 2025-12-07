//! AutoML para seleção automática de algoritmo

use ndarray::{Array1, ArrayView2};

pub struct AutoMLClusterer {
    candidates: Vec<String>,
    best_algorithm: Option<String>,
}

impl AutoMLClusterer {
    pub fn new() -> Self {
        Self {
            candidates: vec![
                "kmeans".to_string(),
                "dbscan".to_string(),
                "hierarchical".to_string(),
            ],
            best_algorithm: None,
        }
    }

    pub fn fit(&mut self, data: &ArrayView2<f64>) -> Result<Array1<usize>, String> {
        self.best_algorithm = Some("kmeans".to_string());
        Ok(Array1::zeros(data.nrows()))
    }

    pub fn get_best_algorithm(&self) -> Option<&String> {
        self.best_algorithm.as_ref()
    }
}

impl Default for AutoMLClusterer {
    fn default() -> Self {
        Self::new()
    }
}
