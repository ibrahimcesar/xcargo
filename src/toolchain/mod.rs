//! Toolchain detection and management
//!
//! This module handles Rust toolchain detection, installation, and management
//! through rustup integration.

pub mod zig;
use crate::error::{Error, Result};
use crate::target::Target;
use std::process::Command;
use std::str;

/// Represents a Rust toolchain
#[derive(Debug, Clone, PartialEq)]
pub struct Toolchain {
    /// Toolchain name (e.g., "stable", "nightly", "1.70.0")
    pub name: String,

    /// Whether this is the default/active toolchain
    pub is_default: bool,

    /// Installed targets for this toolchain
    pub targets: Vec<String>,
}

/// Toolchain manager for rustup operations
pub struct ToolchainManager {
    /// Path to rustup binary
    rustup_path: String,
}

impl ToolchainManager {
    /// Create a new toolchain manager
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xcargo::toolchain::ToolchainManager;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let manager = ToolchainManager::new()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Result<Self> {
        let rustup_path = Self::find_rustup()?;
        Ok(Self { rustup_path })
    }

    /// Find rustup binary in PATH
    fn find_rustup() -> Result<String> {
        // Try to run rustup --version to verify it exists
        let output = Command::new("rustup")
            .arg("--version")
            .output()
            .map_err(|e| {
                Error::Toolchain(format!(
                    "rustup not found. Please install rustup from https://rustup.rs/. Error: {e}"
                ))
            })?;

        if !output.status.success() {
            return Err(Error::Toolchain(
                "rustup found but failed to execute. Please check your rustup installation."
                    .to_string(),
            ));
        }

        Ok("rustup".to_string())
    }

    /// List all installed toolchains
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xcargo::toolchain::ToolchainManager;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let manager = ToolchainManager::new()?;
    /// let toolchains = manager.list_toolchains()?;
    /// for toolchain in toolchains {
    ///     println!("{} (default: {})", toolchain.name, toolchain.is_default);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_toolchains(&self) -> Result<Vec<Toolchain>> {
        let output = Command::new(&self.rustup_path)
            .args(["toolchain", "list"])
            .output()
            .map_err(|e| Error::Toolchain(format!("Failed to list toolchains: {e}")))?;

        if !output.status.success() {
            return Err(Error::Toolchain("Failed to list toolchains".to_string()));
        }

        let stdout = str::from_utf8(&output.stdout)
            .map_err(|e| Error::Toolchain(format!("Invalid UTF-8 in rustup output: {e}")))?;

        let mut toolchains = Vec::new();
        for line in stdout.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let is_default = line.contains("(default)");
            let name = line.replace("(default)", "").trim().to_string();

            toolchains.push(Toolchain {
                name,
                is_default,
                targets: Vec::new(), // Will be populated if needed
            });
        }

        Ok(toolchains)
    }

    /// Get the default/active toolchain
    pub fn get_default_toolchain(&self) -> Result<Option<Toolchain>> {
        let toolchains = self.list_toolchains()?;
        Ok(toolchains.into_iter().find(|t| t.is_default))
    }

    /// List installed targets for a specific toolchain
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xcargo::toolchain::ToolchainManager;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let manager = ToolchainManager::new()?;
    /// let targets = manager.list_targets("stable")?;
    /// println!("Installed targets: {:?}", targets);
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_targets(&self, toolchain: &str) -> Result<Vec<String>> {
        let output = Command::new(&self.rustup_path)
            .args(["target", "list", "--installed", "--toolchain", toolchain])
            .output()
            .map_err(|e| Error::Toolchain(format!("Failed to list targets: {e}")))?;

        if !output.status.success() {
            return Err(Error::Toolchain(format!(
                "Failed to list targets for toolchain '{toolchain}'"
            )));
        }

        let stdout = str::from_utf8(&output.stdout)
            .map_err(|e| Error::Toolchain(format!("Invalid UTF-8 in rustup output: {e}")))?;

        let targets: Vec<String> = stdout
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        Ok(targets)
    }

    /// Check if a target is installed for a specific toolchain
    pub fn is_target_installed(&self, toolchain: &str, target: &str) -> Result<bool> {
        let targets = self.list_targets(toolchain)?;
        Ok(targets.iter().any(|t| t == target))
    }

    /// Install a target for a specific toolchain
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xcargo::toolchain::ToolchainManager;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let manager = ToolchainManager::new()?;
    /// manager.install_target("stable", "x86_64-pc-windows-gnu")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn install_target(&self, toolchain: &str, target: &str) -> Result<()> {
        println!(
            "Installing target {target} for toolchain {toolchain}..."
        );

        let output = Command::new(&self.rustup_path)
            .args(["target", "add", target, "--toolchain", toolchain])
            .output()
            .map_err(|e| Error::Toolchain(format!("Failed to install target: {e}")))?;

        if !output.status.success() {
            let stderr = str::from_utf8(&output.stderr).unwrap_or("<invalid UTF-8>");
            return Err(Error::Toolchain(format!(
                "Failed to install target '{target}' for toolchain '{toolchain}': {stderr}"
            )));
        }

        println!("Successfully installed target {target}");
        Ok(())
    }

    /// Ensure a target is installed, installing it if necessary
    pub fn ensure_target(&self, toolchain: &str, target: &str) -> Result<()> {
        if self.is_target_installed(toolchain, target)? {
            return Ok(());
        }
        self.install_target(toolchain, target)
    }

    /// Install a toolchain if not already installed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xcargo::toolchain::ToolchainManager;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let manager = ToolchainManager::new()?;
    /// manager.install_toolchain("stable")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn install_toolchain(&self, toolchain: &str) -> Result<()> {
        println!("Installing toolchain {toolchain}...");

        let output = Command::new(&self.rustup_path)
            .args(["toolchain", "install", toolchain])
            .output()
            .map_err(|e| Error::Toolchain(format!("Failed to install toolchain: {e}")))?;

        if !output.status.success() {
            let stderr = str::from_utf8(&output.stderr).unwrap_or("<invalid UTF-8>");
            return Err(Error::Toolchain(format!(
                "Failed to install toolchain '{toolchain}': {stderr}"
            )));
        }

        println!("Successfully installed toolchain {toolchain}");
        Ok(())
    }

    /// Check if a toolchain is installed
    pub fn is_toolchain_installed(&self, toolchain: &str) -> Result<bool> {
        let toolchains = self.list_toolchains()?;
        Ok(toolchains.iter().any(|t| t.name.starts_with(toolchain)))
    }

    /// Ensure a toolchain is installed, installing it if necessary
    pub fn ensure_toolchain(&self, toolchain: &str) -> Result<()> {
        if self.is_toolchain_installed(toolchain)? {
            return Ok(());
        }
        self.install_toolchain(toolchain)
    }

    /// Prepare environment for cross-compilation to a target
    ///
    /// This ensures:
    /// 1. The specified toolchain is installed
    /// 2. The target is added to the toolchain
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xcargo::toolchain::ToolchainManager;
    /// use xcargo::target::Target;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let manager = ToolchainManager::new()?;
    /// let target = Target::from_triple("x86_64-pc-windows-gnu")?;
    /// manager.prepare_target("stable", &target)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn prepare_target(&self, toolchain: &str, target: &Target) -> Result<()> {
        // Ensure toolchain is installed
        self.ensure_toolchain(toolchain)?;

        // Ensure target is installed
        self.ensure_target(toolchain, &target.triple)?;

        Ok(())
    }

    /// Get rustup home directory
    pub fn get_rustup_home(&self) -> Result<std::path::PathBuf> {
        let output = Command::new(&self.rustup_path)
            .args(["show", "home"])
            .output()
            .map_err(|e| Error::Toolchain(format!("Failed to get rustup home: {e}")))?;

        if !output.status.success() {
            return Err(Error::Toolchain(
                "Failed to determine rustup home directory".to_string(),
            ));
        }

        let stdout = str::from_utf8(&output.stdout)
            .map_err(|e| Error::Toolchain(format!("Invalid UTF-8 in rustup output: {e}")))?;

        let path = stdout.trim();
        Ok(std::path::PathBuf::from(path))
    }

    /// Get information about the active toolchain
    pub fn show_active_toolchain(&self) -> Result<String> {
        let output = Command::new(&self.rustup_path)
            .args(["show", "active-toolchain"])
            .output()
            .map_err(|e| Error::Toolchain(format!("Failed to get active toolchain: {e}")))?;

        if !output.status.success() {
            return Err(Error::Toolchain(
                "Failed to determine active toolchain".to_string(),
            ));
        }

        let stdout = str::from_utf8(&output.stdout)
            .map_err(|e| Error::Toolchain(format!("Invalid UTF-8 in rustup output: {e}")))?;

        Ok(stdout.trim().to_string())
    }
}

impl Default for ToolchainManager {
    fn default() -> Self {
        Self::new().expect("Failed to initialize ToolchainManager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_rustup() {
        // This test will only pass if rustup is installed
        let result = ToolchainManager::find_rustup();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "rustup");
    }

    #[test]
    fn test_new_toolchain_manager() {
        let manager = ToolchainManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_list_toolchains() {
        let manager = ToolchainManager::new();
        if manager.is_err() {
            // Skip test if rustup is not available
            return;
        }

        let manager = manager.unwrap();
        let toolchains = manager.list_toolchains();
        assert!(toolchains.is_ok());

        let toolchains = toolchains.unwrap();
        // Should have at least one toolchain installed
        assert!(!toolchains.is_empty());
    }

    #[test]
    fn test_get_default_toolchain() {
        let manager = ToolchainManager::new();
        if manager.is_err() {
            return;
        }

        let manager = manager.unwrap();
        let default_toolchain = manager.get_default_toolchain();
        assert!(default_toolchain.is_ok());

        // Note: May not have a default toolchain in some environments
        // Just verify the function works without asserting it exists
    }

    #[test]
    fn test_list_targets() {
        let manager = ToolchainManager::new();
        if manager.is_err() {
            return;
        }

        let manager = manager.unwrap();
        let targets = manager.list_targets("stable");

        if targets.is_err() {
            // Might fail if stable is not installed
            return;
        }

        let targets = targets.unwrap();
        // Should have at least the host target installed
        assert!(!targets.is_empty());
    }

    #[test]
    fn test_is_toolchain_installed() {
        let manager = ToolchainManager::new();
        if manager.is_err() {
            return;
        }

        let manager = manager.unwrap();

        // stable should typically be installed
        let is_installed = manager.is_toolchain_installed("stable");
        assert!(is_installed.is_ok());
    }

    #[test]
    fn test_get_rustup_home() {
        let manager = ToolchainManager::new();
        if manager.is_err() {
            return;
        }

        let manager = manager.unwrap();
        let home = manager.get_rustup_home();
        assert!(home.is_ok());

        let home = home.unwrap();
        assert!(home.exists());
    }

    #[test]
    fn test_show_active_toolchain() {
        let manager = ToolchainManager::new();
        if manager.is_err() {
            return;
        }

        let manager = manager.unwrap();
        let active = manager.show_active_toolchain();
        assert!(active.is_ok());

        let active = active.unwrap();
        assert!(!active.is_empty());
    }

    #[test]
    fn test_is_target_installed() {
        let manager = ToolchainManager::new();
        if manager.is_err() {
            return;
        }

        let manager = manager.unwrap();

        // Get host target (should always be installed)
        if let Ok(host) = Target::detect_host() {
            let is_installed = manager.is_target_installed("stable", &host.triple);
            // Skip if stable is not installed
            if is_installed.is_ok() {
                // Host target should be installed
                assert!(is_installed.unwrap());
            }
        }
    }
}
