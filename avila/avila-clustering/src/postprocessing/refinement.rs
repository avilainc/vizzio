//! Refinamento de clusters: merge, split, boundary adjustment

use ndarray::{Array1, Array2, ArrayView2, Axis};
use std::collections::{HashMap, HashSet};

/// ClusterRefiner - refinador de clusters
pub struct ClusterRefiner {
    min_cluster_size: usize,
    merge_threshold: f64,
}

impl ClusterRefiner {
    pub fn new(min_cluster_size: usize, merge_threshold: f64) -> Self {
        Self {
            min_cluster_size,
            merge_threshold,
        }
    }

    /// Mescla clusters pequenos com o cluster mais próximo
    pub fn merge_small_clusters(
        &self,
        labels: &Array1<usize>,
        data: &ArrayView2<f64>,
    ) -> Array1<usize> {
        let mut refined_labels = labels.clone();
        let n_samples = labels.len();

        // Conta tamanho de cada cluster
        let mut cluster_sizes: HashMap<usize, usize> = HashMap::new();
        for &label in labels.iter() {
            *cluster_sizes.entry(label).or_insert(0) += 1;
        }

        // Identifica clusters pequenos
        let small_clusters: HashSet<usize> = cluster_sizes
            .iter()
            .filter(|(_, &size)| size < self.min_cluster_size)
            .map(|(&cluster, _)| cluster)
            .collect();

        if small_clusters.is_empty() {
            return refined_labels;
        }

        // Calcula centroides
        let centroids = self.calculate_centroids(data, labels);

        // Para cada cluster pequeno, encontra o mais próximo
        for &small_cluster in &small_clusters {
            let small_centroid = &centroids[&small_cluster];

            let mut min_distance = f64::INFINITY;
            let mut nearest_cluster = small_cluster;

            for (&cluster, centroid) in &centroids {
                if small_clusters.contains(&cluster) || cluster == small_cluster {
                    continue;
                }

                let distance = self.euclidean_distance(small_centroid, centroid);
                if distance < min_distance {
                    min_distance = distance;
                    nearest_cluster = cluster;
                }
            }

            // Reatribui pontos do cluster pequeno
            for i in 0..n_samples {
                if refined_labels[i] == small_cluster {
                    refined_labels[i] = nearest_cluster;
                }
            }
        }

        // Renumera clusters
        self.renumber_labels(&refined_labels)
    }

    /// Divide clusters grandes ou heterogêneos
    pub fn split_heterogeneous_clusters(
        &self,
        labels: &Array1<usize>,
        data: &ArrayView2<f64>,
        max_inertia: f64,
    ) -> Array1<usize> {
        let mut refined_labels = labels.clone();
        let n_clusters = labels.iter().max().unwrap_or(&0) + 1;
        let mut next_cluster_id = n_clusters;

        for cluster_id in 0..n_clusters {
            let cluster_points: Vec<usize> = labels
                .iter()
                .enumerate()
                .filter(|(_, &label)| label == cluster_id)
                .map(|(idx, _)| idx)
                .collect();

            if cluster_points.len() < 2 * self.min_cluster_size {
                continue;
            }

            // Calcula inércia do cluster
            let centroid = self.calculate_cluster_centroid(data, &cluster_points);
            let inertia: f64 = cluster_points
                .iter()
                .map(|&idx| {
                    let point = data.row(idx);
                    self.euclidean_distance(&point.to_vec(), &centroid).powi(2)
                })
                .sum();

            if inertia > max_inertia {
                // Split simples: divide em dois subgrupos baseado na primeira dimensão
                let mut first_half = Vec::new();
                let mut second_half = Vec::new();

                let median_idx = cluster_points.len() / 2;
                for (i, &idx) in cluster_points.iter().enumerate() {
                    if i < median_idx {
                        first_half.push(idx);
                    } else {
                        second_half.push(idx);
                        refined_labels[idx] = next_cluster_id;
                    }
                }

                next_cluster_id += 1;
            }
        }

        self.renumber_labels(&refined_labels)
    }

    /// Ajusta limites de clusters
    pub fn refine_boundaries(
        &self,
        labels: &Array1<usize>,
        data: &ArrayView2<f64>,
        n_iterations: usize,
    ) -> Array1<usize> {
        let mut refined_labels = labels.clone();

        for _ in 0..n_iterations {
            let centroids = self.calculate_centroids(data, &refined_labels);

            // Reatribui cada ponto ao centroide mais próximo
            for (i, point) in data.axis_iter(Axis(0)).enumerate() {
                let point_vec = point.to_vec();

                let mut min_distance = f64::INFINITY;
                let mut nearest_cluster = refined_labels[i];

                for (&cluster, centroid) in &centroids {
                    let distance = self.euclidean_distance(&point_vec, centroid);
                    if distance < min_distance {
                        min_distance = distance;
                        nearest_cluster = cluster;
                    }
                }

                refined_labels[i] = nearest_cluster;
            }
        }

        refined_labels
    }

    // Métodos auxiliares

    fn calculate_centroids(
        &self,
        data: &ArrayView2<f64>,
        labels: &Array1<usize>,
    ) -> HashMap<usize, Vec<f64>> {
        let mut centroids: HashMap<usize, Vec<f64>> = HashMap::new();
        let mut counts: HashMap<usize, usize> = HashMap::new();

        let n_features = data.ncols();

        for (i, &label) in labels.iter().enumerate() {
            let centroid = centroids.entry(label).or_insert_with(|| vec![0.0; n_features]);
            let count = counts.entry(label).or_insert(0);

            for (j, &val) in data.row(i).iter().enumerate() {
                centroid[j] += val;
            }
            *count += 1;
        }

        for (label, centroid) in centroids.iter_mut() {
            let count = counts[label] as f64;
            for val in centroid.iter_mut() {
                *val /= count;
            }
        }

        centroids
    }

    fn calculate_cluster_centroid(&self, data: &ArrayView2<f64>, indices: &[usize]) -> Vec<f64> {
        let n_features = data.ncols();
        let mut centroid = vec![0.0; n_features];

        for &idx in indices {
            for (j, &val) in data.row(idx).iter().enumerate() {
                centroid[j] += val;
            }
        }

        for val in &mut centroid {
            *val /= indices.len() as f64;
        }

        centroid
    }

    fn euclidean_distance(&self, a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    fn renumber_labels(&self, labels: &Array1<usize>) -> Array1<usize> {
        let mut unique_labels: Vec<usize> = labels.iter().copied().collect();
        unique_labels.sort_unstable();
        unique_labels.dedup();

        let label_map: HashMap<usize, usize> = unique_labels
            .into_iter()
            .enumerate()
            .map(|(new, old)| (old, new))
            .collect();

        labels.mapv(|old| label_map[&old])
    }
}

impl Default for ClusterRefiner {
    fn default() -> Self {
        Self::new(5, 0.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_merge_small_clusters() {
        let labels = array![0, 0, 1, 1, 2]; // Cluster 2 tem apenas 1 elemento
        let data = array![
            [0.0, 0.0],
            [0.1, 0.1],
            [5.0, 5.0],
            [5.1, 5.1],
            [0.2, 0.2], // Próximo ao cluster 0
        ];

        let refiner = ClusterRefiner::new(2, 0.5);
        let refined = refiner.merge_small_clusters(&labels, &data.view());

        assert_eq!(refined[4], refined[0]); // Cluster pequeno foi mesclado
    }
}
