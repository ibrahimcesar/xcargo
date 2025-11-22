//! Example: Output formatting and user messaging
//!
//! This example demonstrates the delightful output formatting
//! capabilities of xcargo, including tips, hints, and colored messages.
//!
//! Run with: cargo run --example output_demo

use xcargo::output::{helpers, tips, Message, MessageType};

fn main() {
    println!("\n=== xcargo Output Demo ===\n");

    // Section headers
    helpers::section("Message Types");

    // Different message types
    helpers::success("Build completed successfully!");
    helpers::error("Failed to compile target");
    helpers::warning("Using fallback container runtime");
    helpers::info("Detected 4 available targets");
    helpers::progress("Installing target x86_64-pc-windows-gnu...");
    println!();

    // Tips and hints
    helpers::section("Tips & Hints");
    helpers::tip(tips::INSTALL_TARGET);
    helpers::tip(tips::LIST_TARGETS);
    helpers::tip(tips::CONFIG_FILE);
    helpers::hint("Try 'xcargo --help' for more commands");
    println!();

    // Custom messages
    helpers::section("Custom Workflows");

    // Simulating a build workflow
    helpers::progress("Detecting host platform...");
    helpers::info("Host: aarch64-apple-darwin");
    println!();

    helpers::progress("Checking toolchain...");
    helpers::success("Toolchain 'stable' is installed");
    println!();

    helpers::progress("Installing target...");
    helpers::warning("Target not found in cache");
    helpers::progress("Downloading target from rustup...");
    helpers::success("Target installed successfully");
    println!();

    helpers::tip(tips::NATIVE_BUILDS);
    helpers::tip(tips::PARALLEL_BUILDS);
    println!();

    // Manual message creation
    helpers::section("Advanced Usage");

    let msg = Message::new(
        MessageType::Tip,
        "You can create custom messages with the Message API",
    );
    msg.print();

    let msg = Message::hint("Configure build profiles for different scenarios");
    msg.print();
    println!();

    // Performance tips
    helpers::section("Performance Tips");
    helpers::tip(tips::BUILD_CACHE);
    helpers::tip(tips::PARALLEL_BUILDS);
    helpers::tip(tips::NATIVE_BUILDS);
    println!();

    // Configuration tips
    helpers::section("Configuration");
    helpers::tip(tips::CONFIG_FILE);
    helpers::tip(tips::BUILD_PROFILES);
    helpers::tip(tips::CONTAINER_BUILDS);
    println!();

    helpers::success("Demo complete! xcargo makes cross-compilation delightful ✨");
    println!();
}
