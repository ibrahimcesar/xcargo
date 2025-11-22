//! Integration tests for xcargo CLI commands

#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Get the xcargo command
fn xcargo() -> Command {
    Command::cargo_bin("xcargo").unwrap()
}

// ============================================================================
// Version and Help Commands
// ============================================================================

#[test]
fn test_version_command() {
    xcargo()
        .arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("xcargo"))
        .stdout(predicate::str::contains("Cross-compilation"));
}

#[test]
fn test_help_command() {
    xcargo()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Cross-compilation"))
        .stdout(predicate::str::contains("build"))
        .stdout(predicate::str::contains("target"))
        .stdout(predicate::str::contains("check"))
        .stdout(predicate::str::contains("test"));
}

#[test]
fn test_build_help() {
    xcargo()
        .args(["build", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--target"))
        .stdout(predicate::str::contains("--release"))
        .stdout(predicate::str::contains("--zig"));
}

#[test]
fn test_check_help() {
    xcargo()
        .args(["check", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--target"))
        .stdout(predicate::str::contains("--zig"));
}

#[test]
fn test_test_help() {
    xcargo()
        .args(["test", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--target"))
        .stdout(predicate::str::contains("--release"));
}

// ============================================================================
// Target Commands
// ============================================================================

#[test]
fn test_target_list() {
    xcargo()
        .args(["target", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Linux"))
        .stdout(predicate::str::contains("Windows"))
        .stdout(predicate::str::contains("macOS"));
}

#[test]
fn test_target_list_installed() {
    xcargo()
        .args(["target", "list", "--installed"])
        .assert()
        .success();
}

#[test]
fn test_target_info_valid() {
    xcargo()
        .args(["target", "info", "x86_64-unknown-linux-gnu"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Triple"))
        .stdout(predicate::str::contains("Architecture"))
        .stdout(predicate::str::contains("OS"));
}

#[test]
fn test_target_info_with_alias() {
    xcargo()
        .args(["target", "info", "linux"])
        .assert()
        .success()
        .stdout(predicate::str::contains("x86_64-unknown-linux-gnu"));
}

#[test]
fn test_target_info_unknown() {
    // Currently xcargo allows unknown targets (it parses them as Container tier)
    // This behavior may change in the future to be more strict
    xcargo()
        .args(["target", "info", "unknown-target-triple"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Container")); // Unknown targets are Container tier
}

// ============================================================================
// Config Commands
// ============================================================================

#[test]
fn test_config_default() {
    xcargo()
        .args(["config", "--default"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[targets]"))
        .stdout(predicate::str::contains("[build]"))
        .stdout(predicate::str::contains("[container]"));
}

#[test]
fn test_config_no_file() {
    let temp_dir = TempDir::new().unwrap();

    xcargo()
        .current_dir(temp_dir.path())
        .arg("config")
        .assert()
        .success()
        .stdout(predicate::str::contains("No xcargo.toml found"));
}

// ============================================================================
// Init Command
// ============================================================================

#[test]
fn test_init_creates_config() {
    let temp_dir = TempDir::new().unwrap();

    // Create a minimal Cargo.toml so it looks like a Rust project
    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    xcargo()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Created xcargo.toml"));

    // Verify the file was created
    assert!(temp_dir.path().join("xcargo.toml").exists());
}

// ============================================================================
// Build Command (Basic Tests)
// ============================================================================

#[test]
fn test_build_no_cargo_toml() {
    let temp_dir = TempDir::new().unwrap();

    // Should fail gracefully when no Cargo.toml exists
    xcargo()
        .current_dir(temp_dir.path())
        .arg("build")
        .assert()
        .failure();
}

#[test]
fn test_build_conflicting_args() {
    // --all and --target are mutually exclusive
    xcargo()
        .args(["build", "--all", "--target", "x86_64-unknown-linux-gnu"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn test_build_zig_conflicting_args() {
    // --zig and --no-zig are mutually exclusive
    xcargo()
        .args(["build", "--zig", "--no-zig"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

// ============================================================================
// Check Command (Basic Tests)
// ============================================================================

#[test]
fn test_check_conflicting_args() {
    // --all and --target are mutually exclusive
    xcargo()
        .args(["check", "--all", "--target", "x86_64-unknown-linux-gnu"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

// ============================================================================
// Exit Codes
// ============================================================================

#[test]
fn test_exit_code_success() {
    xcargo().arg("version").assert().code(0);
}

#[test]
fn test_exit_code_invalid_command() {
    xcargo().arg("nonexistent-command").assert().failure();
}

// ============================================================================
// Verbose Mode
// ============================================================================

#[test]
fn test_verbose_flag_global() {
    xcargo().args(["-v", "version"]).assert().success();
}

#[test]
fn test_verbose_flag_long() {
    xcargo().args(["--verbose", "version"]).assert().success();
}
