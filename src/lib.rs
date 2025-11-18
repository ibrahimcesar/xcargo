//! # apex 🎯
//!
//! > Reach the apex of cross-compilation
//!
//! **apex** is a Rust cross-compilation tool that simplifies building for
//! multiple targets through automatic toolchain management and intelligent
//! container usage.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use apex::Target;
//!
//! // Detect available targets
//! let targets = Target::detect_installed()?;
//!
//! // Build for a specific target
//! apex::build("x86_64-pc-windows-gnu")?;
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

/// Target platform definitions and detection
pub mod target {
    //! Target platform management
}

/// Toolchain installation and management
pub mod toolchain {
    //! Toolchain detection and setup
}

/// Build orchestration
pub mod build {
    //! Build execution and management
}

/// Container runtime integration
pub mod container {
    //! Container runtime (youki, docker, podman)
}

/// Configuration file handling
pub mod config {
    //! Configuration parsing and defaults
}

/// Dependency management (OpenSSL, etc.)
pub mod deps {
    //! Native dependency handling
}

/// Output and logging
pub mod output {
    //! User-facing output formatting
}

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
    //! use apex::prelude::*;
    //! ```

    pub use crate::error::{Error, Result};
}

// Re-exports
pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        // Placeholder test
        assert_eq!(2 + 2, 4);
    }
}
