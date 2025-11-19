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
pub mod build {
    //! Build execution and management
}

/// Container runtime integration
pub mod container {
    //! Container runtime (youki, docker, podman)
}

/// Dependency management (OpenSSL, etc.)
pub mod deps {
    //! Native dependency handling
}

/// Output and logging
pub mod output;

/// Error types
pub mod error {
    //! Error definitions
    
    use thiserror::Error;
    
    /// Main error type for apex
    #[derive(Error, Debug)]
    pub enum Error {
        /// IO error
        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),
        
        /// Target not found
        #[error("Target not found: {0}")]
        TargetNotFound(String),
        
        /// Toolchain error
        #[error("Toolchain error: {0}")]
        Toolchain(String),
        
        /// Build error
        #[error("Build failed: {0}")]
        Build(String),
        
        /// Configuration error
        #[error("Configuration error: {0}")]
        Config(String),
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

    pub use crate::error::{Error, Result};
    pub use crate::target::{Target, TargetRequirements, TargetTier};
    pub use crate::config::Config;
    pub use crate::toolchain::{Toolchain, ToolchainManager};
}

// Re-exports
pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // Placeholder test
        assert_eq!(2 + 2, 4);
    }
}
