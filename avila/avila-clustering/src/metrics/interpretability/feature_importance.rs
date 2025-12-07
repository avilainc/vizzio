//! Importância de features por cluster

use ndarray::{Array1, ArrayView2, Axis};

pub fn calculate_feature_importance(
    data: &ArrayView2<f64>,
    labels: &Array1<usize>,
) -> Vec<Array1<f64>> {
    let n_clusters = labels.iter().max().unwrap_or(&0) + 1;
    let n_features = data.ncols();
    let global_mean = data.mean_axis(Axis(0)).unwrap();

    (0..n_clusters)
        .map(|cluster_id| {
            let cluster_points: Vec<usize> = labels
                .iter()
                .enumerate()
                .filter(|(_, &l)| l == cluster_id)
                .map(|(i, _)| i)
                .collect();

            let mut importance = Array1::zeros(n_features);
            if !cluster_points.is_empty() {
                for j in 0..n_features {
                    let cluster_mean: f64 = cluster_points
                        .iter()
                        .map(|&i| data[[i, j]])
                        .sum::<f64>()
                        / cluster_points.len() as f64;
                    importance[j] = (cluster_mean - global_mean[j]).abs();
                }
            }
            importance
        })
        .collect()
}
