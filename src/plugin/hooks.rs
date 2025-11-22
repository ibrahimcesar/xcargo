//! Plugin hook execution

use crate::error::Result;

use super::context::PluginContext;
use super::traits::Plugin;

/// Plugin hook execution points
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginHook {
    /// Before build starts
    PreBuild,
    /// After build completes
    PostBuild,
    /// When build fails
    BuildFailed,
    /// Before toolchain installation
    PreToolchainInstall,
    /// After toolchain installation
    PostToolchainInstall,
    /// On plugin initialization
    Init,
    /// On plugin shutdown
    Shutdown,
}

impl PluginHook {
    /// Get hook name as string
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::PreBuild => "pre-build",
            Self::PostBuild => "post-build",
            Self::BuildFailed => "build-failed",
            Self::PreToolchainInstall => "pre-toolchain-install",
            Self::PostToolchainInstall => "post-toolchain-install",
            Self::Init => "init",
            Self::Shutdown => "shutdown",
        }
    }

    /// Execute this hook on a plugin
    ///
    /// # Errors
    /// Returns error if the plugin hook fails
    pub fn execute(&self, plugin: &dyn Plugin, ctx: &PluginContext) -> Result<()> {
        match self {
            Self::PreBuild => plugin.on_pre_build(ctx),
            Self::PostBuild => plugin.on_post_build(ctx),
            Self::BuildFailed => {
                // For build failed, we need an error message
                // This is just a placeholder - actual error will be passed in
                plugin.on_build_failed(ctx, "build failed")
            }
            Self::PreToolchainInstall => plugin.on_pre_toolchain_install(ctx),
            Self::PostToolchainInstall => plugin.on_post_toolchain_install(ctx),
            Self::Init => plugin.on_init(),
            Self::Shutdown => plugin.on_shutdown(),
        }
    }

    /// Execute hook with error message (for BuildFailed hook)
    ///
    /// # Errors
    /// Returns error if the plugin hook fails
    pub fn execute_with_error(
        &self,
        plugin: &dyn Plugin,
        ctx: &PluginContext,
        error: &str,
    ) -> Result<()> {
        match self {
            Self::BuildFailed => plugin.on_build_failed(ctx, error),
            _ => self.execute(plugin, ctx),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_as_str() {
        assert_eq!(PluginHook::PreBuild.as_str(), "pre-build");
        assert_eq!(PluginHook::PostBuild.as_str(), "post-build");
        assert_eq!(PluginHook::BuildFailed.as_str(), "build-failed");
        assert_eq!(
            PluginHook::PreToolchainInstall.as_str(),
            "pre-toolchain-install"
        );
        assert_eq!(
            PluginHook::PostToolchainInstall.as_str(),
            "post-toolchain-install"
        );
        assert_eq!(PluginHook::Init.as_str(), "init");
        assert_eq!(PluginHook::Shutdown.as_str(), "shutdown");
    }

    struct TestPlugin {
        pre_build_called: std::sync::Arc<std::sync::Mutex<bool>>,
    }

    impl Plugin for TestPlugin {
        fn name(&self) -> &str {
            "test"
        }

        fn on_pre_build(&self, _ctx: &PluginContext) -> Result<()> {
            *self.pre_build_called.lock().unwrap() = true;
            Ok(())
        }
    }

    #[test]
    fn test_hook_execute() {
        let called = std::sync::Arc::new(std::sync::Mutex::new(false));
        let plugin = TestPlugin {
            pre_build_called: called.clone(),
        };
        let ctx = PluginContext::default();

        PluginHook::PreBuild.execute(&plugin, &ctx).unwrap();

        assert!(*called.lock().unwrap());
    }
}
