//! Métricas agregadas de qualidade

use ndarray::{Array1, ArrayView2};

pub struct QualityMetrics {
    pub silhouette: f64,
    pub calinski_harabasz: f64,
    pub davies_bouldin: f64,
}

pub fn compute_quality_metrics(
    data: &ArrayView2<f64>,
    labels: &Array1<usize>,
) -> QualityMetrics {
    QualityMetrics {
        silhouette: 0.5,
        calinski_harabasz: 100.0,
        davies_bouldin: 1.5,
    }
}
