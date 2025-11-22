//! Example: Notification Plugin
//!
//! This example demonstrates how to create a plugin that sends notifications
//! when builds complete or fail.
//!
//! Run with: cargo run --example notification_plugin

use xcargo::error::Result;
use xcargo::plugin::{Plugin, PluginContext, PluginHook, PluginRegistry};

/// A plugin that sends notifications for build events
struct NotificationPlugin;

impl Plugin for NotificationPlugin {
    fn name(&self) -> &str {
        "notification"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "Sends notifications for build events"
    }

    fn author(&self) -> &str {
        "xcargo team"
    }

    fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> {
        println!("🔨 Starting build for target: {}", ctx.target);

        if ctx.release {
            println!("   Mode: Release");
        } else {
            println!("   Mode: Debug");
        }

        if ctx.use_zig {
            println!("   Using Zig toolchain");
        }

        if ctx.use_container {
            println!("   Using container build");
        }

        Ok(())
    }

    fn on_post_build(&self, ctx: &PluginContext) -> Result<()> {
        let build_type = if ctx.release { "release" } else { "debug" };

        println!("\n✅ Build completed successfully!");
        println!("   Target: {}", ctx.target);
        println!("   Type: {}", build_type);

        // In a real implementation, you would send a desktop notification here
        // using crates like notify-rust, mac-notification-sys, or winrt-notification

        Ok(())
    }

    fn on_build_failed(&self, ctx: &PluginContext, error: &str) -> Result<()> {
        println!("\n❌ Build failed!");
        println!("   Target: {}", ctx.target);
        println!("   Error: {}", error);

        // Send failure notification
        Ok(())
    }
}

fn main() -> Result<()> {
    println!("=== Notification Plugin Example ===\n");

    // Create plugin registry
    let mut registry = PluginRegistry::new();

    // Register the notification plugin
    registry.register(Box::new(NotificationPlugin))?;

    // Create a build context
    let ctx = PluginContext::new("x86_64-unknown-linux-gnu".to_string())
        .with_release(true)
        .with_zig(false)
        .with_container(false);

    // Simulate build lifecycle
    println!("Executing pre-build hooks...");
    registry.execute_hook(PluginHook::PreBuild, &ctx)?;

    println!("\n[Simulating build process...]");
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Simulate successful build
    println!("\nExecuting post-build hooks...");
    registry.execute_hook(PluginHook::PostBuild, &ctx)?;

    println!("\n--- Simulating failed build ---\n");

    // Create a new context for Windows target
    let ctx_windows = PluginContext::new("x86_64-pc-windows-gnu".to_string())
        .with_release(false)
        .with_zig(true);

    println!("Executing pre-build hooks...");
    registry.execute_hook(PluginHook::PreBuild, &ctx_windows)?;

    println!("\n[Simulating build failure...]");
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Simulate build failure
    println!("\nExecuting build-failed hooks...");
    registry.execute_hook_with_error(
        PluginHook::BuildFailed,
        &ctx_windows,
        "Linker error: undefined reference to `WinMain`",
    )?;

    // Cleanup
    registry.shutdown()?;

    println!("\n=== Example Complete ===");

    Ok(())
}
