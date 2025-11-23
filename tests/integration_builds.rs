// Integration tests that exercise real build execution paths
// These tests create temporary Rust projects and run actual builds

use std::fs;
use tempfile::TempDir;
use xcargo::build::{BuildOptions, Builder, CargoOperation};
use xcargo::error::Result;
use xcargo::target::Target;

/// Helper to create a minimal Rust project in a temp directory
fn create_test_project(name: &str) -> Result<TempDir> {
    let temp_dir = TempDir::new()?;

    // Create Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
        name
    );
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml)?;

    // Create src directory
    fs::create_dir(temp_dir.path().join("src"))?;

    // Create minimal main.rs
    let main_rs = r#"fn main() {
    println!("Hello, world!");
}
"#;
    fs::write(temp_dir.path().join("src/main.rs"), main_rs)?;

    Ok(temp_dir)
}

/// Helper to create a library project
fn create_test_library(name: &str) -> Result<TempDir> {
    let temp_dir = TempDir::new()?;

    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
"#,
        name
    );
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml)?;

    fs::create_dir(temp_dir.path().join("src"))?;

    let lib_rs = r#"pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
"#;
    fs::write(temp_dir.path().join("src/lib.rs"), lib_rs)?;

    Ok(temp_dir)
}

#[test]
fn test_build_host_target() -> Result<()> {
    let project = create_test_project("test_host_build")?;
    let original_dir = std::env::current_dir().unwrap();

    // Change to project directory
    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;

    let options = BuildOptions {
        target: Some(host.triple.clone()),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: false,
        use_container: false,
        use_zig: None,
        operation: CargoOperation::Build,
    };

    // This should succeed for the host target
    let result = builder.build(&options);

    // Restore directory
    std::env::set_current_dir(original_dir).unwrap();

    // Check that build succeeded
    assert!(result.is_ok(), "Host target build should succeed");
    Ok(())
}

#[test]
fn test_check_operation() -> Result<()> {
    let project = create_test_library("test_check")?;
    let original_dir = std::env::current_dir().unwrap();

    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;

    let options = BuildOptions {
        target: Some(host.triple),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: false,
        use_container: false,
        use_zig: None,
        operation: CargoOperation::Check,
    };

    let result = builder.build(&options);

    std::env::set_current_dir(original_dir).unwrap();

    assert!(result.is_ok(), "Check operation should succeed");
    Ok(())
}

#[test]
fn test_test_operation() -> Result<()> {
    let project = create_test_library("test_tests")?;
    let original_dir = std::env::current_dir().unwrap();

    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;

    let options = BuildOptions {
        target: Some(host.triple),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: false,
        use_container: false,
        use_zig: None,
        operation: CargoOperation::Test,
    };

    let result = builder.build(&options);

    std::env::set_current_dir(original_dir).unwrap();

    assert!(result.is_ok(), "Test operation should succeed");
    Ok(())
}

#[test]
fn test_release_build() -> Result<()> {
    let project = create_test_project("test_release")?;
    let original_dir = std::env::current_dir().unwrap();

    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;

    let options = BuildOptions {
        target: Some(host.triple.clone()),
        release: true, // Release mode
        cargo_args: vec![],
        toolchain: None,
        verbose: false,
        use_container: false,
        use_zig: None,
        operation: CargoOperation::Build,
    };

    let result = builder.build(&options);

    std::env::set_current_dir(original_dir).unwrap();

    assert!(result.is_ok(), "Release build should succeed");

    // Verify release artifact exists
    let release_dir = project.path().join("target").join(&host.triple).join("release");
    assert!(
        release_dir.exists() || project.path().join("target/release").exists(),
        "Release directory should exist"
    );

    Ok(())
}

#[test]
fn test_verbose_build() -> Result<()> {
    let project = create_test_project("test_verbose")?;
    let original_dir = std::env::current_dir().unwrap();

    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;

    let options = BuildOptions {
        target: Some(host.triple),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: true, // Verbose output
        use_container: false,
        use_zig: None,
        operation: CargoOperation::Build,
    };

    let result = builder.build(&options);

    std::env::set_current_dir(original_dir).unwrap();

    assert!(result.is_ok(), "Verbose build should succeed");
    Ok(())
}

#[test]
fn test_build_with_cargo_args() -> Result<()> {
    let project = create_test_library("test_cargo_args")?;
    let original_dir = std::env::current_dir().unwrap();

    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;

    let options = BuildOptions {
        target: Some(host.triple),
        release: false,
        cargo_args: vec!["--lib".to_string()], // Build only the library
        toolchain: None,
        verbose: false,
        use_container: false,
        use_zig: None,
        operation: CargoOperation::Build,
    };

    let result = builder.build(&options);

    std::env::set_current_dir(original_dir).unwrap();

    assert!(result.is_ok(), "Build with cargo args should succeed");
    Ok(())
}

#[test]
fn test_missing_cargo_toml() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();

    // Empty directory with no Cargo.toml
    std::env::set_current_dir(temp_dir.path()).unwrap();

    let builder = Builder::new()?;
    let host = Target::detect_host()?;

    let options = BuildOptions {
        target: Some(host.triple),
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: false,
        use_container: false,
        use_zig: None,
        operation: CargoOperation::Build,
    };

    let result = builder.build(&options);

    std::env::set_current_dir(original_dir).unwrap();

    // Should fail with config error
    assert!(result.is_err(), "Build without Cargo.toml should fail");
    assert!(
        matches!(result.unwrap_err(), xcargo::error::Error::Config(_)),
        "Error should be Config type"
    );

    Ok(())
}

#[test]
fn test_build_no_target_specified() -> Result<()> {
    let project = create_test_project("test_no_target")?;
    let original_dir = std::env::current_dir().unwrap();

    std::env::set_current_dir(project.path()).unwrap();

    let builder = Builder::new()?;

    let options = BuildOptions {
        target: None, // No target specified - should use host
        release: false,
        cargo_args: vec![],
        toolchain: None,
        verbose: false,
        use_container: false,
        use_zig: None,
        operation: CargoOperation::Build,
    };

    let result = builder.build(&options);

    std::env::set_current_dir(original_dir).unwrap();

    assert!(result.is_ok(), "Build with no target should use host target");
    Ok(())
}
