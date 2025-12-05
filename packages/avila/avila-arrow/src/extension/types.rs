//! Extension type definitions

use std::sync::Arc;

/// Extension type trait
pub trait ExtensionType: Send + Sync {
    fn name(&self) -> &str;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Self where Self: Sized;
}

/// Generic extension type wrapper
#[derive(Debug, Clone)]
pub struct Extension {
    pub name: String,
    pub metadata: Vec<u8>,
}

impl Extension {
    pub fn new(name: impl Into<String>, metadata: Vec<u8>) -> Self {
        Self {
            name: name.into(),
            metadata,
        }
    }
}
