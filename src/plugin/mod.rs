//! Plugin system for extending xcargo functionality
//!
//! The plugin system allows developers to extend xcargo with custom toolchains,
//! build hooks, and target support through a trait-based architecture.
//!
//! # Plugin Types
//!
//! - **Toolchain Plugins**: Add support for new toolchains (e.g., custom linkers)
//! - **Build Hooks**: Execute code before/after build steps
//! - **Target Plugins**: Add support for new target platforms
//!
//! # Example
//!
//! ```rust,ignore
//! use xcargo::plugin::{Plugin, PluginContext, PluginHook};
//!
//! struct MyPlugin;
//!
//! impl Plugin for MyPlugin {
//!     fn name(&self) -> &str {
//!         "my-plugin"
//!     }
//!
//!     fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> {
//!         println!("Building for target: {}", ctx.target);
//!         Ok(())
//!     }
//! }
//! ```

mod context;
mod hooks;
mod registry;
mod traits;

pub use context::{PluginContext, PluginMetadata};
pub use hooks::PluginHook;
pub use registry::PluginRegistry;
pub use traits::Plugin;

use crate::error::Result;

/// Initialize the plugin system
///
/// This should be called once at application startup to discover and
/// register all available plugins.
pub fn init() -> Result<PluginRegistry> {
    let registry = PluginRegistry::new();

    // Built-in plugins are registered here
    // External plugins will be registered via feature flags or config

    Ok(registry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_init() {
        let registry = init().unwrap();
        assert_eq!(registry.count(), 0); // No plugins registered yet
    }
}
