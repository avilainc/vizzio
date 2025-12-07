// Unit tests for plugin system
#[cfg(test)]
mod plugin_tests {
    use crate::plugin::*;

    #[test]
    fn test_plugin_registry() {
        let mut registry = PluginRegistry::new();
        assert_eq!(registry.list_plugins().len(), 0);
    }

    #[test]
    fn test_plugin_sandbox() {
        let sandbox = PluginSandbox::new(60, 256);
        assert!(sandbox.check_limits(100 * 1024 * 1024));
    }
}
