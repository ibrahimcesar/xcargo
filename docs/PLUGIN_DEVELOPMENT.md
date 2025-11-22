# Plugin Development Guide

## Overview

xcargo's plugin system allows developers to extend xcargo's functionality through a trait-based architecture. Plugins can hook into various stages of the build process to customize behavior, add new toolchain support, or integrate with external tools.

## Architecture

The plugin system is designed with the following principles:

- **Type Safety**: Compile-time guarantees through Rust's trait system
- **Simplicity**: Easy to implement and maintain
- **Performance**: Minimal overhead with zero-cost abstractions
- **Security**: No dynamic library loading or unsafe code execution

### Plugin Types

xcargo supports several types of plugins:

1. **Build Hooks**: Execute code before/after build steps
2. **Toolchain Plugins**: Add support for custom toolchains and linkers
3. **Target Plugins**: Add support for new target platforms

## Creating a Plugin

### Basic Plugin Structure

All plugins must implement the `Plugin` trait:

```rust
use xcargo::plugin::{Plugin, PluginContext};
use xcargo::error::Result;

struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "my-plugin"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "My custom xcargo plugin"
    }

    fn author(&self) -> &str {
        "Your Name <your.email@example.com>"
    }

    fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> {
        println!("Building for target: {}", ctx.target);
        Ok(())
    }

    fn on_post_build(&self, ctx: &PluginContext) -> Result<()> {
        println!("Build completed successfully!");
        Ok(())
    }
}
```

### Plugin Hooks

Plugins can implement any of the following hooks:

#### Build Lifecycle Hooks

- **`on_pre_build`**: Called before the build starts
  - Return `Err` to abort the build
  - Use for pre-build validation or setup

- **`on_post_build`**: Called after successful build completion
  - Use for post-build tasks like notifications or artifact processing

- **`on_build_failed`**: Called when a build fails
  - Receives error message as parameter
  - Use for error reporting or cleanup

#### Toolchain Hooks

- **`on_pre_toolchain_install`**: Called before toolchain installation
  - Return `Err` to skip installation
  - Use for custom toolchain validation

- **`on_post_toolchain_install`**: Called after toolchain installation
  - Use for toolchain configuration or verification

#### Lifecycle Hooks

- **`on_init`**: Called when plugin is registered
  - Use for one-time initialization

- **`on_shutdown`**: Called when plugin is unloaded
  - Use for cleanup and resource deallocation

### Plugin Context

The `PluginContext` struct provides information about the current build:

```rust
pub struct PluginContext {
    /// Target triple being built (e.g., "x86_64-unknown-linux-gnu")
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

    /// Custom metadata for inter-plugin communication
    pub metadata: HashMap<String, String>,
}
```

## Example Plugins

### 1. Notification Plugin

Send desktop notifications when builds complete:

```rust
use xcargo::plugin::{Plugin, PluginContext};
use xcargo::error::Result;

struct NotificationPlugin;

impl Plugin for NotificationPlugin {
    fn name(&self) -> &str {
        "notification"
    }

    fn on_post_build(&self, ctx: &PluginContext) -> Result<()> {
        let message = format!(
            "Build completed for {} ({})",
            ctx.target,
            if ctx.release { "release" } else { "debug" }
        );

        // Send notification using system notification API
        // notify_rust::Notification::new()
        //     .summary("xcargo Build")
        //     .body(&message)
        //     .show()?;

        println!("📬 {}", message);
        Ok(())
    }

    fn on_build_failed(&self, ctx: &PluginContext, error: &str) -> Result<()> {
        let message = format!("Build failed for {}: {}", ctx.target, error);

        // Send failure notification
        println!("❌ {}", message);
        Ok(())
    }
}
```

### 2. Build Cache Validator

Validate build cache before compilation:

```rust
use xcargo::plugin::{Plugin, PluginContext};
use xcargo::error::{Error, Result};
use xcargo::cache::BuildCache;

struct CacheValidatorPlugin {
    cache: BuildCache,
}

impl CacheValidatorPlugin {
    fn new() -> Result<Self> {
        Ok(Self {
            cache: BuildCache::new()?,
        })
    }
}

impl Plugin for CacheValidatorPlugin {
    fn name(&self) -> &str {
        "cache-validator"
    }

    fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> {
        // Check if cache exists for target
        if let Some(entry) = self.cache.get(&ctx.target) {
            println!("Using cached build from {}", entry.timestamp);
        } else {
            println!("No cache found, building from scratch");
        }
        Ok(())
    }
}
```

### 3. Custom Linker Plugin

Configure custom linker for specific targets:

```rust
use xcargo::plugin::{Plugin, PluginContext};
use xcargo::error::Result;
use std::env;

struct CustomLinkerPlugin;

impl Plugin for CustomLinkerPlugin {
    fn name(&self) -> &str {
        "custom-linker"
    }

    fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> {
        // Set custom linker for Windows targets
        if ctx.target.contains("windows") {
            env::set_var("CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER", "x86_64-w64-mingw32-gcc");
            println!("Using custom Windows linker");
        }
        Ok(())
    }
}
```

### 4. Metrics Collection Plugin

Collect build metrics and statistics:

```rust
use xcargo::plugin::{Plugin, PluginContext};
use xcargo::error::Result;
use std::time::Instant;

struct MetricsPlugin {
    start_time: std::sync::Mutex<Option<Instant>>,
}

impl MetricsPlugin {
    fn new() -> Self {
        Self {
            start_time: std::sync::Mutex::new(None),
        }
    }
}

impl Plugin for MetricsPlugin {
    fn name(&self) -> &str {
        "metrics"
    }

    fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> {
        *self.start_time.lock().unwrap() = Some(Instant::now());
        println!("📊 Starting build for {}", ctx.target);
        Ok(())
    }

    fn on_post_build(&self, ctx: &PluginContext) -> Result<()> {
        if let Some(start) = *self.start_time.lock().unwrap() {
            let duration = start.elapsed();
            println!("✅ Build completed in {:.2}s", duration.as_secs_f64());

            // Store metrics in context for other plugins
            // ctx.set_metadata("build_duration".to_string(), duration.as_secs().to_string());
        }
        Ok(())
    }

    fn on_build_failed(&self, _ctx: &PluginContext, error: &str) -> Result<()> {
        if let Some(start) = *self.start_time.lock().unwrap() {
            let duration = start.elapsed();
            println!("❌ Build failed after {:.2}s: {}", duration.as_secs_f64(), error);
        }
        Ok(())
    }
}
```

## Registering Plugins

### Manual Registration

```rust
use xcargo::plugin::PluginRegistry;

fn main() -> Result<()> {
    let mut registry = PluginRegistry::new();

    // Register plugins
    registry.register(Box::new(NotificationPlugin))?;
    registry.register(Box::new(MetricsPlugin::new()))?;
    registry.register(Box::new(CustomLinkerPlugin))?;

    // Execute build hooks
    let ctx = PluginContext::new("x86_64-unknown-linux-gnu".to_string())
        .with_release(true);

    registry.execute_hook(PluginHook::PreBuild, &ctx)?;

    // ... perform build ...

    registry.execute_hook(PluginHook::PostBuild, &ctx)?;

    Ok(())
}
```

### Configuration-Based Registration

Plugins can be configured in `xcargo.toml` (future feature):

```toml
[plugins]
enabled = true

[[plugins.plugin]]
name = "notification"
enabled = true

[[plugins.plugin]]
name = "metrics"
enabled = true
config = { verbose = true }
```

## Plugin Execution Order

Plugins execute in the order they were registered. You can customize execution order:

```rust
let mut registry = PluginRegistry::new();

registry.register(Box::new(PluginA))?;
registry.register(Box::new(PluginB))?;
registry.register(Box::new(PluginC))?;

// Set custom execution order
registry.set_execution_order(vec![
    "plugin-c".to_string(),
    "plugin-a".to_string(),
    "plugin-b".to_string(),
])?;
```

## Error Handling

Plugins should return `Result<()>` from all hook methods:

- **`Ok(())`**: Hook executed successfully, continue execution
- **`Err(e)`**: Hook failed, stop execution and propagate error

Example:

```rust
fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> {
    if ctx.target.contains("unsupported") {
        return Err(Error::Config(
            "This plugin doesn't support this target".to_string()
        ));
    }
    Ok(())
}
```

## Best Practices

1. **Keep plugins focused**: Each plugin should do one thing well
2. **Minimize side effects**: Avoid modifying global state unless necessary
3. **Handle errors gracefully**: Return meaningful error messages
4. **Document configuration**: Clearly document any configuration options
5. **Test thoroughly**: Write tests for all plugin hooks
6. **Use metadata**: Share state between plugins via `PluginContext::metadata`
7. **Be efficient**: Plugins run on every build, keep them fast

## Testing Plugins

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_pre_build() {
        let plugin = MyPlugin;
        let ctx = PluginContext::new("x86_64-unknown-linux-gnu".to_string());

        let result = plugin.on_pre_build(&ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_plugin_initialization() {
        let plugin = MyPlugin;
        assert_eq!(plugin.name(), "my-plugin");
        assert_eq!(plugin.version(), "1.0.0");
    }
}
```

## Future Enhancements

The plugin system is designed to evolve. Planned features include:

1. **Dynamic Plugin Loading**: Support for loading plugins from shared libraries
2. **Plugin Marketplace**: Discover and install community plugins
3. **Plugin Dependencies**: Declare dependencies between plugins
4. **Configuration Schema**: JSON Schema validation for plugin configs
5. **Plugin Sandboxing**: Security boundaries for untrusted plugins

## API Reference

See the [API documentation](https://docs.rs/xcargo/latest/xcargo/plugin/) for complete reference.

## Contributing

Want to contribute a plugin? Check out our [Contributing Guide](../CONTRIBUTING.md) and submit a pull request!

## License

All plugins should be compatible with xcargo's MIT license.
