//! Plugin registry for managing registered plugins

use std::collections::HashMap;
use std::sync::Arc;

use crate::error::{Error, Result};

use super::context::PluginContext;
use super::hooks::PluginHook;
use super::traits::Plugin;

/// Registry for managing plugins
///
/// The registry maintains a collection of registered plugins and provides
/// methods for executing hooks across all enabled plugins.
///
/// # Example
///
/// ```rust,ignore
/// use xcargo::plugin::{PluginRegistry, PluginContext, PluginHook};
///
/// let mut registry = PluginRegistry::new();
/// registry.register(Box::new(MyPlugin));
///
/// let ctx = PluginContext::new("x86_64-unknown-linux-gnu".to_string());
/// registry.execute_hook(PluginHook::PreBuild, &ctx)?;
/// ```
#[derive(Default)]
pub struct PluginRegistry {
    /// Registered plugins by name
    plugins: HashMap<String, Arc<dyn Plugin>>,

    /// Execution order for plugins (by name)
    /// If empty, plugins execute in arbitrary order
    execution_order: Vec<String>,
}

impl PluginRegistry {
    /// Create a new empty plugin registry
    #[must_use]
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            execution_order: Vec::new(),
        }
    }

    /// Register a plugin
    ///
    /// # Errors
    /// Returns error if a plugin with the same name is already registered
    pub fn register(&mut self, plugin: Box<dyn Plugin>) -> Result<()> {
        let name = plugin.name().to_string();

        if self.plugins.contains_key(&name) {
            return Err(Error::Config(format!(
                "Plugin '{name}' is already registered"
            )));
        }

        // Initialize the plugin
        plugin.on_init()?;

        self.plugins.insert(name.clone(), Arc::from(plugin));
        self.execution_order.push(name);

        Ok(())
    }

    /// Unregister a plugin by name
    ///
    /// # Errors
    /// Returns error if plugin shutdown fails
    pub fn unregister(&mut self, name: &str) -> Result<()> {
        if let Some(plugin) = self.plugins.remove(name) {
            plugin.on_shutdown()?;
            self.execution_order.retain(|n| n != name);
        }
        Ok(())
    }

    /// Get a plugin by name
    #[must_use]
    pub fn get(&self, name: &str) -> Option<Arc<dyn Plugin>> {
        self.plugins.get(name).cloned()
    }

    /// Check if a plugin is registered
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.plugins.contains_key(name)
    }

    /// Get number of registered plugins
    #[must_use]
    pub fn count(&self) -> usize {
        self.plugins.len()
    }

    /// List all registered plugin names
    #[must_use]
    pub fn list(&self) -> Vec<String> {
        self.execution_order.clone()
    }

    /// Execute a hook on all registered plugins
    ///
    /// Plugins execute in the order they were registered.
    /// If any plugin returns an error, execution stops and the error is returned.
    ///
    /// # Errors
    /// Returns error if any plugin hook fails
    pub fn execute_hook(&self, hook: PluginHook, ctx: &PluginContext) -> Result<()> {
        for name in &self.execution_order {
            if let Some(plugin) = self.plugins.get(name) {
                hook.execute(plugin.as_ref(), ctx)?;
            }
        }
        Ok(())
    }

    /// Execute a hook with error message (for BuildFailed hook)
    ///
    /// # Errors
    /// Returns error if any plugin hook fails
    pub fn execute_hook_with_error(
        &self,
        hook: PluginHook,
        ctx: &PluginContext,
        error: &str,
    ) -> Result<()> {
        for name in &self.execution_order {
            if let Some(plugin) = self.plugins.get(name) {
                hook.execute_with_error(plugin.as_ref(), ctx, error)?;
            }
        }
        Ok(())
    }

    /// Shutdown all plugins
    ///
    /// # Errors
    /// Returns error if any plugin shutdown fails
    pub fn shutdown(&mut self) -> Result<()> {
        for name in self.execution_order.clone() {
            self.unregister(&name)?;
        }
        Ok(())
    }

    /// Set custom execution order for plugins
    ///
    /// # Errors
    /// Returns error if any plugin name in the order is not registered
    pub fn set_execution_order(&mut self, order: Vec<String>) -> Result<()> {
        for name in &order {
            if !self.plugins.contains_key(name) {
                return Err(Error::Config(format!("Plugin '{name}' is not registered")));
            }
        }
        self.execution_order = order;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin {
        name: String,
    }

    impl Plugin for TestPlugin {
        fn name(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn test_registry_new() {
        let registry = PluginRegistry::new();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_register_plugin() {
        let mut registry = PluginRegistry::new();

        let plugin = Box::new(TestPlugin {
            name: "test-plugin".to_string(),
        });

        registry.register(plugin).unwrap();

        assert_eq!(registry.count(), 1);
        assert!(registry.contains("test-plugin"));
    }

    #[test]
    fn test_register_duplicate_fails() {
        let mut registry = PluginRegistry::new();

        let plugin1 = Box::new(TestPlugin {
            name: "test-plugin".to_string(),
        });
        let plugin2 = Box::new(TestPlugin {
            name: "test-plugin".to_string(),
        });

        registry.register(plugin1).unwrap();
        let result = registry.register(plugin2);

        assert!(result.is_err());
    }

    #[test]
    fn test_unregister_plugin() {
        let mut registry = PluginRegistry::new();

        let plugin = Box::new(TestPlugin {
            name: "test-plugin".to_string(),
        });

        registry.register(plugin).unwrap();
        assert_eq!(registry.count(), 1);

        registry.unregister("test-plugin").unwrap();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_get_plugin() {
        let mut registry = PluginRegistry::new();

        let plugin = Box::new(TestPlugin {
            name: "test-plugin".to_string(),
        });

        registry.register(plugin).unwrap();

        let retrieved = registry.get("test-plugin");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name(), "test-plugin");
    }

    #[test]
    fn test_list_plugins() {
        let mut registry = PluginRegistry::new();

        registry
            .register(Box::new(TestPlugin {
                name: "plugin1".to_string(),
            }))
            .unwrap();
        registry
            .register(Box::new(TestPlugin {
                name: "plugin2".to_string(),
            }))
            .unwrap();

        let list = registry.list();
        assert_eq!(list.len(), 2);
        assert!(list.contains(&"plugin1".to_string()));
        assert!(list.contains(&"plugin2".to_string()));
    }

    #[test]
    fn test_execution_order() {
        let mut registry = PluginRegistry::new();

        registry
            .register(Box::new(TestPlugin {
                name: "plugin1".to_string(),
            }))
            .unwrap();
        registry
            .register(Box::new(TestPlugin {
                name: "plugin2".to_string(),
            }))
            .unwrap();

        // Set custom order
        registry
            .set_execution_order(vec!["plugin2".to_string(), "plugin1".to_string()])
            .unwrap();

        let list = registry.list();
        assert_eq!(list, vec!["plugin2", "plugin1"]);
    }

    #[test]
    fn test_execute_hook() {
        let mut registry = PluginRegistry::new();

        registry
            .register(Box::new(TestPlugin {
                name: "test".to_string(),
            }))
            .unwrap();

        let ctx = PluginContext::default();
        let result = registry.execute_hook(PluginHook::PreBuild, &ctx);

        assert!(result.is_ok());
    }
}
