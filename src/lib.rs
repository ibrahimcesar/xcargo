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
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::assigning_clones)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unused_self)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::unnecessary_unwrap)]
#![cfg_attr(test, allow(deprecated))]

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
pub mod deps {}

/// Output and logging
pub mod output;

/// Error types
pub mod error;

/// Prelude for convenient imports
pub mod prelude {
    //! Convenient re-exports
    //!
    //! ```rust
    //! use xcargo::prelude::*;
    //! ```
    #![allow(clippy::mixed_attributes_style)]

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
