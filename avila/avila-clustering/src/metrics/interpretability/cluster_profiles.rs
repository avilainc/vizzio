//! Perfis de características de clusters

use ndarray::{Array1, ArrayView2};
use std::collections::HashMap;

pub struct ClusterCharacteristics {
    pub size: usize,
    pub density: f64,
    pub cohesion: f64,
}

pub fn compute_cluster_characteristics(
    data: &ArrayView2<f64>,
    labels: &Array1<usize>,
) -> HashMap<usize, ClusterCharacteristics> {
    let n_clusters = labels.iter().max().unwrap_or(&0) + 1;
    let mut characteristics = HashMap::new();

    for cluster_id in 0..n_clusters {
        let size = labels.iter().filter(|&&l| l == cluster_id).count();
        characteristics.insert(
            cluster_id,
            ClusterCharacteristics {
                size,
                density: 1.0,
                cohesion: 1.0,
            },
        );
    }

    characteristics
}
