//! Example: Toolchain information and management
//!
//! This example demonstrates how to use the toolchain module to:
//! - List installed toolchains
//! - Get the active toolchain
//! - List installed targets
//! - Prepare targets for cross-compilation
//!
//! Run with: cargo run --example toolchain_info

use xcargo::prelude::*;

fn main() -> xcargo::Result<()> {
    println!("=== Rust Toolchain Information ===\n");

    // Create a toolchain manager
    let manager = ToolchainManager::new()?;

    // Show active toolchain
    println!("Active toolchain:");
    match manager.show_active_toolchain() {
        Ok(active) => println!("  {}\n", active),
        Err(e) => println!("  Error: {}\n", e),
    }

    // List all installed toolchains
    println!("Installed toolchains:");
    match manager.list_toolchains() {
        Ok(toolchains) => {
            if toolchains.is_empty() {
                println!("  No toolchains installed");
            } else {
                for toolchain in &toolchains {
                    let marker = if toolchain.is_default {
                        " (default)"
                    } else {
                        ""
                    };
                    println!("  - {}{}", toolchain.name, marker);
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // Get default toolchain
    println!("Default toolchain:");
    match manager.get_default_toolchain() {
        Ok(Some(toolchain)) => println!("  {}\n", toolchain.name),
        Ok(None) => println!("  No default toolchain set\n"),
        Err(e) => println!("  Error: {}\n", e),
    }

    // List targets for stable toolchain
    println!("Installed targets for 'stable':");
    match manager.list_targets("stable") {
        Ok(targets) => {
            if targets.is_empty() {
                println!("  No targets installed (or stable toolchain not found)");
            } else {
                for target in &targets {
                    println!("  - {}", target);
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // Check if specific target is installed
    let test_target = "x86_64-unknown-linux-gnu";
    println!("Checking if {} is installed:", test_target);
    match manager.is_target_installed("stable", test_target) {
        Ok(installed) => {
            if installed {
                println!("  ✓ Installed\n");
            } else {
                println!("  ✗ Not installed\n");
            }
        }
        Err(e) => println!("  Error: {}\n", e),
    }

    // Get rustup home
    println!("Rustup home directory:");
    match manager.get_rustup_home() {
        Ok(home) => println!("  {}\n", home.display()),
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example: Prepare a target for cross-compilation
    println!("Example: Preparing Windows target for cross-compilation");
    let windows_target = match Target::from_triple("x86_64-pc-windows-gnu") {
        Ok(target) => target,
        Err(e) => {
            println!("  Error parsing target: {}\n", e);
            return Ok(());
        }
    };

    println!("  Target: {}", windows_target.triple);
    println!("  OS: {}", windows_target.os);
    println!("  Architecture: {}", windows_target.arch);

    match manager.is_target_installed("stable", &windows_target.triple) {
        Ok(installed) => {
            if installed {
                println!("  Status: ✓ Already installed");
            } else {
                println!("  Status: ✗ Not installed");
                println!(
                    "  Note: Run `rustup target add {}` to install",
                    windows_target.triple
                );
            }
        }
        Err(e) => println!("  Error checking installation: {}", e),
    }

    Ok(())
}
