//! Sampling estratégico de dados

use ndarray::{Array2, ArrayView2, Axis};

pub struct DataSampler;

impl DataSampler {
    pub fn random_sample(data: &ArrayView2<f64>, n_samples: usize, seed: u64) -> Array2<f64> {
        let n_rows = data.nrows();
        if n_samples >= n_rows {
            return data.to_owned();
        }

        let mut indices: Vec<usize> = (0..n_rows).collect();
        Self::shuffle_vec(&mut indices, seed);

        let selected_indices = &indices[..n_samples];
        let mut result = Array2::zeros((n_samples, data.ncols()));

        for (i, &idx) in selected_indices.iter().enumerate() {
            result.row_mut(i).assign(&data.row(idx));
        }

        result
    }

    pub fn stratified_sample(
        data: &ArrayView2<f64>,
        labels: &[usize],
        n_samples: usize,
    ) -> Result<Array2<f64>, String> {
        if data.nrows() != labels.len() {
            return Err("Dados e labels têm tamanhos diferentes".to_string());
        }

        // Implementação simplificada
        Ok(Self::random_sample(data, n_samples, 42))
    }

    fn shuffle_vec(vec: &mut [usize], seed: u64) {
        let mut rng = seed;
        for i in (1..vec.len()).rev() {
            rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
            let j = (rng as usize) % (i + 1);
            vec.swap(i, j);
        }
    }
}
