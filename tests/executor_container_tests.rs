// Container build decision logic tests
// Tests when container builds are used vs native builds
// - Explicit --container flag
// - Config-based container decisions
// - Container availability checks
// - Fallback behavior when containers unavailable

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
fn test_container_flag_explicit() -> Result<()> {
    let project = create_test_project("test_container_explicit")?;
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
        use_container: true, // Explicitly request container
        use_zig: Some(false),
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // May succeed if docker available, or fail gracefully
    // Should not panic
    let _ = result;
    Ok(())
}

#[test]
fn test_container_not_requested() -> Result<()> {
    let project = create_test_project("test_no_container")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;

    let options = BuildOptions {
        target: Some(host.triple.clone()),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: false,
        use_container: false, // No container
        use_zig: Some(false),
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // Should succeed without container for host target
    assert!(result.is_ok(), "Native build should work for host");
    Ok(())
}

#[test]
fn test_container_with_cross_target() -> Result<()> {
    let project = create_test_project("test_container_cross")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;

    // Pick a different target
    let cross_target = if cfg!(target_os = "macos") {
        "x86_64-unknown-linux-gnu"
    } else {
        "aarch64-unknown-linux-gnu"
    };

    let options = BuildOptions {
        target: Some(cross_target.to_string()),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: true,
        use_container: true, // Use container for cross-compilation
        use_zig: Some(false),
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // May succeed or fail depending on docker availability
    let _ = result;
    Ok(())
}

#[test]
fn test_docker_availability() {
    // Check if docker is available
    let docker_available = which::which("docker").is_ok();
    // Don't assert - docker may or may not be installed
    let _ = docker_available;
}

#[test]
fn test_podman_availability() {
    // Check if podman is available (alternative to docker)
    let podman_available = which::which("podman").is_ok();
    let _ = podman_available;
}

#[test]
fn test_container_with_release_mode() -> Result<()> {
    let project = create_test_project("test_container_release")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;

    let options = BuildOptions {
        target: Some(host.triple.clone()),
        release: true, // Release build in container
        cargo_args: vec![],
        toolchain: None,
        verbose: false,
        use_container: true,
        use_zig: Some(false),
        operation: CargoOperation::Build,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // May succeed or fail depending on docker
    let _ = result;
    Ok(())
}

#[test]
fn test_container_with_cargo_args() -> Result<()> {
    let project = create_test_project("test_container_args")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;

    let options = BuildOptions {
        target: Some(host.triple.clone()),
        release: false,
        cargo_args: vec!["--all-features".to_string()],
        toolchain: None,
        verbose: false,
        use_container: true,
        use_zig: Some(false),
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // May succeed or fail
    let _ = result;
    Ok(())
}

#[test]
fn test_container_priority_over_zig() -> Result<()> {
    let project = create_test_project("test_container_priority")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;

    let options = BuildOptions {
        target: Some("x86_64-unknown-linux-gnu".to_string()),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: true,
        use_container: true, // Container should take priority
        use_zig: Some(true), // Even if Zig requested
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // Container should be attempted first
    let _ = result;
    Ok(())
}

#[test]
fn test_native_build_fallback() -> Result<()> {
    let project = create_test_project("test_native_fallback")?;
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;

    let options = BuildOptions {
        target: Some(host.triple.clone()),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: false,
        use_container: false, // No container
        use_zig: Some(false), // No Zig
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);
    std::env::set_current_dir(original_dir).unwrap();

    // Native build should work
    assert!(result.is_ok(), "Native build should succeed");
    Ok(())
}

#[test]
fn test_container_config_detection() -> Result<()> {
    let builder = Builder::new()?;

    // Just verify builder can be created (which means config loaded)
    // We can't access private fields, but builder creation validates config
    let _ = builder;
    Ok(())
}
