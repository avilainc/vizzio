// Malware predictor
use super::feature_extractor::FeatureExtractor;
use super::model::MalwareModel;
use std::error::Error;

/// Malware predictor combining feature extraction and model inference
pub struct MalwarePredictor {
    extractor: FeatureExtractor,
    model: Box<dyn MalwareModel>,
}

impl MalwarePredictor {
    pub fn new(model: Box<dyn MalwareModel>) -> Self {
        Self {
            extractor: FeatureExtractor::new(),
            model,
        }
    }

    /// Predict if binary is malware
    pub fn predict(&self, binary_data: &[u8]) -> Result<PredictionResult, Box<dyn Error>> {
        // Extract features
        let features = self.extractor.extract(binary_data)?;

        // Run prediction
        let prediction = self.model.predict(&features)?;

        Ok(PredictionResult {
            is_malware: prediction.is_malware,
            confidence: prediction.confidence,
            family: prediction.detected_family,
            model_name: self.model.name().to_string(),
            features_used: features.len(),
        })
    }

    /// Batch prediction
    pub fn predict_batch(&self, binaries: &[Vec<u8>]) -> Result<Vec<PredictionResult>, Box<dyn Error>> {
        binaries
            .iter()
            .map(|binary| self.predict(binary))
            .collect()
    }

    /// Get model info
    pub fn model_info(&self) -> ModelInfo {
        ModelInfo {
            name: self.model.name().to_string(),
            version: self.model.version().to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PredictionResult {
    pub is_malware: bool,
    pub confidence: f32,
    pub family: Option<String>,
    pub model_name: String,
    pub features_used: usize,
}

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub version: String,
}
