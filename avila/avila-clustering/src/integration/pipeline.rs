//! Pipeline composável de clustering

use ndarray::{Array1, ArrayView2};

pub struct ClusteringPipeline {
    steps: Vec<Box<dyn PipelineStep>>,
}

pub trait PipelineStep {
    fn execute(&self, data: &ArrayView2<f64>) -> Result<Array1<usize>, String>;
}

impl ClusteringPipeline {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn add_step(&mut self, step: Box<dyn PipelineStep>) {
        self.steps.push(step);
    }

    pub fn execute(&self, data: &ArrayView2<f64>) -> Result<Array1<usize>, String> {
        if let Some(step) = self.steps.last() {
            step.execute(data)
        } else {
            Err("Pipeline vazio".to_string())
        }
    }
}

impl Default for ClusteringPipeline {
    fn default() -> Self {
        Self::new()
    }
}
