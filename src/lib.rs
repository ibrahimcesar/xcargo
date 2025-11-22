//! # xcargo 🚀
//!
//! > Cross-compilation made simple
//!
//! **xcargo** is a Rust cross-compilation tool that simplifies building for
//! multiple targets through automatic toolchain management and intelligent
//! container usage.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use xcargo::Target;
//!
//! // Detect available targets
//! let targets = Target::detect_installed()?;
//!
//! // Build for a specific target
//! xcargo build --target x86_64-pc-windows-gnu
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

/// Target platform definitions and detection
pub mod target;

/// Configuration file handling
pub mod config;

/// Toolchain installation and management
pub mod toolchain;

/// Build orchestration
pub mod build;

/// Container runtime integration
#[cfg(feature = "container")]
pub mod container;

/// Dependency management (OpenSSL, etc.)
pub mod deps {
    //! Native dependency handling
}

/// Output and logging
pub mod output;

/// Error types
pub mod error {
    //! Error definitions with structured error codes and suggestions

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

        /// Get a suggestion for fixing this error
        #[must_use] 
        pub fn suggestion(&self) -> Option<String> {
            match self {
                Error::InvalidTarget { suggestions, .. } => {
                    if suggestions.is_empty() {
                        Some("Run 'xcargo target list' to see available targets".to_string())
                    } else {
                        Some(format!("Did you mean: {}?", suggestions.join(", ")))
                    }
                }
                Error::ToolchainMissing { install_hint, .. } => Some(install_hint.clone()),
                Error::LinkerMissing { install_hint, .. } => Some(install_hint.clone()),
                Error::BuildFailed { suggestion, .. } => suggestion.clone(),
                Error::ContainerNotAvailable { install_hint, .. } => Some(install_hint.clone()),
                Error::ConfigParse { path, .. } => {
                    Some(format!("Check {path} for syntax errors"))
                }
                _ => None,
            }
        }

        /// Get a hint (additional context) for this error
        #[must_use] 
        pub fn hint(&self) -> Option<String> {
            match self {
                Error::TargetNotFound(_) | Error::InvalidTarget { .. } => {
                    Some("Use 'xcargo target list' to see available targets".to_string())
                }
                Error::LinkerMissing { target, .. } => Some(format!(
                    "Cross-compiling to {target} requires a compatible linker"
                )),
                Error::BuildFailed {
                    exit_code: Some(code),
                    ..
                } => Some(format!("Cargo exited with code {code}")),
                Error::ContainerNotAvailable { runtime, .. } => {
                    Some(format!("Tried to use {runtime} but it's not running"))
                }
                _ => None,
            }
        }

        /// Create a linker missing error with platform-specific install hints
        #[must_use] 
        pub fn linker_not_found(linker: &str, target: &str, host_os: &str) -> Self {
            let install_hint = match (host_os, target) {
                ("macos", t) if t.contains("windows") => {
                    "brew install mingw-w64".to_string()
                }
                ("macos", t) if t.contains("linux") => {
                    "Consider using Zig: brew install zig && xcargo build --zig".to_string()
                }
                ("linux", t) if t.contains("windows") => {
                    "sudo apt install mingw-w64  # or your distro's package manager".to_string()
                }
                ("linux", t) if t.contains("darwin") || t.contains("apple") => {
                    "macOS cross-compilation requires osxcross: https://github.com/tpoechtrager/osxcross".to_string()
                }
                ("windows", t) if t.contains("linux") => {
                    "Consider using Zig: scoop install zig && xcargo build --zig".to_string()
                }
                _ => format!("Install a linker that supports {target}"),
            };

            Error::LinkerMissing {
                linker: linker.to_string(),
                target: target.to_string(),
                install_hint,
            }
        }

        /// Create a container not available error with platform-specific hints
        #[must_use] 
        pub fn container_not_found(runtime: &str, host_os: &str) -> Self {
            let install_hint = match host_os {
                "macos" => "Install Docker Desktop: https://www.docker.com/products/docker-desktop\n\
                     Or Podman: brew install podman && podman machine init && podman machine start".to_string(),
                "linux" => "Install Docker: sudo apt install docker.io && sudo systemctl start docker\n\
                     Or Podman: sudo apt install podman".to_string(),
                "windows" => "Install Docker Desktop: https://www.docker.com/products/docker-desktop\n\
                     Or Podman: winget install RedHat.Podman".to_string(),
                _ => format!("Install {runtime} or a compatible container runtime"),
            };

            Error::ContainerNotAvailable {
                runtime: runtime.to_string(),
                install_hint,
            }
        }
    }

    /// Result type alias
    pub type Result<T> = std::result::Result<T, Error>;
}

/// Prelude for convenient imports
pub mod prelude {
    //! Convenient re-exports
    //!
    //! ```rust
    //! use xcargo::prelude::*;
    //! ```

    pub use crate::build::{BuildOptions, Builder, CargoOperation};
    pub use crate::config::Config;
    pub use crate::error::{Error, ExitCode, Result};
    pub use crate::target::{Target, TargetRequirements, TargetTier};
    pub use crate::toolchain::{Toolchain, ToolchainManager};
}

// Re-exports
pub use error::{Error, ExitCode, Result};

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // Placeholder test
        assert_eq!(2 + 2, 4);
    }
}
