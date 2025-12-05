//! Detecção e remoção de outliers

use ndarray::{Array1, Array2, ArrayView2, Axis};

/// IQROutlierRemover - remove outliers usando método IQR
pub struct IQROutlierRemover {
    factor: f64,
    lower_bounds: Option<Array1<f64>>,
    upper_bounds: Option<Array1<f64>>,
}

impl IQROutlierRemover {
    pub fn new(factor: f64) -> Self {
        Self {
            factor,
            lower_bounds: None,
            upper_bounds: None,
        }
    }

    fn calculate_quartiles(mut values: Vec<f64>) -> (f64, f64, f64) {
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = values.len();

        let q1_idx = len / 4;
        let q2_idx = len / 2;
        let q3_idx = 3 * len / 4;

        (values[q1_idx], values[q2_idx], values[q3_idx])
    }

    pub fn fit(&mut self, data: &ArrayView2<f64>) -> Result<(), String> {
        let n_features = data.ncols();
        let mut lower = Array1::zeros(n_features);
        let mut upper = Array1::zeros(n_features);

        for i in 0..n_features {
            let col: Vec<f64> = data.column(i).to_vec();
            let (q1, _, q3) = Self::calculate_quartiles(col);
            let iqr = q3 - q1;

            lower[i] = q1 - self.factor * iqr;
            upper[i] = q3 + self.factor * iqr;
        }

        self.lower_bounds = Some(lower);
        self.upper_bounds = Some(upper);
        Ok(())
    }

    pub fn is_outlier(&self, data: &ArrayView2<f64>) -> Result<Array1<bool>, String> {
        let lower = self.lower_bounds.as_ref().ok_or("Não ajustado")?;
        let upper = self.upper_bounds.as_ref().ok_or("Não ajustado")?;

        let mut outliers = Array1::from_elem(data.nrows(), false);

        for (i, row) in data.axis_iter(Axis(0)).enumerate() {
            for (j, &value) in row.iter().enumerate() {
                if value < lower[j] || value > upper[j] {
                    outliers[i] = true;
                    break;
                }
            }
        }

        Ok(outliers)
    }

    pub fn remove_outliers(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>, String> {
        let outliers = self.is_outlier(data)?;

        let n_inliers = outliers.iter().filter(|&&is_outlier| !is_outlier).count();
        let n_features = data.ncols();

        let mut result = Array2::zeros((n_inliers, n_features));
        let mut result_idx = 0;

        for (i, &is_outlier) in outliers.iter().enumerate() {
            if !is_outlier {
                result.row_mut(result_idx).assign(&data.row(i));
                result_idx += 1;
            }
        }

        Ok(result)
    }
}

impl Default for IQROutlierRemover {
    fn default() -> Self {
        Self::new(1.5)
    }
}

/// ZScoreOutlierRemover - remove outliers usando Z-score
pub struct ZScoreOutlierRemover {
    threshold: f64,
    mean: Option<Array1<f64>>,
    std: Option<Array1<f64>>,
}

impl ZScoreOutlierRemover {
    pub fn new(threshold: f64) -> Self {
        Self {
            threshold,
            mean: None,
            std: None,
        }
    }

    pub fn fit(&mut self, data: &ArrayView2<f64>) -> Result<(), String> {
        let mean = data.mean_axis(Axis(0)).ok_or("Erro ao calcular média")?;
        let std = data.std_axis(Axis(0), 0.0);

        self.mean = Some(mean);
        self.std = Some(std);
        Ok(())
    }

    pub fn is_outlier(&self, data: &ArrayView2<f64>) -> Result<Array1<bool>, String> {
        let mean = self.mean.as_ref().ok_or("Não ajustado")?;
        let std = self.std.as_ref().ok_or("Não ajustado")?;

        let mut outliers = Array1::from_elem(data.nrows(), false);

        for (i, row) in data.axis_iter(Axis(0)).enumerate() {
            for (j, &value) in row.iter().enumerate() {
                if std[j] > 1e-10 {
                    let z_score = ((value - mean[j]) / std[j]).abs();
                    if z_score > self.threshold {
                        outliers[i] = true;
                        break;
                    }
                }
            }
        }

        Ok(outliers)
    }

    pub fn remove_outliers(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>, String> {
        let outliers = self.is_outlier(data)?;

        let n_inliers = outliers.iter().filter(|&&is_outlier| !is_outlier).count();
        let n_features = data.ncols();

        let mut result = Array2::zeros((n_inliers, n_features));
        let mut result_idx = 0;

        for (i, &is_outlier) in outliers.iter().enumerate() {
            if !is_outlier {
                result.row_mut(result_idx).assign(&data.row(i));
                result_idx += 1;
            }
        }

        Ok(result)
    }
}

impl Default for ZScoreOutlierRemover {
    fn default() -> Self {
        Self::new(3.0)
    }
}

/// LocalOutlierFactor - detecção de outliers baseada em densidade local
pub struct LocalOutlierFactor {
    n_neighbors: usize,
    contamination: f64,
}

impl LocalOutlierFactor {
    pub fn new(n_neighbors: usize, contamination: f64) -> Self {
        Self {
            n_neighbors,
            contamination,
        }
    }

    pub fn fit_predict(&self, data: &ArrayView2<f64>) -> Result<Array1<i32>, String> {
        // Implementação simplificada - versão completa requer estrutura k-NN
        let n_samples = data.nrows();
        let mut scores = Array1::ones(n_samples);

        // Placeholder: marcar como inliers por padrão
        Ok(scores.mapv(|_| 1))
    }
}

impl Default for LocalOutlierFactor {
    fn default() -> Self {
        Self::new(20, 0.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_zscore_outlier() {
        let data = array![
            [1.0, 2.0],
            [2.0, 3.0],
            [3.0, 4.0],
            [100.0, 200.0], // outlier claro
        ];

        let mut remover = ZScoreOutlierRemover::new(2.0);
        remover.fit(&data.view()).unwrap();

        let outliers = remover.is_outlier(&data.view()).unwrap();
        assert!(outliers[3]); // última linha deve ser outlier
    }
}
