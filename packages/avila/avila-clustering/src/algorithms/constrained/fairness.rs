//! Fairness-aware clustering

use ndarray::{Array1, ArrayView2};

pub struct FairClusterer {
    n_clusters: usize,
    sensitive_features: Vec<usize>,
    fairness_metric: FairnessMetric,
}

#[derive(Debug, Clone, Copy)]
pub enum FairnessMetric {
    DemographicParity,
    EqualizedOdds,
    Balance,
}

impl FairClusterer {
    pub fn new(n_clusters: usize, sensitive_features: Vec<usize>) -> Self {
        Self {
            n_clusters,
            sensitive_features,
            fairness_metric: FairnessMetric::DemographicParity,
        }
    }

    pub fn with_metric(mut self, metric: FairnessMetric) -> Self {
        self.fairness_metric = metric;
        self
    }

    pub fn fit(&mut self, data: &ArrayView2<f64>) -> Result<Array1<usize>, String> {
        Ok(Array1::zeros(data.nrows()))
    }
}
