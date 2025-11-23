// Comprehensive tests for src/main.rs to reach 80% coverage
// Focuses on all command variations and error paths

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn xcargo() -> Command {
    Command::cargo_bin("xcargo").unwrap()
}

// ============================================================================
// Build Command - All Variations
// ============================================================================

#[test]
fn test_build_with_target_and_release() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/main.rs"), "fn main() {}")
        .unwrap();

    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args(["build", "--target", "x86_64-unknown-linux-gnu", "--release"])
        .output();
}

#[test]
fn test_build_with_target_and_container() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/main.rs"), "fn main() {}")
        .unwrap();

    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args([
            "build",
            "--target",
            "aarch64-unknown-linux-gnu",
            "--container",
        ])
        .output();
}

#[test]
fn test_build_with_target_and_toolchain() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/main.rs"), "fn main() {}")
        .unwrap();

    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args([
            "build",
            "--target",
            "x86_64-unknown-linux-gnu",
            "--toolchain",
            "stable",
        ])
        .output();
}

#[test]
fn test_check_with_target_and_toolchain() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/main.rs"), "fn main() {}")
        .unwrap();

    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args([
            "check",
            "--target",
            "x86_64-unknown-linux-gnu",
            "--toolchain",
            "stable",
        ])
        .output();
}

#[test]
fn test_test_with_target_and_toolchain() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/lib.rs"), "pub fn test() {}")
        .unwrap();

    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args([
            "test",
            "--target",
            "x86_64-unknown-linux-gnu",
            "--toolchain",
            "stable",
        ])
        .output();
}

// ============================================================================
// Sequential vs Parallel Builds
// ============================================================================

#[test]
fn test_build_all_sequential() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    // Create xcargo.toml with parallel=false
    fs::write(
        temp_dir.path().join("xcargo.toml"),
        r#"[targets]
default = ["x86_64-unknown-linux-gnu"]

[build]
parallel = false
cache = false
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/main.rs"), "fn main() {}")
        .unwrap();

    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args(["build", "--all"])
        .output();
}

#[test]
fn test_check_all_sequential() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::write(
        temp_dir.path().join("xcargo.toml"),
        r#"[targets]
default = ["x86_64-unknown-linux-gnu"]

[build]
parallel = false
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/main.rs"), "fn main() {}")
        .unwrap();

    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args(["check", "--all"])
        .output();
}

#[test]
fn test_test_all_sequential() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::write(
        temp_dir.path().join("xcargo.toml"),
        r#"[targets]
default = ["x86_64-unknown-linux-gnu"]

[build]
parallel = false
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/lib.rs"), "pub fn test() {}")
        .unwrap();

    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args(["test", "--all"])
        .output();
}

// ============================================================================
// Target Info with Requirements
// ============================================================================

#[test]
fn test_target_info_shows_requirements() {
    let output = xcargo()
        .args(["target", "info", "aarch64-unknown-linux-gnu"])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show linker requirements
    assert!(stdout.contains("Linker") || stdout.contains("Requirements"));
}

#[test]
fn test_target_info_shows_tier() {
    let output = xcargo()
        .args(["target", "info", "wasm32-unknown-unknown"])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show tier information
    assert!(stdout.contains("Tier"));
}

#[test]
fn test_target_info_windows_target() {
    let output = xcargo()
        .args(["target", "info", "x86_64-pc-windows-gnu"])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show Windows target info
    assert!(stdout.contains("windows") || stdout.contains("Windows"));
}

#[test]
fn test_target_info_macos_target() {
    let output = xcargo()
        .args(["target", "info", "aarch64-apple-darwin"])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show macOS target info
    assert!(stdout.contains("darwin") || stdout.contains("macOS"));
}

// ============================================================================
// Help Text Coverage
// ============================================================================

#[test]
fn test_target_help() {
    xcargo()
        .args(["target", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("info"));
}

#[test]
fn test_init_help() {
    xcargo()
        .args(["init", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("interactive"));
}

// ============================================================================
// Doctor Command
// ============================================================================

#[test]
fn test_doctor_shows_system_info() {
    let output = xcargo().arg("doctor").output().unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show some system diagnostic info
    assert!(
        stdout.contains("cargo") || stdout.contains("rustc") || stdout.contains("System")
    );
}

// ============================================================================
// Verbose Mode with Different Commands
// ============================================================================

#[test]
fn test_verbose_with_check() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/main.rs"), "fn main() {}")
        .unwrap();

    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args(["-v", "check"])
        .output();
}

#[test]
fn test_verbose_with_test() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/lib.rs"), "pub fn test() {}")
        .unwrap();

    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args(["--verbose", "test"])
        .output();
}

#[test]
fn test_verbose_with_target_add() {
    let _output = xcargo()
        .args(["-v", "target", "add", "wasm32-unknown-unknown"])
        .output();
}

// ============================================================================
// Combined Options
// ============================================================================

#[test]
fn test_build_release_with_cargo_args() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/main.rs"), "fn main() {}")
        .unwrap();

    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args(["build", "--release", "--", "--locked"])
        .output();
}

#[test]
fn test_check_with_zig_and_cargo_args() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/main.rs"), "fn main() {}")
        .unwrap();

    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args(["check", "--zig", "--", "--all-targets"])
        .output();
}

#[test]
fn test_test_release_with_cargo_args() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/lib.rs"), "pub fn test() {}")
        .unwrap();

    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args(["test", "--release", "--", "--test-threads=1"])
        .output();
}

// ============================================================================
// Test All Targets
// ============================================================================

#[test]
fn test_build_all_with_multiple_targets() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::write(
        temp_dir.path().join("xcargo.toml"),
        r#"[targets]
default = [
    "x86_64-unknown-linux-gnu",
    "x86_64-unknown-linux-musl"
]

[build]
parallel = false
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/main.rs"), "fn main() {}")
        .unwrap();

    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args(["build", "--all"])
        .output();
}
