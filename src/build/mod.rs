//! Build execution and management
//!
//! This module handles the actual build process, including invoking cargo
//! with the appropriate flags for cross-compilation.

mod executor;
mod options;
mod parallel;

// Re-export public types
pub use executor::Builder;
pub use options::{BuildOptions, CargoOperation};
