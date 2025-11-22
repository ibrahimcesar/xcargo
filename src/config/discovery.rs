//! Configuration file discovery
//!
//! This module handles finding xcargo.toml files in the filesystem

use crate::error::Result;
use std::env;
use std::path::PathBuf;

/// Configuration file discovery utility
pub struct ConfigDiscovery;

impl ConfigDiscovery {
    /// Find xcargo.toml starting from current directory
    ///
    /// Searches upward through parent directories until finding xcargo.toml
    /// or reaching the filesystem root.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xcargo::config::ConfigDiscovery;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// if let Some(path) = ConfigDiscovery::find()? {
    ///     println!("Found config at: {}", path.display());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn find() -> Result<Option<PathBuf>> {
        Self::find_from(env::current_dir()?)
    }

    /// Find xcargo.toml starting from a specific directory
    pub fn find_from(start: PathBuf) -> Result<Option<PathBuf>> {
        let mut current = start;

        loop {
            let config_path = current.join("xcargo.toml");

            if config_path.exists() && config_path.is_file() {
                return Ok(Some(config_path));
            }

            // Try to go to parent directory
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break, // Reached filesystem root
            }
        }

        Ok(None)
    }

    /// Check if xcargo.toml exists in the current directory
    pub fn exists_in_current() -> Result<bool> {
        let current = env::current_dir()?;
        Ok(current.join("xcargo.toml").exists())
    }

    /// Get the default config file path (current directory)
    pub fn default_path() -> Result<PathBuf> {
        Ok(env::current_dir()?.join("xcargo.toml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_find_in_current_dir() {
        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("xcargo.toml");

        // Create empty config file
        fs::write(&config_path, "[targets]\n").unwrap();

        // Find from temp directory
        let found = ConfigDiscovery::find_from(temp.path().to_path_buf()).unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap(), config_path);
    }

    #[test]
    fn test_find_in_parent_dir() {
        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("xcargo.toml");
        let sub_dir = temp.path().join("sub");

        // Create config in root
        fs::write(&config_path, "[targets]\n").unwrap();

        // Create subdirectory
        fs::create_dir(&sub_dir).unwrap();

        // Find from subdirectory should find parent's config
        let found = ConfigDiscovery::find_from(sub_dir).unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap(), config_path);
    }

    #[test]
    fn test_not_found() {
        let temp = TempDir::new().unwrap();

        // No config file exists
        let found = ConfigDiscovery::find_from(temp.path().to_path_buf()).unwrap();

        assert!(found.is_none());
    }

    #[test]
    fn test_default_path() {
        let path = ConfigDiscovery::default_path().unwrap();
        assert!(path.ends_with("xcargo.toml"));
    }
}
