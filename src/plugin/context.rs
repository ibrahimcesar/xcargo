//! Plugin execution context

use std::collections::HashMap;
use std::path::PathBuf;

/// Context passed to plugin hooks
///
/// Contains information about the current build, target, and environment.
#[derive(Debug, Clone, Default)]
pub struct PluginContext {
    /// Target triple being built
    pub target: String,

    /// Whether this is a release build
    pub release: bool,

    /// Project root directory
    pub project_root: PathBuf,

    /// Additional cargo arguments
    pub cargo_args: Vec<String>,

    /// Toolchain being used (if any)
    pub toolchain: Option<String>,

    /// Whether using container build
    pub use_container: bool,

    /// Whether using Zig for cross-compilation
    pub use_zig: bool,

    /// Custom metadata that plugins can use to share state
    pub metadata: HashMap<String, String>,
}

impl PluginContext {
    /// Create a new plugin context
    #[must_use]
    pub fn new(target: String) -> Self {
        Self {
            target,
            release: false,
            project_root: PathBuf::new(),
            cargo_args: Vec::new(),
            toolchain: None,
            use_container: false,
            use_zig: false,
            metadata: HashMap::new(),
        }
    }

    /// Set release mode
    #[must_use]
    pub fn with_release(mut self, release: bool) -> Self {
        self.release = release;
        self
    }

    /// Set project root
    #[must_use]
    pub fn with_project_root(mut self, root: PathBuf) -> Self {
        self.project_root = root;
        self
    }

    /// Add cargo arguments
    #[must_use]
    pub fn with_cargo_args(mut self, args: Vec<String>) -> Self {
        self.cargo_args = args;
        self
    }

    /// Set toolchain
    #[must_use]
    pub fn with_toolchain(mut self, toolchain: Option<String>) -> Self {
        self.toolchain = toolchain;
        self
    }

    /// Set container usage
    #[must_use]
    pub fn with_container(mut self, use_container: bool) -> Self {
        self.use_container = use_container;
        self
    }

    /// Set Zig usage
    #[must_use]
    pub fn with_zig(mut self, use_zig: bool) -> Self {
        self.use_zig = use_zig;
        self
    }

    /// Add metadata entry
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata entry
    #[must_use]
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

/// Plugin metadata
#[derive(Debug, Clone)]
pub struct PluginMetadata {
    /// Plugin name
    pub name: String,

    /// Plugin version
    pub version: String,

    /// Plugin description
    pub description: String,

    /// Plugin author
    pub author: String,

    /// Whether plugin is enabled
    pub enabled: bool,
}

impl PluginMetadata {
    /// Create new plugin metadata
    #[must_use]
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            description: String::new(),
            author: String::new(),
            enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_context_builder() {
        let ctx = PluginContext::new("x86_64-unknown-linux-gnu".to_string())
            .with_release(true)
            .with_project_root(PathBuf::from("/project"))
            .with_cargo_args(vec!["--features".to_string(), "full".to_string()])
            .with_toolchain(Some("stable".to_string()))
            .with_container(true)
            .with_zig(false);

        assert_eq!(ctx.target, "x86_64-unknown-linux-gnu");
        assert!(ctx.release);
        assert_eq!(ctx.project_root, PathBuf::from("/project"));
        assert_eq!(ctx.cargo_args.len(), 2);
        assert_eq!(ctx.toolchain, Some("stable".to_string()));
        assert!(ctx.use_container);
        assert!(!ctx.use_zig);
    }

    #[test]
    fn test_plugin_context_metadata() {
        let mut ctx = PluginContext::new("test-target".to_string());

        ctx.set_metadata("key1".to_string(), "value1".to_string());
        ctx.set_metadata("key2".to_string(), "value2".to_string());

        assert_eq!(ctx.get_metadata("key1"), Some(&"value1".to_string()));
        assert_eq!(ctx.get_metadata("key2"), Some(&"value2".to_string()));
        assert_eq!(ctx.get_metadata("nonexistent"), None);
    }

    #[test]
    fn test_plugin_metadata_creation() {
        let metadata = PluginMetadata::new("test-plugin".to_string(), "1.0.0".to_string());

        assert_eq!(metadata.name, "test-plugin");
        assert_eq!(metadata.version, "1.0.0");
        assert!(metadata.enabled);
    }
}
