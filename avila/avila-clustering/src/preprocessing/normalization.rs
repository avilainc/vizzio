//! Algoritmos de normalização e escalonamento de dados

use ndarray::{Array1, Array2, ArrayView2, Axis};

/// StandardScaler - normalização Z-score (média 0, desvio padrão 1)
pub struct StandardScaler {
    mean: Option<Array1<f64>>,
    std: Option<Array1<f64>>,
}

impl StandardScaler {
    pub fn new() -> Self {
        Self {
            mean: None,
            std: None,
        }
    }
}

impl Default for StandardScaler {
    fn default() -> Self {
        Self::new()
    }
}

impl super::DataTransformer for StandardScaler {
    type Output = Array2<f64>;

    fn fit(&mut self, data: &ArrayView2<f64>) -> Result<(), String> {
        let mean = data.mean_axis(Axis(0)).ok_or("Erro ao calcular média")?;
        let std = data.std_axis(Axis(0), 0.0);

        self.mean = Some(mean);
        self.std = Some(std);
        Ok(())
    }

    fn transform(&self, data: &ArrayView2<f64>) -> Result<Self::Output, String> {
        let mean = self.mean.as_ref().ok_or("Scaler não ajustado")?;
        let std = self.std.as_ref().ok_or("Scaler não ajustado")?;

        let mut result = data.to_owned();
        for (i, mut col) in result.axis_iter_mut(Axis(1)).enumerate() {
            col -= mean[i];
            if std[i] > 1e-10 {
                col /= std[i];
            }
        }

        Ok(result)
    }
}

/// MinMaxScaler - escalonamento para intervalo [0, 1]
pub struct MinMaxScaler {
    min: Option<Array1<f64>>,
    max: Option<Array1<f64>>,
    feature_range: (f64, f64),
}

impl MinMaxScaler {
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
            feature_range: (0.0, 1.0),
        }
    }

    pub fn with_range(min: f64, max: f64) -> Self {
        Self {
            min: None,
            max: None,
            feature_range: (min, max),
        }
    }
}

impl Default for MinMaxScaler {
    fn default() -> Self {
        Self::new()
    }
}

impl super::DataTransformer for MinMaxScaler {
    type Output = Array2<f64>;

    fn fit(&mut self, data: &ArrayView2<f64>) -> Result<(), String> {
        let min = data.map_axis(Axis(0), |col| {
            col.iter().copied().fold(f64::INFINITY, f64::min)
        });
        let max = data.map_axis(Axis(0), |col| {
            col.iter().copied().fold(f64::NEG_INFINITY, f64::max)
        });

        self.min = Some(min);
        self.max = Some(max);
        Ok(())
    }

    fn transform(&self, data: &ArrayView2<f64>) -> Result<Self::Output, String> {
        let min = self.min.as_ref().ok_or("Scaler não ajustado")?;
        let max = self.max.as_ref().ok_or("Scaler não ajustado")?;

        let (target_min, target_max) = self.feature_range;
        let target_range = target_max - target_min;

        let mut result = data.to_owned();
        for (i, mut col) in result.axis_iter_mut(Axis(1)).enumerate() {
            let range = max[i] - min[i];
            if range > 1e-10 {
                col -= min[i];
                col /= range;
                col *= target_range;
                col += target_min;
            }
        }

        Ok(result)
    }
}

/// RobustScaler - escalonamento robusto usando mediana e IQR
pub struct RobustScaler {
    median: Option<Array1<f64>>,
    iqr: Option<Array1<f64>>,
}

impl RobustScaler {
    pub fn new() -> Self {
        Self {
            median: None,
            iqr: None,
        }
    }

    fn calculate_median(mut values: Vec<f64>) -> f64 {
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = values.len();
        if len % 2 == 0 {
            (values[len / 2 - 1] + values[len / 2]) / 2.0
        } else {
            values[len / 2]
        }
    }
}

impl Default for RobustScaler {
    fn default() -> Self {
        Self::new()
    }
}

impl super::DataTransformer for RobustScaler {
    type Output = Array2<f64>;

    fn fit(&mut self, data: &ArrayView2<f64>) -> Result<(), String> {
        let n_features = data.ncols();
        let mut median = Array1::zeros(n_features);
        let mut iqr = Array1::zeros(n_features);

        for i in 0..n_features {
            let col: Vec<f64> = data.column(i).to_vec();
            median[i] = Self::calculate_median(col.clone());

            let q1 = Self::calculate_median(col.iter().filter(|&&x| x < median[i]).copied().collect());
            let q3 = Self::calculate_median(col.iter().filter(|&&x| x > median[i]).copied().collect());
            iqr[i] = q3 - q1;
        }

        self.median = Some(median);
        self.iqr = Some(iqr);
        Ok(())
    }

    fn transform(&self, data: &ArrayView2<f64>) -> Result<Self::Output, String> {
        let median = self.median.as_ref().ok_or("Scaler não ajustado")?;
        let iqr = self.iqr.as_ref().ok_or("Scaler não ajustado")?;

        let mut result = data.to_owned();
        for (i, mut col) in result.axis_iter_mut(Axis(1)).enumerate() {
            col -= median[i];
            if iqr[i] > 1e-10 {
                col /= iqr[i];
            }
        }

        Ok(result)
    }
}
