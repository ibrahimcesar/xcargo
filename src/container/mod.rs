//! Container runtime integration for cross-compilation
//!
//! This module provides support for building in containers when native
//! cross-compilation toolchains are not available or practical.

use crate::error::{Error, Result};

mod images;
mod runtime;

pub use images::{CrossImage, ImageSelector};
pub use runtime::{ContainerRuntime, RuntimeType};

/// Container build configuration
#[derive(Debug, Clone)]
pub struct ContainerConfig {
    /// Runtime to use (docker, podman, auto)
    pub runtime: RuntimeType,

    /// Container image to use
    pub image: String,

    /// Additional volumes to mount
    pub volumes: Vec<(String, String)>,

    /// Environment variables to pass to container
    pub env: Vec<(String, String)>,

    /// Working directory inside container
    pub workdir: String,
}

impl Default for ContainerConfig {
    fn default() -> Self {
        Self {
            runtime: RuntimeType::Auto,
            image: String::new(),
            volumes: Vec::new(),
            env: Vec::new(),
            workdir: "/project".to_string(),
        }
    }
}

/// Container builder for executing builds in containers
pub struct ContainerBuilder {
    runtime: Box<dyn ContainerRuntime>,
    image_selector: ImageSelector,
}

impl ContainerBuilder {
    /// Create a new container builder
    pub fn new(runtime_type: RuntimeType) -> Result<Self> {
        let runtime = runtime::create_runtime(runtime_type)?;
        let image_selector = ImageSelector::new();

        Ok(Self {
            runtime,
            image_selector,
        })
    }

    /// Check if the container runtime is available
    #[must_use]
    pub fn is_available(&self) -> bool {
        self.runtime.is_available()
    }

    /// Get the runtime name
    #[must_use]
    pub fn runtime_name(&self) -> &str {
        self.runtime.name()
    }

    /// Select appropriate image for target
    pub fn select_image(&self, target: &str) -> Result<CrossImage> {
        self.image_selector.select_for_target(target)
    }

    /// Execute a build command in a container
    pub fn build(
        &self,
        target: &str,
        cargo_args: &[String],
        config: &ContainerConfig,
    ) -> Result<()> {
        // Verify runtime is available
        if !self.is_available() {
            return Err(Error::Container(format!(
                "Container runtime '{}' is not available",
                self.runtime_name()
            )));
        }

        // Select image if not specified
        let image = if config.image.is_empty() {
            self.select_image(target)?.full_name()
        } else {
            config.image.clone()
        };

        // Pull image if needed
        self.runtime.pull_image(&image)?;

        // Build the container command
        let mut volumes = config.volumes.clone();

        // Add current directory as volume
        let current_dir = std::env::current_dir()
            .map_err(|e| Error::Container(format!("Failed to get current directory: {e}")))?;
        let current_dir_str = current_dir.to_string_lossy().to_string();
        volumes.push((current_dir_str.clone(), config.workdir.clone()));

        // Add cargo cache volume for faster builds
        if let Ok(home) = std::env::var("HOME") {
            let cargo_cache = format!("{home}/.cargo");
            volumes.push((cargo_cache, "/root/.cargo".to_string()));
        }

        // Build cargo command
        let mut cmd = vec!["cargo".to_string(), "build".to_string()];
        cmd.push("--target".to_string());
        cmd.push(target.to_string());
        cmd.extend_from_slice(cargo_args);

        // Run in container
        self.runtime
            .run(&image, &cmd, &volumes, &config.env, &config.workdir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_config_default() {
        let config = ContainerConfig::default();
        assert_eq!(config.runtime, RuntimeType::Auto);
        assert_eq!(config.workdir, "/project");
    }

    #[test]
    fn test_container_builder_creation() {
        // This will succeed if docker/podman is available
        let builder = ContainerBuilder::new(RuntimeType::Auto);
        // Don't fail test if container runtime not available
        if let Ok(builder) = builder {
            assert!(!builder.runtime_name().is_empty());
        }
    }
}
