//! Error definitions with structured error codes and suggestions

mod suggestions;

use thiserror::Error;

/// Exit codes for CI systems
/// Based on BSD sysexits.h conventions where applicable
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    /// Success
    Success = 0,
    /// General error
    GeneralError = 1,
    /// Configuration error (malformed config, missing file)
    ConfigError = 2,
    /// Target not found or invalid
    TargetError = 3,
    /// Toolchain error (rustup, linker missing)
    ToolchainError = 4,
    /// Build failed (cargo returned error)
    BuildError = 5,
    /// Container error (Docker/Podman issue)
    ContainerError = 6,
    /// IO error (file not found, permission denied)
    IoError = 7,
    /// User cancelled operation
    UserCancelled = 130,
}

impl From<&Error> for ExitCode {
    fn from(error: &Error) -> Self {
        match error {
            Error::Io(_) => ExitCode::IoError,
            Error::Prompt(_) => ExitCode::UserCancelled,
            Error::TargetNotFound(_) | Error::InvalidTarget { .. } => ExitCode::TargetError,
            Error::Toolchain(_)
            | Error::ToolchainMissing { .. }
            | Error::LinkerMissing { .. } => ExitCode::ToolchainError,
            Error::Build(_) | Error::BuildFailed { .. } => ExitCode::BuildError,
            Error::Config(_) | Error::ConfigParse { .. } => ExitCode::ConfigError,
            Error::Container(_) | Error::ContainerNotAvailable { .. } => {
                ExitCode::ContainerError
            }
        }
    }
}

/// Main error type for xcargo
#[derive(Error, Debug)]
pub enum Error {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Prompt/interactive input error
    #[error("Input error: {0}")]
    Prompt(String),

    /// Target not found (simple)
    #[error("Target not found: {0}")]
    TargetNotFound(String),

    /// Invalid target with suggestion
    #[error("Invalid target '{target}'")]
    InvalidTarget {
        /// The invalid target triple
        target: String,
        /// Suggested valid targets
        suggestions: Vec<String>,
    },

    /// Toolchain error (simple)
    #[error("Toolchain error: {0}")]
    Toolchain(String),

    /// Toolchain missing with install hint
    #[error("Toolchain '{toolchain}' is not installed")]
    ToolchainMissing {
        /// The missing toolchain
        toolchain: String,
        /// Install command hint
        install_hint: String,
    },

    /// Linker missing with install hint
    #[error("Linker '{linker}' not found for target '{target}'")]
    LinkerMissing {
        /// The missing linker
        linker: String,
        /// Target that requires it
        target: String,
        /// Install command hint
        install_hint: String,
    },

    /// Build error (simple)
    #[error("Build failed: {0}")]
    Build(String),

    /// Build failed with details
    #[error("Build failed for target '{target}'")]
    BuildFailed {
        /// Target that failed
        target: String,
        /// Exit code from cargo
        exit_code: Option<i32>,
        /// Suggestion for fixing
        suggestion: Option<String>,
    },

    /// Configuration error (simple)
    #[error("Configuration error: {0}")]
    Config(String),

    /// Config parse error with location
    #[error("Failed to parse configuration")]
    ConfigParse {
        /// Config file path
        path: String,
        /// Line number if available
        line: Option<usize>,
        /// Parse error message
        message: String,
    },

    /// Container error (simple)
    #[error("Container error: {0}")]
    Container(String),

    /// Container runtime not available
    #[error("Container runtime not available")]
    ContainerNotAvailable {
        /// Tried runtime
        runtime: String,
        /// Install hint
        install_hint: String,
    },
}

impl Error {
    /// Get the exit code for this error
    #[must_use]
    pub fn exit_code(&self) -> i32 {
        ExitCode::from(self) as i32
    }
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_code_io_error() {
        let err = Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
        assert_eq!(err.exit_code(), ExitCode::IoError as i32);
    }

    #[test]
    fn test_exit_code_target_error() {
        let err = Error::TargetNotFound("invalid".to_string());
        assert_eq!(err.exit_code(), ExitCode::TargetError as i32);
    }

    #[test]
    fn test_exit_code_build_error() {
        let err = Error::Build("failed".to_string());
        assert_eq!(err.exit_code(), ExitCode::BuildError as i32);
    }

    #[test]
    fn test_exit_code_config_error() {
        let err = Error::Config("bad config".to_string());
        assert_eq!(err.exit_code(), ExitCode::ConfigError as i32);
    }
}
