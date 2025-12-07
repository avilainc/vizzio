//! Imputação de valores ausentes

use ndarray::{Array1, Array2, ArrayView2, Axis};

/// Estratégias de imputação
#[derive(Debug, Clone, Copy)]
pub enum ImputationStrategy {
    Mean,
    Median,
    Mode,
    Constant(f64),
}

/// SimpleImputer - imputação simples de valores ausentes
pub struct SimpleImputer {
    strategy: ImputationStrategy,
    fill_values: Option<Array1<f64>>,
    missing_value: f64,
}

impl SimpleImputer {
    pub fn new(strategy: ImputationStrategy) -> Self {
        Self {
            strategy,
            fill_values: None,
            missing_value: f64::NAN,
        }
    }

    pub fn with_missing_value(mut self, value: f64) -> Self {
        self.missing_value = value;
        self
    }

    fn calculate_median(mut values: Vec<f64>) -> f64 {
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = values.len();
        if len == 0 {
            return 0.0;
        }
        if len % 2 == 0 {
            (values[len / 2 - 1] + values[len / 2]) / 2.0
        } else {
            values[len / 2]
        }
    }

    fn calculate_mode(values: Vec<f64>) -> f64 {
        use std::collections::HashMap;

        let mut counts: HashMap<i64, usize> = HashMap::new();
        for &val in &values {
            let key = (val * 1000.0) as i64; // Precisão de 3 casas decimais
            *counts.entry(key).or_insert(0) += 1;
        }

        counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(key, _)| key as f64 / 1000.0)
            .unwrap_or(0.0)
    }

    fn is_missing(&self, value: f64) -> bool {
        if self.missing_value.is_nan() {
            value.is_nan()
        } else {
            (value - self.missing_value).abs() < 1e-10
        }
    }
}

impl super::DataTransformer for SimpleImputer {
    type Output = Array2<f64>;

    fn fit(&mut self, data: &ArrayView2<f64>) -> Result<(), String> {
        let n_features = data.ncols();
        let mut fill_values = Array1::zeros(n_features);

        for i in 0..n_features {
            let col: Vec<f64> = data
                .column(i)
                .iter()
                .copied()
                .filter(|&val| !self.is_missing(val))
                .collect();

            if col.is_empty() {
                return Err(format!("Coluna {} contém apenas valores ausentes", i));
            }

            fill_values[i] = match self.strategy {
                ImputationStrategy::Mean => {
                    col.iter().sum::<f64>() / col.len() as f64
                }
                ImputationStrategy::Median => Self::calculate_median(col),
                ImputationStrategy::Mode => Self::calculate_mode(col),
                ImputationStrategy::Constant(val) => val,
            };
        }

        self.fill_values = Some(fill_values);
        Ok(())
    }

    fn transform(&self, data: &ArrayView2<f64>) -> Result<Self::Output, String> {
        let fill_values = self.fill_values.as_ref().ok_or("Imputer não ajustado")?;

        let mut result = data.to_owned();

        for (i, mut col) in result.axis_iter_mut(Axis(1)).enumerate() {
            for val in col.iter_mut() {
                if self.is_missing(*val) {
                    *val = fill_values[i];
                }
            }
        }

        Ok(result)
    }
}

/// KNNImputer - imputação usando k-nearest neighbors
pub struct KNNImputer {
    n_neighbors: usize,
    missing_value: f64,
}

impl KNNImputer {
    pub fn new(n_neighbors: usize) -> Self {
        Self {
            n_neighbors,
            missing_value: f64::NAN,
        }
    }

    pub fn with_missing_value(mut self, value: f64) -> Self {
        self.missing_value = value;
        self
    }

    fn is_missing(&self, value: f64) -> bool {
        if self.missing_value.is_nan() {
            value.is_nan()
        } else {
            (value - self.missing_value).abs() < 1e-10
        }
    }

    pub fn transform(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>, String> {
        let mut result = data.to_owned();
        let n_samples = data.nrows();

        for i in 0..n_samples {
            let row = data.row(i);

            // Verifica se há valores ausentes
            let has_missing = row.iter().any(|&val| self.is_missing(val));
            if !has_missing {
                continue;
            }

            // Encontra vizinhos mais próximos (implementação simplificada)
            let mut distances: Vec<(usize, f64)> = (0..n_samples)
                .filter(|&j| j != i)
                .map(|j| {
                    let other_row = data.row(j);
                    let dist: f64 = row
                        .iter()
                        .zip(other_row.iter())
                        .filter(|(&a, &b)| !self.is_missing(a) && !self.is_missing(b))
                        .map(|(&a, &b)| (a - b).powi(2))
                        .sum();
                    (j, dist.sqrt())
                })
                .collect();

            distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            // Imputa valores ausentes usando média dos vizinhos
            for (col_idx, &val) in row.iter().enumerate() {
                if self.is_missing(val) {
                    let neighbor_values: Vec<f64> = distances
                        .iter()
                        .take(self.n_neighbors)
                        .map(|(neighbor_idx, _)| data[[*neighbor_idx, col_idx]])
                        .filter(|&v| !self.is_missing(v))
                        .collect();

                    if !neighbor_values.is_empty() {
                        result[[i, col_idx]] = neighbor_values.iter().sum::<f64>()
                            / neighbor_values.len() as f64;
                    }
                }
            }
        }

        Ok(result)
    }
}

impl Default for KNNImputer {
    fn default() -> Self {
        Self::new(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_mean_imputation() {
        let data = array![
            [1.0, 2.0],
            [f64::NAN, 3.0],
            [3.0, f64::NAN],
        ];

        let mut imputer = SimpleImputer::new(ImputationStrategy::Mean);
        imputer.fit(&data.view()).unwrap();
        let result = imputer.transform(&data.view()).unwrap();

        assert!(!result[[1, 0]].is_nan());
        assert!(!result[[2, 1]].is_nan());
    }
}
