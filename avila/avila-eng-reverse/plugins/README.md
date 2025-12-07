# Plugins Directory

Place custom plugins here for extending Deriax functionality.

## Plugin Structure

A plugin is a dynamic library (.dll/.so) that implements the `Plugin` trait:

```rust
use deriax::plugin::{Plugin, PluginContext, PluginResult};

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "MyCustomPlugin"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn author(&self) -> &str {
        "Your Name"
    }

    fn description(&self) -> &str {
        "Description of what the plugin does"
    }

    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize plugin
        Ok(())
    }

    fn execute(&self, context: &PluginContext) -> Result<PluginResult, Box<dyn std::error::Error>> {
        // Plugin logic here
        let mut result = PluginResult::new();
        result.success = true;
        Ok(result)
    }

    fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Cleanup resources
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(MyPlugin))
}
```

## Building a Plugin

```bash
cargo build --release --lib
```

## Installing

Copy the compiled plugin to this directory:
```bash
cp target/release/libmyplugin.so plugins/
```

The plugin will be auto-loaded on startup if `auto_load = true` in config.toml.
