// Plugin registry for management
use super::api::Plugin;
use std::collections::HashMap;
use std::error::Error;

/// Plugin registry for managing registered plugins
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
    enabled: HashMap<String, bool>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            enabled: HashMap::new(),
        }
    }

    /// Register a plugin
    pub fn register(&mut self, plugin: Box<dyn Plugin>) -> Result<(), Box<dyn Error>> {
        let name = plugin.name().to_string();
        self.plugins.insert(name.clone(), plugin);
        self.enabled.insert(name, true);
        Ok(())
    }

    /// Unregister a plugin
    pub fn unregister(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        self.plugins.remove(name);
        self.enabled.remove(name);
        Ok(())
    }

    /// Get plugin by name
    pub fn get(&self, name: &str) -> Option<&Box<dyn Plugin>> {
        self.plugins.get(name)
    }

    /// Enable/disable plugin
    pub fn set_enabled(&mut self, name: &str, enabled: bool) {
        if self.plugins.contains_key(name) {
            self.enabled.insert(name.to_string(), enabled);
        }
    }

    /// Check if plugin is enabled
    pub fn is_enabled(&self, name: &str) -> bool {
        self.enabled.get(name).copied().unwrap_or(false)
    }

    /// List all plugins
    pub fn list_plugins(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }
}
