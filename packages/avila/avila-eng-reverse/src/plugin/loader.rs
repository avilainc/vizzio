// Plugin dynamic loader
use super::api::Plugin;
use std::error::Error;
use std::path::Path;

/// Plugin loader for dynamic loading of .dll/.so files
pub struct PluginLoader {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginLoader {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Load plugin from file
    pub fn load_plugin<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn Error>> {
        // TODO: Implement dynamic loading using libloading
        todo!("Implement dynamic plugin loading")
    }

    /// Unload plugin by name
    pub fn unload_plugin(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        self.plugins.retain(|p| p.name() != name);
        Ok(())
    }

    /// Get all loaded plugins
    pub fn loaded_plugins(&self) -> &[Box<dyn Plugin>] {
        &self.plugins
    }

    /// Hot reload a plugin
    pub fn reload_plugin(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        // TODO: Implement hot reload
        todo!("Implement hot reload")
    }
}
