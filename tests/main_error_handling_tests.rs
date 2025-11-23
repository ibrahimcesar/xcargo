// Tests for src/main.rs error handling and edge cases
// Focuses on uncovered error paths and interactive flows

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn xcargo() -> Command {
    Command::cargo_bin("xcargo").unwrap()
}

// ============================================================================
// Build Command Error Handling
// ============================================================================

#[test]
fn test_build_all_with_no_default_targets() {
    let temp_dir = TempDir::new().unwrap();

    // Create valid Cargo.toml
    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    // Create empty xcargo.toml with no default targets
    fs::write(
        temp_dir.path().join("xcargo.toml"),
        r#"[targets]
default = []

[build]
parallel = false
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/main.rs"), "fn main() {}")
        .unwrap();

    // Should fail with error about no default targets
    xcargo()
        .current_dir(temp_dir.path())
        .args(["build", "--all"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("No default targets"));
}

#[test]
fn test_check_all_with_no_default_targets() {
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
default = []
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/main.rs"), "fn main() {}")
        .unwrap();

    xcargo()
        .current_dir(temp_dir.path())
        .args(["check", "--all"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("No default targets"));
}

#[test]
fn test_test_all_with_no_default_targets() {
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
default = []
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/lib.rs"), "pub fn test() {}")
        .unwrap();

    xcargo()
        .current_dir(temp_dir.path())
        .args(["test", "--all"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("No default targets"));
}

// ============================================================================
// Config Command Error Handling
// ============================================================================

#[test]
fn test_config_with_existing_file() {
    let temp_dir = TempDir::new().unwrap();

    // Create a valid xcargo.toml
    fs::write(
        temp_dir.path().join("xcargo.toml"),
        r#"[targets]
default = ["x86_64-unknown-linux-gnu"]

[build]
parallel = true
cache = true

[container]
use_when = "never"
"#,
    )
    .unwrap();

    xcargo()
        .current_dir(temp_dir.path())
        .arg("config")
        .assert()
        .success()
        .stdout(predicate::str::contains("Configuration from"))
        .stdout(predicate::str::contains("[targets]"))
        .stdout(predicate::str::contains("x86_64-unknown-linux-gnu"));
}

#[test]
fn test_config_default_shows_toml() {
    xcargo()
        .arg("config")
        .arg("--default")
        .assert()
        .success()
        .stdout(predicate::str::contains("[targets]"))
        .stdout(predicate::str::contains("[build]"))
        .stdout(predicate::str::contains("parallel"))
        .stdout(predicate::str::contains("cache"));
}

// ============================================================================
// Init Command - Overwrite Scenarios
// ============================================================================

#[test]
fn test_init_with_existing_xcargo_toml() {
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

    // Create existing xcargo.toml
    fs::write(
        temp_dir.path().join("xcargo.toml"),
        r#"[targets]
default = ["x86_64-unknown-linux-gnu"]
"#,
    )
    .unwrap();

    // Running init should warn about existing file
    // The command will prompt (which will fail in CI), but we test the warning is shown
    let output = xcargo()
        .current_dir(temp_dir.path())
        .arg("init")
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should mention that xcargo.toml already exists (in stdout or stderr)
    assert!(
        stdout.contains("xcargo.toml already exists")
        || stderr.contains("xcargo.toml already exists")
        || stdout.contains("Overwrite")
        || stderr.contains("Overwrite")
    );
}

// ============================================================================
// Target Commands
// ============================================================================

#[test]
fn test_target_add_with_custom_toolchain() {
    // This test verifies the toolchain parameter works
    // It may fail if toolchain isn't installed, which is expected
    let output = xcargo()
        .args(["target", "add", "wasm32-unknown-unknown", "--toolchain", "stable"])
        .output()
        .unwrap();

    // Should attempt to add target (may succeed or fail based on system state)
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should at least show it's trying to add the target
    assert!(
        stdout.contains("wasm32-unknown-unknown")
        || stderr.contains("wasm32-unknown-unknown")
        || stdout.contains("Adding target")
        || stderr.contains("rustup")
    );
}

#[test]
fn test_target_list_with_custom_toolchain() {
    xcargo()
        .args(["target", "list", "--installed", "--toolchain", "stable"])
        .assert()
        .success();
}

#[test]
fn test_target_info_invalid_target() {
    // Invalid targets (less than 3 parts) should fail
    xcargo()
        .args(["target", "info", "invalid"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("Invalid target"));
}

// ============================================================================
// Build Options Combinations
// ============================================================================

#[test]
fn test_build_with_zig_and_target() {
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

    // Should attempt build with Zig (may fail if Zig not installed)
    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args(["build", "--zig", "--target", "x86_64-unknown-linux-musl"])
        .output();
}

#[test]
fn test_build_with_no_zig_flag() {
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
        .args(["build", "--no-zig"])
        .output();
}

#[test]
fn test_check_with_zig_flag() {
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
        .args(["check", "--zig", "--target", "x86_64-unknown-linux-musl"])
        .output();
}

#[test]
fn test_check_with_no_zig_flag() {
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
        .args(["check", "--no-zig"])
        .output();
}

#[test]
fn test_test_with_zig_flag() {
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
        .args(["test", "--zig"])
        .output();
}

#[test]
fn test_test_with_no_zig_flag() {
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
        .args(["test", "--no-zig"])
        .output();
}

#[test]
fn test_test_with_release_flag() {
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
        .args(["test", "--release"])
        .output();
}

// ============================================================================
// Parallel Builds
// ============================================================================

#[test]
fn test_build_all_with_parallel_enabled() {
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

    // Create xcargo.toml with parallel=true and multiple targets
    fs::write(
        temp_dir.path().join("xcargo.toml"),
        r#"[targets]
default = ["x86_64-unknown-linux-gnu", "x86_64-unknown-linux-musl"]

[build]
parallel = true
cache = false
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/main.rs"), "fn main() {}")
        .unwrap();

    // Should attempt parallel build
    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args(["build", "--all"])
        .output();
}

#[test]
fn test_check_all_with_parallel_enabled() {
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
parallel = true
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
fn test_test_all_with_parallel_enabled() {
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
parallel = true
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
// Cargo Args Pass-Through
// ============================================================================

#[test]
fn test_build_with_multiple_cargo_args() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
"#,
    )
    .unwrap();

    fs::create_dir(temp_dir.path().join("src")).unwrap();
    fs::write(temp_dir.path().join("src/lib.rs"), "pub fn test() {}")
        .unwrap();

    let _output = xcargo()
        .current_dir(temp_dir.path())
        .args(["build", "--", "--lib", "--locked"])
        .output();
}

#[test]
fn test_check_with_cargo_args() {
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
        .args(["check", "--", "--all-targets"])
        .output();
}

#[test]
fn test_test_with_cargo_args() {
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
        .args(["test", "--", "--nocapture"])
        .output();
}

// ============================================================================
// Version Command Variations
// ============================================================================

#[test]
fn test_version_shows_url() {
    xcargo()
        .arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("github.com/ibrahimcesar/xcargo"));
}

#[test]
fn test_version_shows_tagline() {
    xcargo()
        .arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("Cross-compilation, zero friction"));
}
