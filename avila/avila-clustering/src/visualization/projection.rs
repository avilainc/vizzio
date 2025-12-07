//! Projeções 2D/3D para visualização de dados multidimensionais

use ndarray::{Array1, Array2, ArrayView2, Axis};

/// Tipo de projeção
#[derive(Debug, Clone, Copy)]
pub enum ProjectionType {
    PCA,
    TSNE,
    RandomProjection,
}

/// ProjectionEngine - reduz dimensionalidade para visualização
pub struct ProjectionEngine {
    projection_type: ProjectionType,
}

impl ProjectionEngine {
    pub fn new(projection_type: ProjectionType) -> Self {
        Self { projection_type }
    }

    /// Projeta dados para 2D
    pub fn project_2d(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>, String> {
        match self.projection_type {
            ProjectionType::PCA => self.pca_projection_2d(data),
            ProjectionType::TSNE => self.tsne_projection_2d(data),
            ProjectionType::RandomProjection => self.random_projection_2d(data),
        }
    }

    /// Projeta dados para 3D
    pub fn project_3d(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>, String> {
        match self.projection_type {
            ProjectionType::PCA => self.pca_projection_3d(data),
            ProjectionType::TSNE => self.tsne_projection_3d(data),
            ProjectionType::RandomProjection => self.random_projection_3d(data),
        }
    }

    // Implementações

    fn pca_projection_2d(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>, String> {
        // Centra os dados
        let mean = data.mean_axis(Axis(0)).ok_or("Erro ao calcular média")?;
        let centered = data.to_owned() - &mean;

        // TODO: Implementação completa de PCA requer eigendecomposition
        // Por ora, retorna projeção simplificada usando primeiras 2 dimensões
        if data.ncols() < 2 {
            return Err("Dados precisam ter pelo menos 2 features".to_string());
        }

        let mut result = Array2::zeros((data.nrows(), 2));
        result.column_mut(0).assign(&centered.column(0));
        result.column_mut(1).assign(&centered.column(1));

        Ok(result)
    }

    fn pca_projection_3d(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>, String> {
        let mean = data.mean_axis(Axis(0)).ok_or("Erro ao calcular média")?;
        let centered = data.to_owned() - &mean;

        if data.ncols() < 3 {
            return Err("Dados precisam ter pelo menos 3 features".to_string());
        }

        let mut result = Array2::zeros((data.nrows(), 3));
        result.column_mut(0).assign(&centered.column(0));
        result.column_mut(1).assign(&centered.column(1));
        result.column_mut(2).assign(&centered.column(2));

        Ok(result)
    }

    fn tsne_projection_2d(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>, String> {
        // Implementação simplificada - t-SNE real requer otimização complexa
        // Por ora, usa projeção aleatória como placeholder
        self.random_projection_2d(data)
    }

    fn tsne_projection_3d(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>, String> {
        self.random_projection_3d(data)
    }

    fn random_projection_2d(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>, String> {
        let n_samples = data.nrows();
        let n_features = data.ncols();

        // Matriz de projeção aleatória
        let mut projection_matrix = Array2::zeros((n_features, 2));
        for i in 0..n_features {
            for j in 0..2 {
                projection_matrix[[i, j]] = (i * 7 + j * 13) as f64 / 100.0 - 0.5;
            }
        }

        // Normaliza colunas
        for j in 0..2 {
            let norm: f64 = projection_matrix.column(j).iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm > 1e-10 {
                for i in 0..n_features {
                    projection_matrix[[i, j]] /= norm;
                }
            }
        }

        Ok(data.dot(&projection_matrix))
    }

    fn random_projection_3d(&self, data: &ArrayView2<f64>) -> Result<Array2<f64>, String> {
        let n_samples = data.nrows();
        let n_features = data.ncols();

        let mut projection_matrix = Array2::zeros((n_features, 3));
        for i in 0..n_features {
            for j in 0..3 {
                projection_matrix[[i, j]] = (i * 7 + j * 13) as f64 / 100.0 - 0.5;
            }
        }

        for j in 0..3 {
            let norm: f64 = projection_matrix.column(j).iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm > 1e-10 {
                for i in 0..n_features {
                    projection_matrix[[i, j]] /= norm;
                }
            }
        }

        Ok(data.dot(&projection_matrix))
    }
}

/// Estrutura para dados de visualização 2D/3D
#[derive(Debug, Clone)]
pub struct ProjectedData {
    pub coordinates: Array2<f64>,
    pub labels: Option<Array1<usize>>,
    pub colors: Option<Vec<String>>,
    pub dimensions: usize,
}

impl ProjectedData {
    pub fn new_2d(coordinates: Array2<f64>) -> Result<Self, String> {
        if coordinates.ncols() != 2 {
            return Err("Coordenadas 2D devem ter 2 colunas".to_string());
        }

        Ok(Self {
            coordinates,
            labels: None,
            colors: None,
            dimensions: 2,
        })
    }

    pub fn new_3d(coordinates: Array2<f64>) -> Result<Self, String> {
        if coordinates.ncols() != 3 {
            return Err("Coordenadas 3D devem ter 3 colunas".to_string());
        }

        Ok(Self {
            coordinates,
            labels: None,
            colors: None,
            dimensions: 3,
        })
    }

    pub fn with_labels(mut self, labels: Array1<usize>) -> Self {
        self.labels = Some(labels);
        self
    }

    pub fn with_colors(mut self, colors: Vec<String>) -> Self {
        self.colors = Some(colors);
        self
    }

    /// Gera cores automáticas baseadas em labels
    pub fn auto_color(&mut self) {
        if let Some(ref labels) = self.labels {
            let n_clusters = labels.iter().max().unwrap_or(&0) + 1;
            let palette = Self::generate_color_palette(n_clusters);

            let colors: Vec<String> = labels
                .iter()
                .map(|&label| palette[label % palette.len()].clone())
                .collect();

            self.colors = Some(colors);
        }
    }

    fn generate_color_palette(n_colors: usize) -> Vec<String> {
        let base_colors = vec![
            "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd",
            "#8c564b", "#e377c2", "#7f7f7f", "#bcbd22", "#17becf",
        ];

        let mut palette = Vec::new();
        for i in 0..n_colors {
            palette.push(base_colors[i % base_colors.len()].to_string());
        }

        palette
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_projection_2d() {
        let data = array![
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 3.0, 4.0, 5.0],
            [3.0, 4.0, 5.0, 6.0],
        ];

        let engine = ProjectionEngine::new(ProjectionType::RandomProjection);
        let projected = engine.project_2d(&data.view()).unwrap();

        assert_eq!(projected.nrows(), 3);
        assert_eq!(projected.ncols(), 2);
    }
}
