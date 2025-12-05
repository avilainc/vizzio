// Plugin API trait definition
use std::any::Any;
use std::error::Error;

/// Plugin trait - All plugins must implement this
pub trait Plugin: Any + Send + Sync {
    /// Get plugin name
    fn name(&self) -> &str;

    /// Get plugin version
    fn version(&self) -> &str;

    /// Get plugin author
    fn author(&self) -> &str;

    /// Get plugin description
    fn description(&self) -> &str;

    /// Initialize plugin
    fn init(&mut self) -> Result<(), Box<dyn Error>>;

    /// Execute plugin with given context
    fn execute(&self, context: &PluginContext) -> Result<PluginResult, Box<dyn Error>>;

    /// Cleanup plugin resources
    fn cleanup(&mut self) -> Result<(), Box<dyn Error>>;
}

/// Context passed to plugins during execution
#[derive(Debug, Clone)]
pub struct PluginContext {
    pub binary_path: String,
    pub binary_data: Vec<u8>,
    pub analysis_mode: AnalysisMode,
    pub config: PluginConfig,
}

/// Plugin configuration
#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub timeout_seconds: u64,
    pub max_memory_mb: usize,
    pub parameters: std::collections::HashMap<String, String>,
}

/// Analysis mode for plugins
#[derive(Debug, Clone, PartialEq)]
pub enum AnalysisMode {
    Static,
    Dynamic,
    Hybrid,
}

/// Result returned by plugin execution
#[derive(Debug, Clone)]
pub struct PluginResult {
    pub success: bool,
    pub data: serde_json::Value,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl PluginResult {
    pub fn new() -> Self {
        Self {
            success: true,
            data: serde_json::Value::Null,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
}
