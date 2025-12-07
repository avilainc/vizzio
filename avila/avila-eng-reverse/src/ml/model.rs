// ML model trait
use std::error::Error;

/// Trait for malware detection models
pub trait MalwareModel: Send + Sync {
    /// Predict if binary is malware
    fn predict(&self, features: &[f32]) -> Result<Prediction, Box<dyn Error>>;

    /// Get model name
    fn name(&self) -> &str;

    /// Get model version
    fn version(&self) -> &str;

    /// Load model from file
    fn load(path: &str) -> Result<Self, Box<dyn Error>> where Self: Sized;

    /// Save model to file
    fn save(&self, path: &str) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug, Clone)]
pub struct Prediction {
    pub is_malware: bool,
    pub confidence: f32,
    pub probabilities: Vec<ClassProbability>,
    pub detected_family: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ClassProbability {
    pub class_name: String,
    pub probability: f32,
}

impl Prediction {
    pub fn new(is_malware: bool, confidence: f32) -> Self {
        Self {
            is_malware,
            confidence,
            probabilities: Vec::new(),
            detected_family: None,
        }
    }
}
