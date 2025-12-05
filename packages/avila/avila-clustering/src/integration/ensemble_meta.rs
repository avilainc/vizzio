//! Meta-ensemble de múltiplos algoritmos

use ndarray::{Array1, ArrayView2};

pub struct EnsembleMeta {
    algorithms: Vec<String>,
    voting_strategy: VotingStrategy,
}

#[derive(Debug, Clone, Copy)]
pub enum VotingStrategy {
    Majority,
    Weighted,
    Consensus,
}

impl EnsembleMeta {
    pub fn new(algorithms: Vec<String>) -> Self {
        Self {
            algorithms,
            voting_strategy: VotingStrategy::Majority,
        }
    }

    pub fn fit(&mut self, data: &ArrayView2<f64>) -> Result<Array1<usize>, String> {
        Ok(Array1::zeros(data.nrows()))
    }
}
