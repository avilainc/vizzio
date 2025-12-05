//! Seleção e redução de dimensionalidade de features

use ndarray::{Array1, Array2, ArrayView2, Axis};

/// PCA - Análise de Componentes Principais
pub struct PCA {
    n_components: usize,
    components: Option<Array2<f64>>,
    mean: Option<Array1<f64>>,
    explained_variance: Option<Array1<f64>>,
}

impl PCA {
    pub fn new(n_components: usize) -> Self {
        Self {
            n_components,
            components: None,
            mean: None,
            explained_variance: None,
        }
    }

    pub fn explained_variance(&self) -> Option<&Array1<f64>> {
        self.explained_variance.as_ref()
    }

    pub fn explained_variance_ratio(&self) -> Option<Array1<f64>> {
        self.explained_variance.as_ref().map(|var| {
            let total: f64 = var.iter().sum();
            var.mapv(|v| v / total)
        })
    }
}

impl super::DataTransformer for PCA {
    type Output = Array2<f64>;

    fn fit(&mut self, data: &ArrayView2<f64>) -> Result<(), String> {
        let mean = data.mean_axis(Axis(0)).ok_or("Erro ao calcular média")?;
        let centered = data.to_owned() - &mean;

        // Matriz de covariância
        let n_samples = centered.nrows() as f64;
        let cov = centered.t().dot(&centered) / (n_samples - 1.0);

        // TODO: Implementar eigendecomposition
        // Por enquanto, retorna erro indicando necessidade de implementação completa
        Err("PCA requer implementação de eigendecomposition".to_string())
    }

    fn transform(&self, data: &ArrayView2<f64>) -> Result<Self::Output, String> {
        let mean = self.mean.as_ref().ok_or("PCA não ajustado")?;
        let components = self.components.as_ref().ok_or("PCA não ajustado")?;

        let centered = data.to_owned() - mean;
        Ok(centered.dot(components))
    }
}

/// VarianceThreshold - remove features com baixa variância
pub struct VarianceThreshold {
    threshold: f64,
    variances: Option<Array1<f64>>,
    selected_features: Option<Vec<usize>>,
}

impl VarianceThreshold {
    pub fn new(threshold: f64) -> Self {
        Self {
            threshold,
            variances: None,
            selected_features: None,
        }
    }

    pub fn variances(&self) -> Option<&Array1<f64>> {
        self.variances.as_ref()
    }

    pub fn selected_features(&self) -> Option<&Vec<usize>> {
        self.selected_features.as_ref()
    }
}

impl super::DataTransformer for VarianceThreshold {
    type Output = Array2<f64>;

    fn fit(&mut self, data: &ArrayView2<f64>) -> Result<(), String> {
        let variances = data.var_axis(Axis(0), 0.0);
        let selected: Vec<usize> = variances
            .iter()
            .enumerate()
            .filter(|(_, &var)| var > self.threshold)
            .map(|(i, _)| i)
            .collect();

        if selected.is_empty() {
            return Err("Nenhuma feature passou o threshold de variância".to_string());
        }

        self.variances = Some(variances);
        self.selected_features = Some(selected);
        Ok(())
    }

    fn transform(&self, data: &ArrayView2<f64>) -> Result<Self::Output, String> {
        let selected = self.selected_features.as_ref().ok_or("Não ajustado")?;

        let n_samples = data.nrows();
        let n_selected = selected.len();
        let mut result = Array2::zeros((n_samples, n_selected));

        for (new_idx, &old_idx) in selected.iter().enumerate() {
            result.column_mut(new_idx).assign(&data.column(old_idx));
        }

        Ok(result)
    }
}

/// FeatureSelector - seleção customizada de features
pub struct FeatureSelector {
    selected_indices: Vec<usize>,
}

impl FeatureSelector {
    pub fn new(indices: Vec<usize>) -> Self {
        Self {
            selected_indices: indices,
        }
    }

    pub fn from_names(feature_names: &[String], selected_names: &[String]) -> Result<Self, String> {
        let indices: Result<Vec<usize>, String> = selected_names
            .iter()
            .map(|name| {
                feature_names
                    .iter()
                    .position(|n| n == name)
                    .ok_or_else(|| format!("Feature não encontrada: {}", name))
            })
            .collect();

        Ok(Self {
            selected_indices: indices?,
        })
    }

    pub fn transform(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>, String> {
        let n_samples = data.nrows();
        let n_features = self.selected_indices.len();

        if self.selected_indices.iter().any(|&i| i >= data.ncols()) {
            return Err("Índice de feature inválido".to_string());
        }

        let mut result = Array2::zeros((n_samples, n_features));

        for (new_idx, &old_idx) in self.selected_indices.iter().enumerate() {
            result.column_mut(new_idx).assign(&data.column(old_idx));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_variance_threshold() {
        let data = array![
            [1.0, 0.0, 5.0],
            [2.0, 0.0, 6.0],
            [3.0, 0.0, 7.0],
        ];

        let mut selector = VarianceThreshold::new(0.1);
        selector.fit(&data.view()).unwrap();

        let transformed = selector.transform(&data.view()).unwrap();
        assert_eq!(transformed.ncols(), 2); // Remove coluna com var=0
    }
}
