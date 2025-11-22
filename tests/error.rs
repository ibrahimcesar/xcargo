//! Unit tests for error types and exit codes

use xcargo::error::{Error, ExitCode};

// ============================================================================
// Exit Code Tests
// ============================================================================

#[test]
fn test_exit_code_values() {
    assert_eq!(ExitCode::Success as i32, 0);
    assert_eq!(ExitCode::GeneralError as i32, 1);
    assert_eq!(ExitCode::ConfigError as i32, 2);
    assert_eq!(ExitCode::TargetError as i32, 3);
    assert_eq!(ExitCode::ToolchainError as i32, 4);
    assert_eq!(ExitCode::BuildError as i32, 5);
    assert_eq!(ExitCode::ContainerError as i32, 6);
    assert_eq!(ExitCode::IoError as i32, 7);
    assert_eq!(ExitCode::UserCancelled as i32, 130);
}

#[test]
fn test_error_to_exit_code_io() {
    let error = Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
    assert_eq!(error.exit_code(), 7);
}

#[test]
fn test_error_to_exit_code_prompt() {
    let error = Error::Prompt("cancelled".to_string());
    assert_eq!(error.exit_code(), 130);
}

#[test]
fn test_error_to_exit_code_target() {
    let error = Error::TargetNotFound("test".to_string());
    assert_eq!(error.exit_code(), 3);
}

#[test]
fn test_error_to_exit_code_invalid_target() {
    let error = Error::InvalidTarget {
        target: "invalid".to_string(),
        suggestions: vec!["linux".to_string()],
    };
    assert_eq!(error.exit_code(), 3);
}

#[test]
fn test_error_to_exit_code_toolchain() {
    let error = Error::Toolchain("test".to_string());
    assert_eq!(error.exit_code(), 4);
}

#[test]
fn test_error_to_exit_code_toolchain_missing() {
    let error = Error::ToolchainMissing {
        toolchain: "nightly".to_string(),
        install_hint: "rustup install nightly".to_string(),
    };
    assert_eq!(error.exit_code(), 4);
}

#[test]
fn test_error_to_exit_code_linker_missing() {
    let error = Error::LinkerMissing {
        linker: "x86_64-w64-mingw32-gcc".to_string(),
        target: "x86_64-pc-windows-gnu".to_string(),
        install_hint: "brew install mingw-w64".to_string(),
    };
    assert_eq!(error.exit_code(), 4);
}

#[test]
fn test_error_to_exit_code_build() {
    let error = Error::Build("test".to_string());
    assert_eq!(error.exit_code(), 5);
}

#[test]
fn test_error_to_exit_code_build_failed() {
    let error = Error::BuildFailed {
        target: "x86_64-unknown-linux-gnu".to_string(),
        exit_code: Some(101),
        suggestion: None,
    };
    assert_eq!(error.exit_code(), 5);
}

#[test]
fn test_error_to_exit_code_config() {
    let error = Error::Config("test".to_string());
    assert_eq!(error.exit_code(), 2);
}

#[test]
fn test_error_to_exit_code_config_parse() {
    let error = Error::ConfigParse {
        path: "xcargo.toml".to_string(),
        line: Some(10),
        message: "invalid syntax".to_string(),
    };
    assert_eq!(error.exit_code(), 2);
}

#[test]
fn test_error_to_exit_code_container() {
    let error = Error::Container("test".to_string());
    assert_eq!(error.exit_code(), 6);
}

#[test]
fn test_error_to_exit_code_container_not_available() {
    let error = Error::ContainerNotAvailable {
        runtime: "docker".to_string(),
        install_hint: "Install Docker".to_string(),
    };
    assert_eq!(error.exit_code(), 6);
}

// ============================================================================
// Suggestion Tests
// ============================================================================

#[test]
fn test_suggestion_invalid_target_with_suggestions() {
    let error = Error::InvalidTarget {
        target: "linuxx".to_string(),
        suggestions: vec!["linux".to_string(), "x86_64-unknown-linux-gnu".to_string()],
    };
    let suggestion = error.suggestion().unwrap();
    assert!(suggestion.contains("Did you mean"));
    assert!(suggestion.contains("linux"));
}

#[test]
fn test_suggestion_invalid_target_empty_suggestions() {
    let error = Error::InvalidTarget {
        target: "totally-invalid".to_string(),
        suggestions: vec![],
    };
    let suggestion = error.suggestion().unwrap();
    assert!(suggestion.contains("xcargo target list"));
}

#[test]
fn test_suggestion_toolchain_missing() {
    let error = Error::ToolchainMissing {
        toolchain: "nightly".to_string(),
        install_hint: "rustup install nightly".to_string(),
    };
    let suggestion = error.suggestion().unwrap();
    assert!(suggestion.contains("rustup install nightly"));
}

#[test]
fn test_suggestion_linker_missing() {
    let error = Error::LinkerMissing {
        linker: "x86_64-w64-mingw32-gcc".to_string(),
        target: "x86_64-pc-windows-gnu".to_string(),
        install_hint: "brew install mingw-w64".to_string(),
    };
    let suggestion = error.suggestion().unwrap();
    assert!(suggestion.contains("brew install mingw-w64"));
}

#[test]
fn test_suggestion_config_parse() {
    let error = Error::ConfigParse {
        path: "xcargo.toml".to_string(),
        line: Some(10),
        message: "invalid syntax".to_string(),
    };
    let suggestion = error.suggestion().unwrap();
    assert!(suggestion.contains("xcargo.toml"));
}

#[test]
fn test_suggestion_build_simple() {
    let error = Error::Build("test".to_string());
    assert!(error.suggestion().is_none());
}

// ============================================================================
// Hint Tests
// ============================================================================

#[test]
fn test_hint_target_not_found() {
    let error = Error::TargetNotFound("test".to_string());
    let hint = error.hint().unwrap();
    assert!(hint.contains("xcargo target list"));
}

#[test]
fn test_hint_linker_missing() {
    let error = Error::LinkerMissing {
        linker: "gcc".to_string(),
        target: "x86_64-unknown-linux-gnu".to_string(),
        install_hint: "install gcc".to_string(),
    };
    let hint = error.hint().unwrap();
    assert!(hint.contains("x86_64-unknown-linux-gnu"));
    assert!(hint.contains("linker"));
}

#[test]
fn test_hint_build_failed_with_exit_code() {
    let error = Error::BuildFailed {
        target: "x86_64-unknown-linux-gnu".to_string(),
        exit_code: Some(101),
        suggestion: None,
    };
    let hint = error.hint().unwrap();
    assert!(hint.contains("101"));
}

#[test]
fn test_hint_container_not_available() {
    let error = Error::ContainerNotAvailable {
        runtime: "docker".to_string(),
        install_hint: "install docker".to_string(),
    };
    let hint = error.hint().unwrap();
    assert!(hint.contains("docker"));
}

// ============================================================================
// Helper Function Tests
// ============================================================================

#[test]
fn test_linker_not_found_macos_windows() {
    let error = Error::linker_not_found("x86_64-w64-mingw32-gcc", "x86_64-pc-windows-gnu", "macos");
    if let Error::LinkerMissing { install_hint, .. } = error {
        assert!(install_hint.contains("mingw-w64"));
    } else {
        panic!("Expected LinkerMissing error");
    }
}

#[test]
fn test_linker_not_found_macos_linux() {
    let error = Error::linker_not_found("gcc", "x86_64-unknown-linux-gnu", "macos");
    if let Error::LinkerMissing { install_hint, .. } = error {
        assert!(install_hint.contains("Zig"));
    } else {
        panic!("Expected LinkerMissing error");
    }
}

#[test]
fn test_linker_not_found_linux_windows() {
    let error = Error::linker_not_found("x86_64-w64-mingw32-gcc", "x86_64-pc-windows-gnu", "linux");
    if let Error::LinkerMissing { install_hint, .. } = error {
        assert!(install_hint.contains("mingw-w64"));
    } else {
        panic!("Expected LinkerMissing error");
    }
}

#[test]
fn test_linker_not_found_linux_macos() {
    let error = Error::linker_not_found("clang", "x86_64-apple-darwin", "linux");
    if let Error::LinkerMissing { install_hint, .. } = error {
        assert!(install_hint.contains("osxcross"));
    } else {
        panic!("Expected LinkerMissing error");
    }
}

#[test]
fn test_container_not_found_macos() {
    let error = Error::container_not_found("docker", "macos");
    if let Error::ContainerNotAvailable { install_hint, .. } = error {
        assert!(install_hint.contains("Docker Desktop") || install_hint.contains("Podman"));
    } else {
        panic!("Expected ContainerNotAvailable error");
    }
}

#[test]
fn test_container_not_found_linux() {
    let error = Error::container_not_found("docker", "linux");
    if let Error::ContainerNotAvailable { install_hint, .. } = error {
        assert!(install_hint.contains("apt") || install_hint.contains("docker"));
    } else {
        panic!("Expected ContainerNotAvailable error");
    }
}

#[test]
fn test_container_not_found_windows() {
    let error = Error::container_not_found("docker", "windows");
    if let Error::ContainerNotAvailable { install_hint, .. } = error {
        assert!(install_hint.contains("Docker Desktop") || install_hint.contains("Podman"));
    } else {
        panic!("Expected ContainerNotAvailable error");
    }
}

// ============================================================================
// Display Tests
// ============================================================================

#[test]
fn test_error_display_io() {
    let error = Error::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "file not found",
    ));
    let display = format!("{}", error);
    assert!(display.contains("IO error"));
}

#[test]
fn test_error_display_target_not_found() {
    let error = Error::TargetNotFound("invalid-target".to_string());
    let display = format!("{}", error);
    assert!(display.contains("invalid-target"));
}

#[test]
fn test_error_display_invalid_target() {
    let error = Error::InvalidTarget {
        target: "invalid".to_string(),
        suggestions: vec![],
    };
    let display = format!("{}", error);
    assert!(display.contains("Invalid target"));
    assert!(display.contains("invalid"));
}

#[test]
fn test_error_display_build_failed() {
    let error = Error::BuildFailed {
        target: "x86_64-unknown-linux-gnu".to_string(),
        exit_code: Some(101),
        suggestion: None,
    };
    let display = format!("{}", error);
    assert!(display.contains("Build failed"));
    assert!(display.contains("x86_64-unknown-linux-gnu"));
}
