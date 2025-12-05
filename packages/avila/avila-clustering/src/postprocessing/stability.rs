//! Análise de estabilidade temporal de clusters

use ndarray::{Array1, Array2, ArrayView2};
use std::collections::HashMap;

/// StabilityAnalyzer - analisa estabilidade de clusters ao longo do tempo
pub struct StabilityAnalyzer {
    history: Vec<ClusterSnapshot>,
}

impl StabilityAnalyzer {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    /// Adiciona um snapshot de clustering
    pub fn add_snapshot(&mut self, labels: Array1<usize>, timestamp: u64) {
        self.history.push(ClusterSnapshot { labels, timestamp });
    }

    /// Calcula estabilidade entre dois snapshots consecutivos
    pub fn stability_between_snapshots(&self, idx1: usize, idx2: usize) -> Result<f64, String> {
        if idx1 >= self.history.len() || idx2 >= self.history.len() {
            return Err("Índice de snapshot inválido".to_string());
        }

        let labels1 = &self.history[idx1].labels;
        let labels2 = &self.history[idx2].labels;

        if labels1.len() != labels2.len() {
            return Err("Snapshots têm tamanhos diferentes".to_string());
        }

        Ok(self.adjusted_rand_index(labels1, labels2))
    }

    /// Calcula estabilidade média ao longo de toda a história
    pub fn overall_stability(&self) -> Result<f64, String> {
        if self.history.len() < 2 {
            return Err("Histórico insuficiente para análise".to_string());
        }

        let mut total_stability = 0.0;
        let mut count = 0;

        for i in 0..(self.history.len() - 1) {
            let stability = self.stability_between_snapshots(i, i + 1)?;
            total_stability += stability;
            count += 1;
        }

        Ok(total_stability / count as f64)
    }

    /// Identifica pontos que mudaram de cluster
    pub fn identify_cluster_transitions(
        &self,
        idx1: usize,
        idx2: usize,
    ) -> Result<Vec<usize>, String> {
        if idx1 >= self.history.len() || idx2 >= self.history.len() {
            return Err("Índice de snapshot inválido".to_string());
        }

        let labels1 = &self.history[idx1].labels;
        let labels2 = &self.history[idx2].labels;

        let transitions: Vec<usize> = labels1
            .iter()
            .zip(labels2.iter())
            .enumerate()
            .filter(|(_, (&l1, &l2))| l1 != l2)
            .map(|(idx, _)| idx)
            .collect();

        Ok(transitions)
    }

    /// Calcula matriz de transição entre clusters
    pub fn transition_matrix(&self, idx1: usize, idx2: usize) -> Result<Array2<usize>, String> {
        if idx1 >= self.history.len() || idx2 >= self.history.len() {
            return Err("Índice de snapshot inválido".to_string());
        }

        let labels1 = &self.history[idx1].labels;
        let labels2 = &self.history[idx2].labels;

        let n_clusters1 = labels1.iter().max().unwrap_or(&0) + 1;
        let n_clusters2 = labels2.iter().max().unwrap_or(&0) + 1;

        let mut matrix = Array2::zeros((n_clusters1, n_clusters2));

        for (&from_cluster, &to_cluster) in labels1.iter().zip(labels2.iter()) {
            matrix[[from_cluster, to_cluster]] += 1;
        }

        Ok(matrix)
    }

    /// Identifica clusters mais estáveis
    pub fn most_stable_clusters(&self, n_top: usize) -> Result<Vec<(usize, f64)>, String> {
        if self.history.len() < 2 {
            return Err("Histórico insuficiente".to_string());
        }

        let last_snapshot = &self.history[self.history.len() - 1].labels;
        let n_clusters = last_snapshot.iter().max().unwrap_or(&0) + 1;

        let mut cluster_stabilities = Vec::new();

        for cluster_id in 0..n_clusters {
            let mut stability_sum = 0.0;
            let mut count = 0;

            for i in 0..(self.history.len() - 1) {
                let labels1 = &self.history[i].labels;
                let labels2 = &self.history[i + 1].labels;

                // Pontos que estavam no cluster em t1
                let cluster_points: Vec<usize> = labels1
                    .iter()
                    .enumerate()
                    .filter(|(_, &l)| l == cluster_id)
                    .map(|(idx, _)| idx)
                    .collect();

                if !cluster_points.is_empty() {
                    // Quantos permaneceram no mesmo cluster em t2?
                    let retained = cluster_points
                        .iter()
                        .filter(|&&idx| labels2[idx] == cluster_id)
                        .count();

                    stability_sum += retained as f64 / cluster_points.len() as f64;
                    count += 1;
                }
            }

            if count > 0 {
                cluster_stabilities.push((cluster_id, stability_sum / count as f64));
            }
        }

        cluster_stabilities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        cluster_stabilities.truncate(n_top);

        Ok(cluster_stabilities)
    }

    // Métodos auxiliares

    fn adjusted_rand_index(&self, labels1: &Array1<usize>, labels2: &Array1<usize>) -> f64 {
        let n = labels1.len();
        if n == 0 {
            return 1.0;
        }

        // Contingency table
        let mut contingency: HashMap<(usize, usize), usize> = HashMap::new();
        for (&l1, &l2) in labels1.iter().zip(labels2.iter()) {
            *contingency.entry((l1, l2)).or_insert(0) += 1;
        }

        // Sum of combinations
        let mut sum_comb_c = 0.0;
        for &count in contingency.values() {
            if count > 1 {
                sum_comb_c += Self::n_choose_2(count);
            }
        }

        // Row and column sums
        let mut row_sums: HashMap<usize, usize> = HashMap::new();
        let mut col_sums: HashMap<usize, usize> = HashMap::new();

        for (&(r, c), &count) in &contingency {
            *row_sums.entry(r).or_insert(0) += count;
            *col_sums.entry(c).or_insert(0) += count;
        }

        let sum_comb_r: f64 = row_sums.values().map(|&c| Self::n_choose_2(c)).sum();
        let sum_comb_c_sum: f64 = col_sums.values().map(|&c| Self::n_choose_2(c)).sum();

        let expected_index = sum_comb_r * sum_comb_c_sum / Self::n_choose_2(n);
        let max_index = (sum_comb_r + sum_comb_c_sum) / 2.0;

        if max_index - expected_index < 1e-10 {
            return 1.0;
        }

        (sum_comb_c - expected_index) / (max_index - expected_index)
    }

    fn n_choose_2(n: usize) -> f64 {
        if n < 2 {
            0.0
        } else {
            (n * (n - 1)) as f64 / 2.0
        }
    }
}

impl Default for StabilityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot de clustering em um ponto no tempo
#[derive(Debug, Clone)]
pub struct ClusterSnapshot {
    pub labels: Array1<usize>,
    pub timestamp: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_stability_analysis() {
        let mut analyzer = StabilityAnalyzer::new();

        // Adiciona snapshots
        analyzer.add_snapshot(array![0, 0, 1, 1], 1000);
        analyzer.add_snapshot(array![0, 0, 1, 1], 2000); // Idêntico

        let stability = analyzer.stability_between_snapshots(0, 1).unwrap();
        assert!((stability - 1.0).abs() < 1e-6); // Deve ser perfeito
    }
}
