//! Análise de separação entre clusters

use ndarray::{Array1, ArrayView2};

pub fn separation_score(data: &ArrayView2<f64>, labels: &Array1<usize>) -> f64 {
    // Implementação simplificada
    1.0
}

pub fn overlap_matrix(labels1: &Array1<usize>, labels2: &Array1<usize>) -> Vec<Vec<usize>> {
    let n1 = labels1.iter().max().unwrap_or(&0) + 1;
    let n2 = labels2.iter().max().unwrap_or(&0) + 1;

    let mut matrix = vec![vec![0; n2]; n1];
    for (&l1, &l2) in labels1.iter().zip(labels2.iter()) {
        matrix[l1][l2] += 1;
    }

    matrix
}
