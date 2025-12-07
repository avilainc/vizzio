//! Model registry for versioning and deployment
//!
//! Manages:
//! - Model versions
//! - Model metadata
//! - Model deployment
//! - A/B testing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRegistry {
    models: HashMap<String, Vec<ModelVersion>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVersion {
    pub id: String,
    pub name: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub metadata: ModelMetadata,
    pub status: ModelStatus,
    // TODO: Add serialized model data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub description: String,
    pub framework: String,
    pub metrics: HashMap<String, f64>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelStatus {
    Training,
    Staging,
    Production,
    Archived,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }

    pub fn register_model(&mut self, model: ModelVersion) -> Result<(), String> {
        self.models
            .entry(model.name.clone())
            .or_insert_with(Vec::new)
            .push(model);
        Ok(())
    }

    pub fn get_model(&self, name: &str, version: &str) -> Option<&ModelVersion> {
        self.models
            .get(name)?
            .iter()
            .find(|m| m.version == version)
    }

    pub fn promote_to_production(&mut self, name: &str, version: &str) -> Result<(), String> {
        // TODO: Implement promotion logic
        Ok(())
    }

    pub fn list_models(&self) -> Vec<String> {
        self.models.keys().cloned().collect()
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}
