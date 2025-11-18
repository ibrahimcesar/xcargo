//! Configuration file handling for xcargo
//!
//! This module handles parsing and managing xcargo.toml configuration files.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::error::{Error, Result};

mod discovery;

pub use discovery::ConfigDiscovery;

/// Main configuration structure for xcargo.toml
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Target platform configuration
    #[serde(default)]
    pub targets: TargetsConfig,

    /// Build configuration
    #[serde(default)]
    pub build: BuildConfig,

    /// Container runtime configuration
    #[serde(default)]
    pub container: ContainerConfig,

    /// Custom profiles for different build scenarios
    #[serde(default)]
    pub profiles: HashMap<String, ProfileConfig>,
}

/// Target configuration section
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TargetsConfig {
    /// Default targets to build when no target is specified
    #[serde(default)]
    pub default: Vec<String>,

    /// Per-target custom configuration
    #[serde(default, flatten)]
    pub custom: HashMap<String, TargetCustomConfig>,
}

/// Custom configuration for a specific target
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TargetCustomConfig {
    /// Custom linker to use for this target
    pub linker: Option<String>,

    /// Force container build for this target
    pub force_container: Option<bool>,

    /// Additional environment variables
    #[serde(default)]
    pub env: HashMap<String, String>,

    /// Additional rustflags
    pub rustflags: Option<Vec<String>>,
}

/// Build configuration section
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuildConfig {
    /// Enable parallel builds for multiple targets
    #[serde(default = "default_true")]
    pub parallel: bool,

    /// Number of parallel jobs (None = auto-detect)
    pub jobs: Option<usize>,

    /// Enable build caching
    #[serde(default = "default_true")]
    pub cache: bool,

    /// Force container builds even when native is possible
    #[serde(default)]
    pub force_container: bool,

    /// Additional cargo flags
    #[serde(default)]
    pub cargo_flags: Vec<String>,
}

/// Container runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContainerConfig {
    /// Container runtime to use: auto, youki, docker, podman
    #[serde(default = "default_runtime")]
    pub runtime: String,

    /// Condition for when to use containers
    /// Examples: "always", "never", "target.os != host.os"
    #[serde(default = "default_use_when")]
    pub use_when: String,

    /// Custom container image registry
    pub registry: Option<String>,

    /// Image pull policy: always, never, if-not-present
    #[serde(default = "default_pull_policy")]
    pub pull_policy: String,
}

/// Profile configuration for different build scenarios
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfileConfig {
    /// Targets to build in this profile
    pub targets: Vec<String>,

    /// Build configuration overrides
    #[serde(flatten)]
    pub build: Option<BuildConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            targets: TargetsConfig::default(),
            build: BuildConfig::default(),
            container: ContainerConfig::default(),
            profiles: HashMap::new(),
        }
    }
}

impl Default for TargetsConfig {
    fn default() -> Self {
        Self {
            default: Vec::new(),
            custom: HashMap::new(),
        }
    }
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            parallel: true,
            jobs: None, // Auto-detect
            cache: true,
            force_container: false,
            cargo_flags: Vec::new(),
        }
    }
}

impl Default for ContainerConfig {
    fn default() -> Self {
        Self {
            runtime: default_runtime(),
            use_when: default_use_when(),
            registry: None,
            pull_policy: default_pull_policy(),
        }
    }
}

// Default value functions for serde
fn default_true() -> bool {
    true
}

fn default_runtime() -> String {
    "auto".to_string()
}

fn default_use_when() -> String {
    "target.os != host.os".to_string()
}

fn default_pull_policy() -> String {
    "if-not-present".to_string()
}

impl Config {
    /// Load configuration from a TOML file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xcargo::config::Config;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let config = Config::from_file("xcargo.toml")?;
    /// println!("Default targets: {:?}", config.targets.default);
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(path.as_ref())
            .map_err(|e| Error::Config(format!("Failed to read config file: {}", e)))?;

        Self::from_str(&contents)
    }

    /// Parse configuration from a TOML string
    pub fn from_str(toml: &str) -> Result<Self> {
        toml::from_str(toml)
            .map_err(|e| Error::Config(format!("Failed to parse TOML: {}", e)))
    }

    /// Discover and load configuration from the current directory
    ///
    /// Searches for xcargo.toml in current directory and parent directories
    pub fn discover() -> Result<Option<(Self, PathBuf)>> {
        if let Some(path) = ConfigDiscovery::find()? {
            let config = Self::from_file(&path)?;
            Ok(Some((config, path)))
        } else {
            Ok(None)
        }
    }

    /// Get the default configuration
    pub fn default_config() -> Self {
        Self::default()
    }

    /// Merge this configuration with another, with other taking precedence
    pub fn merge(&mut self, other: &Config) {
        // Merge targets
        if !other.targets.default.is_empty() {
            self.targets.default = other.targets.default.clone();
        }
        for (key, value) in &other.targets.custom {
            self.targets.custom.insert(key.clone(), value.clone());
        }

        // Merge build config (other overrides self)
        self.build.parallel = other.build.parallel;
        if other.build.jobs.is_some() {
            self.build.jobs = other.build.jobs;
        }
        self.build.cache = other.build.cache;
        self.build.force_container = other.build.force_container;
        if !other.build.cargo_flags.is_empty() {
            self.build.cargo_flags = other.build.cargo_flags.clone();
        }

        // Merge container config
        self.container.runtime = other.container.runtime.clone();
        self.container.use_when = other.container.use_when.clone();
        if other.container.registry.is_some() {
            self.container.registry = other.container.registry.clone();
        }
        self.container.pull_policy = other.container.pull_policy.clone();

        // Merge profiles
        for (key, value) in &other.profiles {
            self.profiles.insert(key.clone(), value.clone());
        }
    }

    /// Get configuration for a specific target
    pub fn get_target_config(&self, target: &str) -> Option<&TargetCustomConfig> {
        self.targets.custom.get(target)
    }

    /// Get a profile by name
    pub fn get_profile(&self, name: &str) -> Option<&ProfileConfig> {
        self.profiles.get(name)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // Validate runtime
        let valid_runtimes = ["auto", "youki", "docker", "podman"];
        if !valid_runtimes.contains(&self.container.runtime.as_str()) {
            return Err(Error::Config(format!(
                "Invalid container runtime: {}. Must be one of: {}",
                self.container.runtime,
                valid_runtimes.join(", ")
            )));
        }

        // Validate pull policy
        let valid_policies = ["always", "never", "if-not-present"];
        if !valid_policies.contains(&self.container.pull_policy.as_str()) {
            return Err(Error::Config(format!(
                "Invalid pull policy: {}. Must be one of: {}",
                self.container.pull_policy,
                valid_policies.join(", ")
            )));
        }

        // Validate jobs count
        if let Some(jobs) = self.build.jobs {
            if jobs == 0 {
                return Err(Error::Config(
                    "build.jobs must be greater than 0".to_string()
                ));
            }
        }

        Ok(())
    }

    /// Convert configuration to TOML string
    pub fn to_toml(&self) -> Result<String> {
        toml::to_string_pretty(self)
            .map_err(|e| Error::Config(format!("Failed to serialize to TOML: {}", e)))
    }

    /// Save configuration to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let toml = self.to_toml()?;
        std::fs::write(path.as_ref(), toml)
            .map_err(|e| Error::Config(format!("Failed to write config file: {}", e)))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.targets.default.is_empty());
        assert!(config.build.parallel);
        assert!(config.build.cache);
        assert!(!config.build.force_container);
        assert_eq!(config.container.runtime, "auto");
    }

    #[test]
    fn test_parse_minimal_config() {
        let toml = r#"
            [targets]
            default = ["x86_64-unknown-linux-gnu"]
        "#;

        let config = Config::from_str(toml).unwrap();
        assert_eq!(config.targets.default, vec!["x86_64-unknown-linux-gnu"]);
    }

    #[test]
    fn test_parse_full_config() {
        let toml = r#"
            [targets]
            default = ["x86_64-unknown-linux-gnu", "x86_64-pc-windows-gnu"]

            [build]
            parallel = true
            jobs = 4
            cache = true
            force_container = false
            cargo_flags = ["--verbose"]

            [container]
            runtime = "docker"
            use_when = "always"
            registry = "ghcr.io/xcargo"
            pull_policy = "if-not-present"

            [profiles.release-all]
            targets = ["x86_64-unknown-linux-gnu", "x86_64-pc-windows-gnu"]
        "#;

        let config = Config::from_str(toml).unwrap();
        assert_eq!(config.targets.default.len(), 2);
        assert_eq!(config.build.jobs, Some(4));
        assert_eq!(config.container.runtime, "docker");
        assert!(config.profiles.contains_key("release-all"));
    }

    #[test]
    fn test_custom_target_config() {
        let toml = r#"
            [targets]
            default = ["x86_64-pc-windows-gnu"]

            [targets."x86_64-pc-windows-gnu"]
            linker = "x86_64-w64-mingw32-gcc"
            force_container = false

            [targets."x86_64-pc-windows-gnu".env]
            CC = "x86_64-w64-mingw32-gcc"
        "#;

        let config = Config::from_str(toml).unwrap();
        let target_config = config.get_target_config("x86_64-pc-windows-gnu").unwrap();
        assert_eq!(target_config.linker, Some("x86_64-w64-mingw32-gcc".to_string()));
        assert_eq!(target_config.force_container, Some(false));
        assert_eq!(target_config.env.get("CC"), Some(&"x86_64-w64-mingw32-gcc".to_string()));
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        assert!(config.validate().is_ok());

        // Invalid runtime
        config.container.runtime = "invalid".to_string();
        assert!(config.validate().is_err());

        // Fix runtime, test invalid pull policy
        config.container.runtime = "auto".to_string();
        config.container.pull_policy = "invalid".to_string();
        assert!(config.validate().is_err());

        // Fix pull policy, test invalid jobs
        config.container.pull_policy = "always".to_string();
        config.build.jobs = Some(0);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_merge() {
        let mut base = Config::default();
        base.targets.default = vec!["linux".to_string()];
        base.build.parallel = false;

        let mut override_config = Config::default();
        override_config.targets.default = vec!["windows".to_string()];
        override_config.build.jobs = Some(8);
        // Override's default is true, so it will override base's false
        assert!(override_config.build.parallel);

        base.merge(&override_config);

        assert_eq!(base.targets.default, vec!["windows"]);
        assert_eq!(base.build.jobs, Some(8));
        assert!(base.build.parallel); // Merged with other's value (default true)
    }

    #[test]
    fn test_to_toml() {
        let config = Config::default();
        let toml = config.to_toml().unwrap();
        assert!(toml.contains("[targets]"));
        assert!(toml.contains("[build]"));
        assert!(toml.contains("[container]"));
    }
}
