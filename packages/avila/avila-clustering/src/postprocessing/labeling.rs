//! Rotulagem automática e nomeação de clusters

use ndarray::{Array1, ArrayView2, Axis};
use std::collections::HashMap;

/// ClusterLabeler - gera rótulos descritivos para clusters
pub struct ClusterLabeler {
    feature_names: Vec<String>,
}

impl ClusterLabeler {
    pub fn new(feature_names: Vec<String>) -> Self {
        Self { feature_names }
    }

    /// Gera perfil estatístico de cada cluster
    pub fn generate_cluster_profiles(
        &self,
        data: &ArrayView2<f64>,
        labels: &Array1<usize>,
    ) -> HashMap<usize, ClusterProfile> {
        let n_clusters = labels.iter().max().unwrap_or(&0) + 1;
        let mut profiles = HashMap::new();

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

            let profile = self.compute_profile(data, &cluster_indices);
            profiles.insert(cluster_id, profile);
        }

        profiles
    }

    /// Gera nomes descritivos baseados em características dominantes
    pub fn generate_cluster_names(
        &self,
        data: &ArrayView2<f64>,
        labels: &Array1<usize>,
    ) -> HashMap<usize, String> {
        let profiles = self.generate_cluster_profiles(data, labels);
        let mut names = HashMap::new();

        for (cluster_id, profile) in profiles {
            let name = self.create_descriptive_name(&profile);
            names.insert(cluster_id, name);
        }

        names
    }

    /// Identifica features mais discriminativas de cada cluster
    pub fn identify_distinctive_features(
        &self,
        data: &ArrayView2<f64>,
        labels: &Array1<usize>,
        top_n: usize,
    ) -> HashMap<usize, Vec<(String, f64)>> {
        let n_clusters = labels.iter().max().unwrap_or(&0) + 1;
        let n_features = data.ncols();

        // Calcula média global
        let global_means = data.mean_axis(Axis(0)).unwrap();

        let mut distinctive_features = HashMap::new();

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

            // Calcula média do cluster
            let mut cluster_means = vec![0.0; n_features];
            for &idx in &cluster_indices {
                for (j, &val) in data.row(idx).iter().enumerate() {
                    cluster_means[j] += val;
                }
            }
            for mean in &mut cluster_means {
                *mean /= cluster_indices.len() as f64;
            }

            // Calcula diferenças normalizadas
            let mut feature_scores: Vec<(String, f64)> = (0..n_features)
                .map(|i| {
                    let diff = (cluster_means[i] - global_means[i]).abs();
                    let name = self.feature_names.get(i)
                        .cloned()
                        .unwrap_or_else(|| format!("feature_{}", i));
                    (name, diff)
                })
                .collect();

            // Ordena por diferença (mais distintivo)
            feature_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            feature_scores.truncate(top_n);

            distinctive_features.insert(cluster_id, feature_scores);
        }

        distinctive_features
    }

    // Métodos auxiliares

    fn compute_profile(&self, data: &ArrayView2<f64>, indices: &[usize]) -> ClusterProfile {
        let n_features = data.ncols();
        let n_samples = indices.len();

        let mut means = vec![0.0; n_features];
        let mut mins = vec![f64::INFINITY; n_features];
        let mut maxs = vec![f64::NEG_INFINITY; n_features];

        for &idx in indices {
            for (j, &val) in data.row(idx).iter().enumerate() {
                means[j] += val;
                mins[j] = mins[j].min(val);
                maxs[j] = maxs[j].max(val);
            }
        }

        for mean in &mut means {
            *mean /= n_samples as f64;
        }

        // Calcula desvios padrão
        let mut stds = vec![0.0; n_features];
        for &idx in indices {
            for (j, &val) in data.row(idx).iter().enumerate() {
                stds[j] += (val - means[j]).powi(2);
            }
        }
        for std in &mut stds {
            *std = (*std / n_samples as f64).sqrt();
        }

        ClusterProfile {
            size: n_samples,
            means,
            stds,
            mins,
            maxs,
        }
    }

    fn create_descriptive_name(&self, profile: &ClusterProfile) -> String {
        // Encontra as 2 features com maior variação
        let mut feature_variations: Vec<(usize, f64)> = profile
            .stds
            .iter()
            .enumerate()
            .map(|(i, &std)| (i, std))
            .collect();

        feature_variations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let top_features: Vec<String> = feature_variations
            .iter()
            .take(2)
            .map(|(i, _)| {
                self.feature_names
                    .get(*i)
                    .cloned()
                    .unwrap_or_else(|| format!("f{}", i))
            })
            .collect();

        if top_features.len() >= 2 {
            format!("Cluster({}, {})", top_features[0], top_features[1])
        } else if top_features.len() == 1 {
            format!("Cluster({})", top_features[0])
        } else {
            format!("Cluster(size={})", profile.size)
        }
    }
}

/// Perfil estatístico de um cluster
#[derive(Debug, Clone)]
pub struct ClusterProfile {
    pub size: usize,
    pub means: Vec<f64>,
    pub stds: Vec<f64>,
    pub mins: Vec<f64>,
    pub maxs: Vec<f64>,
}

impl ClusterProfile {
    /// Retorna sumário textual do perfil
    pub fn summary(&self, feature_names: &[String]) -> String {
        let mut lines = vec![format!("Tamanho: {} amostras", self.size)];

        for (i, name) in feature_names.iter().enumerate() {
            if i < self.means.len() {
                lines.push(format!(
                    "  {}: {:.3} ± {:.3} (min: {:.3}, max: {:.3})",
                    name, self.means[i], self.stds[i], self.mins[i], self.maxs[i]
                ));
            }
        }

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_cluster_profiling() {
        let data = array![
            [1.0, 10.0],
            [2.0, 11.0],
            [10.0, 1.0],
            [11.0, 2.0],
        ];
        let labels = array![0, 0, 1, 1];

        let labeler = ClusterLabeler::new(vec!["x".to_string(), "y".to_string()]);
        let profiles = labeler.generate_cluster_profiles(&data.view(), &labels);

        assert_eq!(profiles.len(), 2);
        assert_eq!(profiles[&0].size, 2);
        assert_eq!(profiles[&1].size, 2);
    }
}
