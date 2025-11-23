// Linker detection and configuration tests
// Tests the executor's linker handling logic including:
// - Linker detection from config
// - Linker detection from target requirements
// - Linker verification with which::which
// - Platform-specific linker suggestions

use std::fs;
use tempfile::TempDir;
use xcargo::build::{BuildOptions, CargoOperation, Builder};
use xcargo::target::Target;
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
fn test_build_with_host_linker() -> Result<()> {
    let project = create_test_project("test_host_linker")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;
    let options = BuildOptions {
        target: Some(host.triple.clone()),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: true, // Enable verbose to see linker messages
        use_container: false,
        use_zig: Some(false),
        operation: CargoOperation::Check, // Use check for faster test
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // Should succeed with host target
    assert!(result.is_ok(), "Host target check should succeed");
    Ok(())
}

#[test]
fn test_linker_detection_verbose_mode() -> Result<()> {
    let project = create_test_project("test_linker_verbose")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;
    let options = BuildOptions {
        target: Some(host.triple.clone()),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: true, // Verbose mode shows linker detection
        use_container: false,
        use_zig: Some(false),
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // Verbose mode should not cause failures
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_cross_compile_without_linker() -> Result<()> {
    let project = create_test_project("test_no_linker")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;

    // Try to cross-compile to a target that likely doesn't have a linker
    let cross_target = if cfg!(target_os = "macos") {
        "x86_64-unknown-linux-gnu"
    } else if cfg!(target_os = "linux") {
        "x86_64-pc-windows-gnu"
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
        use_zig: Some(false), // Disable Zig to test linker detection
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // May succeed or fail depending on installed linkers
    // Just verify it doesn't panic
    let _ = result;
    Ok(())
}

#[test]
fn test_linker_requirements_linux_target() -> Result<()> {
    let target = Target::from_triple("x86_64-unknown-linux-gnu")?;
    let requirements = target.get_requirements();

    // Linux targets may or may not require specific linkers when building from Linux
    // Just verify we can get requirements without error
    let _ = requirements;
    Ok(())
}

#[test]
fn test_linker_requirements_windows_target() -> Result<()> {
    let target = Target::from_triple("x86_64-pc-windows-gnu")?;
    let requirements = target.get_requirements();

    // Windows GNU targets require MinGW linker
    assert!(requirements.linker.is_some(), "Windows GNU target should require a linker");
    assert!(
        requirements.linker.as_ref().unwrap().contains("mingw"),
        "Windows GNU should suggest mingw linker"
    );
    Ok(())
}

#[test]
fn test_linker_requirements_macos_target() -> Result<()> {
    let target = Target::from_triple("x86_64-apple-darwin")?;
    let requirements = target.get_requirements();

    // macOS targets have specific toolchain needs
    let _ = requirements; // Just verify parsing works
    Ok(())
}

#[test]
fn test_build_with_explicit_toolchain() -> Result<()> {
    let project = create_test_project("test_toolchain")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;
    let options = BuildOptions {
        target: Some(host.triple.clone()),
        release: false,
        cargo_args: vec![],
        toolchain: Some("stable".to_string()),
        verbose: false,
        use_container: false,
        use_zig: Some(false),
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // Should succeed with explicit stable toolchain
    assert!(result.is_ok(), "Explicit toolchain should work");
    Ok(())
}

#[test]
fn test_build_with_nightly_toolchain() -> Result<()> {
    let project = create_test_project("test_nightly")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;
    let options = BuildOptions {
        target: Some(host.triple.clone()),
        release: false,
        cargo_args: vec![],
        toolchain: Some("nightly".to_string()),
        verbose: false,
        use_container: false,
        use_zig: Some(false),
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // May succeed or fail depending on nightly installation
    let _ = result;
    Ok(())
}

#[test]
fn test_linker_path_detection() -> Result<()> {
    // Test that we can detect common linkers in PATH
    let common_linkers = vec!["cc", "gcc", "clang"];

    let mut found_linker = false;
    for linker in &common_linkers {
        if which::which(linker).is_ok() {
            found_linker = true;
            break;
        }
    }

    // At least one common linker should be available on CI
    assert!(found_linker, "Should find at least one common linker");
    Ok(())
}

#[test]
fn test_multiple_targets_with_linkers() -> Result<()> {
    let project = create_test_project("test_multi_linker")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;

    let targets = vec![host.triple.clone()];

    let options = BuildOptions {
        target: None, // Will be set per target
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: false,
        use_container: false,
        use_zig: Some(false),
        operation: CargoOperation::Check,
    };

    let result = builder.build_all(&targets, &options);
    std::env::set_current_dir(original_dir).unwrap();

    assert!(result.is_ok(), "Multi-target build should succeed for host");
    Ok(())
}

#[test]
fn test_target_preparation_with_toolchain() -> Result<()> {
    let builder = Builder::new()?;
    let host = Target::detect_host()?;

    // Verify that toolchain manager can prepare the host target
    // Just verify we can build with explicit toolchain
    let options = BuildOptions {
        target: Some(host.triple.clone()),
        release: false,
        cargo_args: vec![],
        toolchain: Some("stable".to_string()),
        verbose: false,
        use_container: false,
        use_zig: Some(false),
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);

    assert!(result.is_ok(), "Should prepare stable toolchain for host target");
    Ok(())
}

#[test]
fn test_linker_suggestion_macos_to_linux() -> Result<()> {
    let host = Target::from_triple("x86_64-apple-darwin")?;
    let target = Target::from_triple("x86_64-unknown-linux-gnu")?;

    // Just verify we can create these targets for suggestion logic
    // Note: macOS triple has "darwin" as OS, not "macos"
    assert!(host.os == "darwin" || host.os == "macos");
    assert_eq!(target.os, "linux");
    Ok(())
}

#[test]
fn test_linker_suggestion_linux_to_windows() -> Result<()> {
    let host = Target::from_triple("x86_64-unknown-linux-gnu")?;
    let target = Target::from_triple("x86_64-pc-windows-gnu")?;

    assert_eq!(host.os, "linux");
    assert_eq!(target.os, "windows");

    // Verify target requires mingw linker
    let requirements = target.get_requirements();
    assert!(requirements.linker.is_some());
    Ok(())
}
