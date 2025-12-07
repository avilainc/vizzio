//! Extension type registry

use super::types::Extension;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Global extension type registry
pub struct ExtensionRegistry {
    types: RwLock<HashMap<String, Arc<Extension>>>,
}

impl ExtensionRegistry {
    pub fn new() -> Self {
        Self {
            types: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, extension: Arc<Extension>) {
        let mut types = self.types.write().unwrap();
        types.insert(extension.name.clone(), extension);
    }

    pub fn get(&self, name: &str) -> Option<Arc<Extension>> {
        let types = self.types.read().unwrap();
        types.get(name).cloned()
    }

    pub fn list(&self) -> Vec<String> {
        let types = self.types.read().unwrap();
        types.keys().cloned().collect()
    }
}

impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
