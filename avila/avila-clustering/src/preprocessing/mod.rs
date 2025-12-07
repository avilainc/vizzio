//! Módulo de pré-processamento de dados para clustering
//!
//! Fornece ferramentas para preparação e transformação de dados antes da aplicação
//! de algoritmos de clustering.

pub mod normalization;
pub mod encoding;
pub mod feature_selection;
pub mod outlier_removal;
pub mod imputation;

pub use normalization::*;
pub use encoding::*;
pub use feature_selection::*;
pub use outlier_removal::*;
pub use imputation::*;

use ndarray::ArrayView2;

/// Trait comum para transformadores de dados
pub trait DataTransformer {
    type Output;

    /// Ajusta o transformador aos dados
    fn fit(&mut self, data: &ArrayView2<f64>) -> Result<(), String>;

    /// Transforma os dados
    fn transform(&self, data: &ArrayView2<f64>) -> Result<Self::Output, String>;

    /// Ajusta e transforma em uma única operação
    fn fit_transform(&mut self, data: &ArrayView2<f64>) -> Result<Self::Output, String> {
        self.fit(data)?;
        self.transform(data)
    }
}
