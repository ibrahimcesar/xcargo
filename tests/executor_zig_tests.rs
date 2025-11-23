// Zig cross-compilation integration tests
// Tests the executor's Zig toolchain logic including:
// - Zig auto-detection for cross-OS compilation
// - Explicit --zig flag
// - Explicit --no-zig flag
// - Zig target support detection
// - Fallback when Zig unavailable

use std::fs;
use tempfile::TempDir;
use xcargo::build::{BuildOptions, CargoOperation, Builder};
use xcargo::target::Target;
use xcargo::toolchain::zig::ZigToolchain;
use xcargo::Result;

fn create_test_project(name: &str) -> Result<TempDir> {
    let temp_dir = TempDir::new()?;
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
"#,
        name
    );
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml)?;
    fs::create_dir(temp_dir.path().join("src"))?;
    fs::write(
        temp_dir.path().join("src/main.rs"),
        "fn main() { println!(\"Hello, world!\"); }",
    )?;
    Ok(temp_dir)
}

#[test]
fn test_zig_disabled_via_flag() -> Result<()> {
    let project = create_test_project("test_zig_disabled")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;
    let options = BuildOptions {
        target: Some(host.triple.clone()),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: true, // See Zig disabled message
        use_container: false,
        use_zig: Some(false), // Explicitly disable Zig
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // Should succeed without Zig
    assert!(result.is_ok(), "Build with --no-zig should succeed");
    Ok(())
}

#[test]
fn test_zig_auto_mode_same_os() -> Result<()> {
    let project = create_test_project("test_zig_auto_same")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;
    let options = BuildOptions {
        target: Some(host.triple.clone()),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: true,
        use_container: false,
        use_zig: None, // Auto mode - should NOT use Zig for same OS
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // Should succeed without Zig for same OS
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_zig_forced_flag() -> Result<()> {
    let project = create_test_project("test_zig_forced")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;
    let options = BuildOptions {
        target: Some(host.triple.clone()),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: true,
        use_container: false,
        use_zig: Some(true), // Force Zig even for same OS
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // May succeed if Zig installed and supports target, or fail gracefully
    // Should not panic
    let _ = result;
    Ok(())
}

#[test]
fn test_zig_cross_os_auto_detection() -> Result<()> {
    let project = create_test_project("test_zig_cross_auto")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;

    // Pick a different OS target
    let cross_target = if cfg!(target_os = "macos") {
        "x86_64-unknown-linux-gnu"
    } else if cfg!(target_os = "linux") {
        "aarch64-unknown-linux-gnu" // Stay on Linux but different arch
    } else {
        "x86_64-unknown-linux-gnu"
    };

    let options = BuildOptions {
        target: Some(cross_target.to_string()),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: true,
        use_container: false,
        use_zig: None, // Auto mode - should TRY to use Zig for cross-OS
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // May succeed or fail depending on Zig availability
    let _ = result;
    Ok(())
}

#[test]
fn test_zig_target_support_x86_64_linux() {
    let supports = ZigToolchain::supports_target_name("x86_64-unknown-linux-gnu");
    assert!(supports, "Zig should support x86_64-unknown-linux-gnu");
}

#[test]
fn test_zig_target_support_aarch64_linux() {
    let supports = ZigToolchain::supports_target_name("aarch64-unknown-linux-gnu");
    assert!(supports, "Zig should support aarch64-unknown-linux-gnu");
}

#[test]
fn test_zig_target_support_armv7_linux() {
    let supports = ZigToolchain::supports_target_name("armv7-unknown-linux-gnueabihf");
    assert!(supports, "Zig should support armv7-unknown-linux-gnueabihf");
}

#[test]
fn test_zig_target_no_support_windows() {
    let supports = ZigToolchain::supports_target_name("x86_64-pc-windows-msvc");
    assert!(!supports, "Zig should not support Windows MSVC targets");
}

#[test]
fn test_zig_target_no_support_macos() {
    let supports = ZigToolchain::supports_target_name("x86_64-apple-darwin");
    assert!(!supports, "Zig should not support macOS targets");
}

#[test]
fn test_zig_availability_check() {
    // Just test that we can check for Zig without panicking
    let zig_available = which::which("zig").is_ok();
    // Don't assert - Zig may or may not be installed
    let _ = zig_available;
}

#[test]
fn test_zig_with_verbose_output() -> Result<()> {
    let project = create_test_project("test_zig_verbose")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;

    // Use a Zig-supported target
    let options = BuildOptions {
        target: Some("x86_64-unknown-linux-gnu".to_string()),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: true, // Should show Zig detection messages
        use_container: false,
        use_zig: Some(true), // Try to use Zig
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // May succeed or fail, but verbose mode should not cause crashes
    let _ = result;
    Ok(())
}

#[test]
fn test_zig_unsupported_target_with_force() -> Result<()> {
    let project = create_test_project("test_zig_unsupported")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;

    // Try to force Zig for an unsupported target
    let options = BuildOptions {
        target: Some("x86_64-pc-windows-msvc".to_string()),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: true,
        use_container: false,
        use_zig: Some(true), // Force Zig for unsupported target
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // Should fail gracefully if Zig is installed
    // Should fail with toolchain error if Zig not installed
    if which::which("zig").is_ok() {
        assert!(result.is_err(), "Should fail for unsupported Zig target");
    }
    Ok(())
}

#[test]
fn test_zig_with_release_build() -> Result<()> {
    let project = create_test_project("test_zig_release")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;

    let options = BuildOptions {
        target: Some("x86_64-unknown-linux-gnu".to_string()),
        release: true, // Release mode
        cargo_args: vec![],
        toolchain: None,
        verbose: false,
        use_container: false,
        use_zig: Some(true),
        operation: CargoOperation::Build, // Full build
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // May succeed or fail depending on Zig
    let _ = result;
    Ok(())
}

#[test]
fn test_zig_detection_with_different_operations() -> Result<()> {
    let project = create_test_project("test_zig_ops")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;

    let operations = vec![
        CargoOperation::Build,
        CargoOperation::Check,
        CargoOperation::Test,
    ];

    for op in operations {
        let options = BuildOptions {
            target: Some("x86_64-unknown-linux-gnu".to_string()),
            release: false,
            cargo_args: vec![],
            toolchain: None,
            verbose: false,
            use_container: false,
            use_zig: None, // Auto mode
            operation: op,
        };

        let result = builder.build(&options);
        // Just verify it doesn't panic
        let _ = result;
    }

    std::env::set_current_dir(original_dir).unwrap();
    Ok(())
}
