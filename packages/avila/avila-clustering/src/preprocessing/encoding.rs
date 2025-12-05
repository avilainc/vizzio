//! Codificação de variáveis categóricas

use std::collections::HashMap;
use ndarray::{Array1, Array2};

/// LabelEncoder - codifica categorias como inteiros
pub struct LabelEncoder {
    classes: Option<Vec<String>>,
    class_to_index: HashMap<String, usize>,
}

impl LabelEncoder {
    pub fn new() -> Self {
        Self {
            classes: None,
            class_to_index: HashMap::new(),
        }
    }

    pub fn fit(&mut self, labels: &[String]) -> Result<(), String> {
        let mut unique_labels: Vec<String> = labels.iter().cloned().collect();
        unique_labels.sort();
        unique_labels.dedup();

        self.class_to_index = unique_labels
            .iter()
            .enumerate()
            .map(|(i, label)| (label.clone(), i))
            .collect();

        self.classes = Some(unique_labels);
        Ok(())
    }

    pub fn transform(&self, labels: &[String]) -> Result<Array1<usize>, String> {
        if self.classes.is_none() {
            return Err("Encoder não ajustado".to_string());
        }

        let encoded: Result<Vec<usize>, String> = labels
            .iter()
            .map(|label| {
                self.class_to_index
                    .get(label)
                    .copied()
                    .ok_or_else(|| format!("Categoria desconhecida: {}", label))
            })
            .collect();

        Ok(Array1::from_vec(encoded?))
    }

    pub fn inverse_transform(&self, indices: &Array1<usize>) -> Result<Vec<String>, String> {
        let classes = self.classes.as_ref().ok_or("Encoder não ajustado")?;

        indices
            .iter()
            .map(|&idx| {
                classes
                    .get(idx)
                    .cloned()
                    .ok_or_else(|| format!("Índice inválido: {}", idx))
            })
            .collect()
    }
}

impl Default for LabelEncoder {
    fn default() -> Self {
        Self::new()
    }
}

/// OneHotEncoder - codifica categorias como vetores binários
pub struct OneHotEncoder {
    categories: Option<Vec<Vec<String>>>,
}

impl OneHotEncoder {
    pub fn new() -> Self {
        Self { categories: None }
    }

    pub fn fit(&mut self, data: &[Vec<String>]) -> Result<(), String> {
        if data.is_empty() {
            return Err("Dados vazios".to_string());
        }

        let n_features = data[0].len();
        let mut categories = vec![Vec::new(); n_features];

        for row in data {
            for (i, value) in row.iter().enumerate() {
                if !categories[i].contains(value) {
                    categories[i].push(value.clone());
                }
            }
        }

        for cat in &mut categories {
            cat.sort();
        }

        self.categories = Some(categories);
        Ok(())
    }

    pub fn transform(&self, data: &[Vec<String>]) -> Result<Array2<f64>, String> {
        let categories = self.categories.as_ref().ok_or("Encoder não ajustado")?;

        let n_samples = data.len();
        let n_encoded_features: usize = categories.iter().map(|c| c.len()).sum();

        let mut result = Array2::zeros((n_samples, n_encoded_features));

        for (row_idx, row) in data.iter().enumerate() {
            let mut col_offset = 0;

            for (feature_idx, value) in row.iter().enumerate() {
                let feature_categories = &categories[feature_idx];
                if let Some(cat_idx) = feature_categories.iter().position(|c| c == value) {
                    result[[row_idx, col_offset + cat_idx]] = 1.0;
                }
                col_offset += feature_categories.len();
            }
        }

        Ok(result)
    }

    pub fn n_features_out(&self) -> Option<usize> {
        self.categories.as_ref().map(|cats| cats.iter().map(|c| c.len()).sum())
    }
}

impl Default for OneHotEncoder {
    fn default() -> Self {
        Self::new()
    }
}

/// OrdinalEncoder - codifica categorias ordinais preservando ordem
pub struct OrdinalEncoder {
    categories: Option<Vec<Vec<String>>>,
}

impl OrdinalEncoder {
    pub fn new() -> Self {
        Self { categories: None }
    }

    pub fn fit(&mut self, data: &[Vec<String>]) -> Result<(), String> {
        if data.is_empty() {
            return Err("Dados vazios".to_string());
        }

        let n_features = data[0].len();
        let mut categories = vec![Vec::new(); n_features];

        for row in data {
            for (i, value) in row.iter().enumerate() {
                if !categories[i].contains(value) {
                    categories[i].push(value.clone());
                }
            }
        }

        self.categories = Some(categories);
        Ok(())
    }

    pub fn transform(&self, data: &[Vec<String>]) -> Result<Array2<f64>, String> {
        let categories = self.categories.as_ref().ok_or("Encoder não ajustado")?;

        let n_samples = data.len();
        let n_features = categories.len();
        let mut result = Array2::zeros((n_samples, n_features));

        for (row_idx, row) in data.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                let ord = categories[col_idx]
                    .iter()
                    .position(|c| c == value)
                    .ok_or_else(|| format!("Categoria desconhecida: {}", value))?;
                result[[row_idx, col_idx]] = ord as f64;
            }
        }

        Ok(result)
    }
}

impl Default for OrdinalEncoder {
    fn default() -> Self {
        Self::new()
    }
}
