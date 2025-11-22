//! Container runtime abstraction layer

use crate::error::{Error, Result};
use std::process::Command;

/// Container runtime type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeType {
    /// Automatically detect available runtime
    Auto,
    /// Use Docker
    Docker,
    /// Use Podman
    Podman,
}

impl RuntimeType {
    /// Parse from string
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "docker" => Ok(Self::Docker),
            "podman" => Ok(Self::Podman),
            _ => Err(Error::Config(format!("Unknown runtime type: {s}"))),
        }
    }
}

/// Container runtime trait
pub trait ContainerRuntime: Send + Sync {
    /// Check if this runtime is available
    fn is_available(&self) -> bool;

    /// Get the runtime name
    fn name(&self) -> &str;

    /// Pull a container image
    fn pull_image(&self, image: &str) -> Result<()>;

    /// Run a command in a container
    fn run(
        &self,
        image: &str,
        command: &[String],
        volumes: &[(String, String)],
        env: &[(String, String)],
        workdir: &str,
    ) -> Result<()>;

    /// List available images
    fn list_images(&self) -> Result<Vec<String>>;
}

/// Docker runtime implementation
pub struct DockerRuntime;

impl DockerRuntime {
    pub fn new() -> Self {
        Self
    }
}

impl ContainerRuntime for DockerRuntime {
    fn is_available(&self) -> bool {
        Command::new("docker")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn name(&self) -> &'static str {
        "docker"
    }

    fn pull_image(&self, image: &str) -> Result<()> {
        let status = Command::new("docker")
            .arg("pull")
            .arg(image)
            .status()
            .map_err(|e| Error::Container(format!("Failed to execute docker pull: {e}")))?;

        if status.success() {
            Ok(())
        } else {
            Err(Error::Container(format!("Failed to pull image: {image}")))
        }
    }

    fn run(
        &self,
        image: &str,
        command: &[String],
        volumes: &[(String, String)],
        env: &[(String, String)],
        workdir: &str,
    ) -> Result<()> {
        let mut cmd = Command::new("docker");
        cmd.arg("run").arg("--rm").arg("-it").arg("-w").arg(workdir);

        // Add volumes
        for (host, container) in volumes {
            cmd.arg("-v").arg(format!("{host}:{container}"));
        }

        // Add environment variables
        for (key, value) in env {
            cmd.arg("-e").arg(format!("{key}={value}"));
        }

        // Add image
        cmd.arg(image);

        // Add command
        for arg in command {
            cmd.arg(arg);
        }

        let status = cmd
            .status()
            .map_err(|e| Error::Container(format!("Failed to execute docker run: {e}")))?;

        if status.success() {
            Ok(())
        } else {
            Err(Error::Container("Container build failed".to_string()))
        }
    }

    fn list_images(&self) -> Result<Vec<String>> {
        let output = Command::new("docker")
            .arg("images")
            .arg("--format")
            .arg("{{.Repository}}:{{.Tag}}")
            .output()
            .map_err(|e| Error::Container(format!("Failed to list images: {e}")))?;

        if output.status.success() {
            let images = String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(std::string::ToString::to_string)
                .collect();
            Ok(images)
        } else {
            Err(Error::Container("Failed to list images".to_string()))
        }
    }
}

/// Podman runtime implementation
pub struct PodmanRuntime;

impl PodmanRuntime {
    pub fn new() -> Self {
        Self
    }
}

impl ContainerRuntime for PodmanRuntime {
    fn is_available(&self) -> bool {
        Command::new("podman")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn name(&self) -> &'static str {
        "podman"
    }

    fn pull_image(&self, image: &str) -> Result<()> {
        let status = Command::new("podman")
            .arg("pull")
            .arg(image)
            .status()
            .map_err(|e| Error::Container(format!("Failed to execute podman pull: {e}")))?;

        if status.success() {
            Ok(())
        } else {
            Err(Error::Container(format!("Failed to pull image: {image}")))
        }
    }

    fn run(
        &self,
        image: &str,
        command: &[String],
        volumes: &[(String, String)],
        env: &[(String, String)],
        workdir: &str,
    ) -> Result<()> {
        let mut cmd = Command::new("podman");
        cmd.arg("run").arg("--rm").arg("-it").arg("-w").arg(workdir);

        // Add volumes
        for (host, container) in volumes {
            cmd.arg("-v").arg(format!("{host}:{container}"));
        }

        // Add environment variables
        for (key, value) in env {
            cmd.arg("-e").arg(format!("{key}={value}"));
        }

        // Add image
        cmd.arg(image);

        // Add command
        for arg in command {
            cmd.arg(arg);
        }

        let status = cmd
            .status()
            .map_err(|e| Error::Container(format!("Failed to execute podman run: {e}")))?;

        if status.success() {
            Ok(())
        } else {
            Err(Error::Container("Container build failed".to_string()))
        }
    }

    fn list_images(&self) -> Result<Vec<String>> {
        let output = Command::new("podman")
            .arg("images")
            .arg("--format")
            .arg("{{.Repository}}:{{.Tag}}")
            .output()
            .map_err(|e| Error::Container(format!("Failed to list images: {e}")))?;

        if output.status.success() {
            let images = String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(std::string::ToString::to_string)
                .collect();
            Ok(images)
        } else {
            Err(Error::Container("Failed to list images".to_string()))
        }
    }
}

/// Create a container runtime based on the type
pub fn create_runtime(runtime_type: RuntimeType) -> Result<Box<dyn ContainerRuntime>> {
    match runtime_type {
        RuntimeType::Auto => {
            // Try Docker first, then Podman
            let docker = DockerRuntime::new();
            if docker.is_available() {
                return Ok(Box::new(docker));
            }

            let podman = PodmanRuntime::new();
            if podman.is_available() {
                return Ok(Box::new(podman));
            }

            // Neither runtime is available - provide helpful error
            let host_os = std::env::consts::OS;
            Err(Error::container_not_found("docker/podman", host_os))
        }
        RuntimeType::Docker => {
            let docker = DockerRuntime::new();
            if docker.is_available() {
                Ok(Box::new(docker))
            } else {
                let host_os = std::env::consts::OS;
                Err(Error::container_not_found("docker", host_os))
            }
        }
        RuntimeType::Podman => {
            let podman = PodmanRuntime::new();
            if podman.is_available() {
                Ok(Box::new(podman))
            } else {
                let host_os = std::env::consts::OS;
                Err(Error::container_not_found("podman", host_os))
            }
        }
    }
}

/// Check if any container runtime is available (for informational purposes)
#[allow(dead_code)]
pub fn check_runtime_availability() -> Option<String> {
    let docker = DockerRuntime::new();
    if docker.is_available() {
        return Some("docker".to_string());
    }

    let podman = PodmanRuntime::new();
    if podman.is_available() {
        return Some("podman".to_string());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_type_from_str() {
        assert_eq!(RuntimeType::from_str("auto").unwrap(), RuntimeType::Auto);
        assert_eq!(
            RuntimeType::from_str("docker").unwrap(),
            RuntimeType::Docker
        );
        assert_eq!(
            RuntimeType::from_str("podman").unwrap(),
            RuntimeType::Podman
        );
        assert!(RuntimeType::from_str("invalid").is_err());
    }

    #[test]
    fn test_docker_runtime_name() {
        let runtime = DockerRuntime::new();
        assert_eq!(runtime.name(), "docker");
    }

    #[test]
    fn test_podman_runtime_name() {
        let runtime = PodmanRuntime::new();
        assert_eq!(runtime.name(), "podman");
    }
}
