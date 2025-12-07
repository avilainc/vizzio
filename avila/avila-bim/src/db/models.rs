//! Database models

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbModel {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub status: ModelStatus,
    pub ifc_s3_key: Option<String>,
    pub glb_s3_key: Option<String>,
    pub element_count: i32,
    pub file_size_bytes: i64,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelStatus {
    Uploaded,
    Converting,
    Ready,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbElement {
    pub id: Uuid,
    pub model_id: Uuid,
    pub guid: String, // IFC GUID
    pub element_type: String, // IfcWall, IfcSlab, etc.
    pub name: Option<String>,
    pub properties: serde_json::Value,
    pub bounds_min: Vec<f64>, // [x, y, z]
    pub bounds_max: Vec<f64>, // [x, y, z]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbProject {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
