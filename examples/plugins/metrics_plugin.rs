//! Example: Metrics Collection Plugin
//!
//! This example demonstrates how to create a plugin that collects build metrics
//! and shares data between hooks using the plugin context metadata.
//!
//! Run with: cargo run --example metrics_plugin

use std::sync::Mutex;
use std::time::Instant;
use xcargo::error::Result;
use xcargo::plugin::{Plugin, PluginContext, PluginHook, PluginRegistry};

/// A plugin that collects build metrics
struct MetricsPlugin {
    start_time: Mutex<Option<Instant>>,
    builds_completed: Mutex<usize>,
    builds_failed: Mutex<usize>,
}

impl MetricsPlugin {
    fn new() -> Self {
        Self {
            start_time: Mutex::new(None),
            builds_completed: Mutex::new(0),
            builds_failed: Mutex::new(0),
        }
    }

    fn format_duration(duration: std::time::Duration) -> String {
        let secs = duration.as_secs();
        if secs < 60 {
            format!("{:.2}s", duration.as_secs_f64())
        } else {
            let mins = secs / 60;
            let secs = secs % 60;
            format!("{}m {}s", mins, secs)
        }
    }
}

impl Plugin for MetricsPlugin {
    fn name(&self) -> &str {
        "metrics"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "Collects and reports build metrics"
    }

    fn on_pre_build(&self, ctx: &PluginContext) -> Result<()> {
        *self.start_time.lock().unwrap() = Some(Instant::now());

        println!("📊 Metrics Plugin");
        println!("   Target: {}", ctx.target);
        println!("   Release: {}", ctx.release);

        if let Some(toolchain) = &ctx.toolchain {
            println!("   Toolchain: {}", toolchain);
        }

        if !ctx.cargo_args.is_empty() {
            println!("   Cargo args: {}", ctx.cargo_args.join(" "));
        }

        println!("   Starting timer...");

        Ok(())
    }

    fn on_post_build(&self, _ctx: &PluginContext) -> Result<()> {
        if let Some(start) = *self.start_time.lock().unwrap() {
            let duration = start.elapsed();

            *self.builds_completed.lock().unwrap() += 1;

            println!("\n📈 Build Metrics:");
            println!("   Duration: {}", Self::format_duration(duration));
            println!("   Status: Success ✅");

            let completed = *self.builds_completed.lock().unwrap();
            let failed = *self.builds_failed.lock().unwrap();
            let total = completed + failed;

            if total > 1 {
                println!("\n📊 Session Statistics:");
                println!("   Total builds: {}", total);
                println!("   Successful: {} ({:.1}%)", completed, (completed as f64 / total as f64) * 100.0);
                println!("   Failed: {} ({:.1}%)", failed, (failed as f64 / total as f64) * 100.0);
            }
        }

        Ok(())
    }

    fn on_build_failed(&self, _ctx: &PluginContext, error: &str) -> Result<()> {
        if let Some(start) = *self.start_time.lock().unwrap() {
            let duration = start.elapsed();

            *self.builds_failed.lock().unwrap() += 1;

            println!("\n📈 Build Metrics:");
            println!("   Duration: {}", Self::format_duration(duration));
            println!("   Status: Failed ❌");
            println!("   Error: {}", error);

            let completed = *self.builds_completed.lock().unwrap();
            let failed = *self.builds_failed.lock().unwrap();
            let total = completed + failed;

            if total > 1 {
                println!("\n📊 Session Statistics:");
                println!("   Total builds: {}", total);
                println!("   Successful: {} ({:.1}%)", completed, (completed as f64 / total as f64) * 100.0);
                println!("   Failed: {} ({:.1}%)", failed, (failed as f64 / total as f64) * 100.0);
            }
        }

        Ok(())
    }

    fn on_shutdown(&self) -> Result<()> {
        let completed = *self.builds_completed.lock().unwrap();
        let failed = *self.builds_failed.lock().unwrap();

        println!("\n🏁 Metrics Plugin Shutting Down");
        println!("   Final Statistics:");
        println!("     Successful builds: {}", completed);
        println!("     Failed builds: {}", failed);
        println!("     Total builds: {}", completed + failed);

        Ok(())
    }
}

fn main() -> Result<()> {
    println!("=== Metrics Plugin Example ===\n");

    // Create plugin registry
    let mut registry = PluginRegistry::new();

    // Register the metrics plugin
    registry.register(Box::new(MetricsPlugin::new()))?;

    // Simulate multiple builds
    let targets = vec![
        ("x86_64-unknown-linux-gnu", true, true),
        ("aarch64-unknown-linux-gnu", true, true),
        ("x86_64-pc-windows-gnu", true, false), // This one will "fail"
        ("wasm32-unknown-unknown", false, true),
    ];

    for (target, release, will_succeed) in targets {
        println!("\n{'=':.>60}", "");
        println!("Building: {} ({})", target, if release { "release" } else { "debug" });
        println!("{'=':.>60}\n", "");

        let ctx = PluginContext::new(target.to_string())
            .with_release(release)
            .with_cargo_args(vec!["--features".to_string(), "full".to_string()]);

        // Pre-build hook
        registry.execute_hook(PluginHook::PreBuild, &ctx)?;

        // Simulate build time (random between 1-3 seconds)
        let build_time = 1 + (target.len() % 3);
        std::thread::sleep(std::time::Duration::from_secs(build_time as u64));

        // Post-build hook
        if will_succeed {
            registry.execute_hook(PluginHook::PostBuild, &ctx)?;
        } else {
            registry.execute_hook_with_error(
                PluginHook::BuildFailed,
                &ctx,
                "Compilation error: missing linker",
            )?;
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    // Cleanup
    println!("\n{'=':.>60}", "");
    registry.shutdown()?;

    println!("\n=== Example Complete ===");

    Ok(())
}
