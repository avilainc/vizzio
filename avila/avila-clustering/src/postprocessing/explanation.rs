//! Explicação e interpretabilidade de clusters

use ndarray::{Array1, Array2, ArrayView2, Axis};
use std::collections::HashMap;

/// ClusterExplainer - fornece explicações sobre decisões de clustering
pub struct ClusterExplainer;

impl ClusterExplainer {
    pub fn new() -> Self {
        Self
    }

    /// Calcula importância de features para cada cluster (SHAP-like)
    pub fn feature_importance(
        &self,
        data: &ArrayView2<f64>,
        labels: &Array1<usize>,
    ) -> HashMap<usize, Array1<f64>> {
        let n_clusters = labels.iter().max().unwrap_or(&0) + 1;
        let n_features = data.ncols();

        // Calcula centroide global
        let global_centroid = data.mean_axis(Axis(0)).unwrap();

        let mut importance_by_cluster = HashMap::new();

        for cluster_id in 0..n_clusters {
            let cluster_indices: Vec<usize> = labels
                .iter()
                .enumerate()
                .filter(|(_, &label)| label == cluster_id)
                .map(|(idx, _)| idx)
                .collect();

            if cluster_indices.is_empty() {
                continue;
            }

            // Calcula centroide do cluster
            let mut cluster_centroid = Array1::zeros(n_features);
            for &idx in &cluster_indices {
                cluster_centroid += &data.row(idx);
            }
            cluster_centroid /= cluster_indices.len() as f64;

            // Importância = diferença normalizada do centroide global
            let mut importance = Array1::zeros(n_features);
            for i in 0..n_features {
                importance[i] = (cluster_centroid[i] - global_centroid[i]).abs();
            }

            // Normaliza para soma = 1
            let total: f64 = importance.iter().sum();
            if total > 1e-10 {
                importance /= total;
            }

            importance_by_cluster.insert(cluster_id, importance);
        }

        importance_by_cluster
    }

    /// Explica por que um ponto pertence a um cluster específico
    pub fn explain_assignment(
        &self,
        point: &ArrayView2<f64>,
        data: &ArrayView2<f64>,
        labels: &Array1<usize>,
        point_label: usize,
        feature_names: &[String],
    ) -> ClusterAssignmentExplanation {
        let n_clusters = labels.iter().max().unwrap_or(&0) + 1;
        let n_features = data.ncols();

        // Calcula centroides
        let mut centroids = vec![Array1::zeros(n_features); n_clusters];
        let mut counts = vec![0; n_clusters];

        for (i, &label) in labels.iter().enumerate() {
            centroids[label] += &data.row(i);
            counts[label] += 1;
        }

        for (centroid, &count) in centroids.iter_mut().zip(counts.iter()) {
            if count > 0 {
                *centroid /= count as f64;
            }
        }

        // Calcula distâncias para cada centroide
        let point_vec = point.row(0);
        let mut distances = Vec::new();

        for (cluster_id, centroid) in centroids.iter().enumerate() {
            let dist = point_vec
                .iter()
                .zip(centroid.iter())
                .map(|(p, c)| (p - c).powi(2))
                .sum::<f64>()
                .sqrt();
            distances.push((cluster_id, dist));
        }

        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Identifica features mais contributivas
        let assigned_centroid = &centroids[point_label];
        let mut feature_contributions: Vec<(String, f64)> = (0..n_features)
            .map(|i| {
                let contribution = (point_vec[i] - assigned_centroid[i]).abs();
                let name = feature_names.get(i)
                    .cloned()
                    .unwrap_or_else(|| format!("feature_{}", i));
                (name, contribution)
            })
            .collect();

        feature_contributions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        ClusterAssignmentExplanation {
            assigned_cluster: point_label,
            distance_to_assigned: distances[0].1,
            alternative_clusters: distances[1..].iter().take(3).cloned().collect(),
            top_features: feature_contributions.into_iter().take(5).collect(),
        }
    }

    /// Analisa separabilidade entre clusters
    pub fn cluster_separation_analysis(
        &self,
        data: &ArrayView2<f64>,
        labels: &Array1<usize>,
    ) -> SeparationAnalysis {
        let n_clusters = labels.iter().max().unwrap_or(&0) + 1;
        let n_features = data.ncols();

        // Calcula centroides
        let mut centroids = vec![Array1::zeros(n_features); n_clusters];
        let mut counts = vec![0; n_clusters];

        for (i, &label) in labels.iter().enumerate() {
            centroids[label] += &data.row(i);
            counts[label] += 1;
        }

        for (centroid, &count) in centroids.iter_mut().zip(counts.iter()) {
            if count > 0 {
                *centroid /= count as f64;
            }
        }

        // Calcula distâncias inter-cluster
        let mut inter_cluster_distances = Vec::new();
        for i in 0..n_clusters {
            for j in (i + 1)..n_clusters {
                let dist = centroids[i]
                    .iter()
                    .zip(centroids[j].iter())
                    .map(|(a, b)| (a - b).powi(2))
                    .sum::<f64>()
                    .sqrt();
                inter_cluster_distances.push(((i, j), dist));
            }
        }

        // Calcula inércia intra-cluster
        let mut intra_cluster_inertias = vec![0.0; n_clusters];
        for (i, &label) in labels.iter().enumerate() {
            let point = data.row(i);
            let centroid = &centroids[label];
            let dist_sq: f64 = point
                .iter()
                .zip(centroid.iter())
                .map(|(p, c)| (p - c).powi(2))
                .sum();
            intra_cluster_inertias[label] += dist_sq;
        }

        SeparationAnalysis {
            n_clusters,
            inter_cluster_distances,
            intra_cluster_inertias,
        }
    }
}

impl Default for ClusterExplainer {
    fn default() -> Self {
        Self::new()
    }
}

/// Explicação da atribuição de um ponto a um cluster
#[derive(Debug)]
pub struct ClusterAssignmentExplanation {
    pub assigned_cluster: usize,
    pub distance_to_assigned: f64,
    pub alternative_clusters: Vec<(usize, f64)>,
    pub top_features: Vec<(String, f64)>,
}

impl ClusterAssignmentExplanation {
    pub fn to_string(&self) -> String {
        let mut lines = vec![
            format!("Ponto atribuído ao Cluster {}", self.assigned_cluster),
            format!("Distância ao centroide: {:.4}", self.distance_to_assigned),
            String::from("\nClusters alternativos:"),
        ];

        for (cluster_id, dist) in &self.alternative_clusters {
            lines.push(format!("  Cluster {}: distância {:.4}", cluster_id, dist));
        }

        lines.push(String::from("\nFeatures mais contributivas:"));
        for (name, contrib) in &self.top_features {
            lines.push(format!("  {}: {:.4}", name, contrib));
        }

        lines.join("\n")
    }
}

/// Análise de separabilidade entre clusters
#[derive(Debug)]
pub struct SeparationAnalysis {
    pub n_clusters: usize,
    pub inter_cluster_distances: Vec<((usize, usize), f64)>,
    pub intra_cluster_inertias: Vec<f64>,
}

impl SeparationAnalysis {
    pub fn average_inter_distance(&self) -> f64 {
        if self.inter_cluster_distances.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.inter_cluster_distances.iter().map(|(_, d)| d).sum();
        sum / self.inter_cluster_distances.len() as f64
    }

    pub fn average_intra_inertia(&self) -> f64 {
        if self.intra_cluster_inertias.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.intra_cluster_inertias.iter().sum();
        sum / self.intra_cluster_inertias.len() as f64
    }

    pub fn silhouette_coefficient(&self) -> f64 {
        let inter = self.average_inter_distance();
        let intra = self.average_intra_inertia();

        if inter + intra == 0.0 {
            return 0.0;
        }

        (inter - intra) / (inter + intra).max(1e-10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_feature_importance() {
        let data = array![
            [1.0, 10.0],
            [2.0, 11.0],
            [10.0, 1.0],
            [11.0, 2.0],
        ];
        let labels = array![0, 0, 1, 1];

        let explainer = ClusterExplainer::new();
        let importance = explainer.feature_importance(&data.view(), &labels);

        assert_eq!(importance.len(), 2);
    }
}
