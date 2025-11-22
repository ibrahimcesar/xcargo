//! Plugin trait definitions

use crate::error::Result;

use super::context::PluginContext;

/// Main plugin trait that all plugins must implement
///
/// Plugins can hook into various stages of the build process:
/// - Pre-build: Before compilation starts
/// - Post-build: After compilation completes
/// - Pre-toolchain-install: Before toolchain installation
/// - Post-toolchain-install: After toolchain installation
///
/// # Example
///
/// ```rust,ignore
/// use xcargo::plugin::{Plugin, PluginContext};
///
/// struct NotificationPlugin;
///
/// impl Plugin for NotificationPlugin {
///     fn name(&self) -> &str {
///         "notification-plugin"
///     }
///
///     fn version(&self) -> &str {
///         "1.0.0"
///     }
///
///     fn on_post_build(&self, ctx: &PluginContext) -> Result<()> {
///         println!("Build completed for target: {}", ctx.target);
///         // Send desktop notification, Slack message, etc.
///         Ok(())
///     }
/// }
/// ```
pub trait Plugin: Send + Sync {
    /// Plugin name (must be unique)
    fn name(&self) -> &str;

    /// Plugin version (semantic versioning recommended)
    fn version(&self) -> &str {
        "0.1.0"
    }

    /// Plugin description
    fn description(&self) -> &str {
        ""
    }

    /// Plugin author(s)
    fn author(&self) -> &str {
        ""
    }

    /// Called before the build starts
    ///
    /// Return `Err` to abort the build.
    fn on_pre_build(&self, _ctx: &PluginContext) -> Result<()> {
        Ok(())
    }

    /// Called after the build completes successfully
    fn on_post_build(&self, _ctx: &PluginContext) -> Result<()> {
        Ok(())
    }

    /// Called if the build fails
    fn on_build_failed(&self, _ctx: &PluginContext, _error: &str) -> Result<()> {
        Ok(())
    }

    /// Called before toolchain installation
    ///
    /// Return `Err` to skip toolchain installation.
    fn on_pre_toolchain_install(&self, _ctx: &PluginContext) -> Result<()> {
        Ok(())
    }

    /// Called after toolchain installation
    fn on_post_toolchain_install(&self, _ctx: &PluginContext) -> Result<()> {
        Ok(())
    }

    /// Called when plugin is initialized
    fn on_init(&self) -> Result<()> {
        Ok(())
    }

    /// Called when plugin is unloaded
    fn on_shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin;

    impl Plugin for TestPlugin {
        fn name(&self) -> &str {
            "test-plugin"
        }

        fn version(&self) -> &str {
            "1.0.0"
        }

        fn description(&self) -> &str {
            "A test plugin"
        }
    }

    #[test]
    fn test_plugin_metadata() {
        let plugin = TestPlugin;
        assert_eq!(plugin.name(), "test-plugin");
        assert_eq!(plugin.version(), "1.0.0");
        assert_eq!(plugin.description(), "A test plugin");
    }

    #[test]
    fn test_plugin_hooks_default_impl() {
        let plugin = TestPlugin;
        let ctx = PluginContext::default();

        assert!(plugin.on_pre_build(&ctx).is_ok());
        assert!(plugin.on_post_build(&ctx).is_ok());
        assert!(plugin.on_build_failed(&ctx, "test error").is_ok());
        assert!(plugin.on_init().is_ok());
        assert!(plugin.on_shutdown().is_ok());
    }
}
