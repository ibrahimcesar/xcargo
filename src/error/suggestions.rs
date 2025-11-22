//! Error suggestion and hint generation

use super::Error;

impl Error {
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
            Error::ConfigParse { path, .. } => Some(format!("Check {path} for syntax errors")),
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
            "macos" => {
                "Install Docker Desktop: https://www.docker.com/products/docker-desktop\n\
                 Or Podman: brew install podman && podman machine init && podman machine start"
                    .to_string()
            }
            "linux" => {
                "Install Docker: sudo apt install docker.io && sudo systemctl start docker\n\
                 Or Podman: sudo apt install podman"
                    .to_string()
            }
            "windows" => {
                "Install Docker Desktop: https://www.docker.com/products/docker-desktop\n\
                 Or Podman: winget install RedHat.Podman"
                    .to_string()
            }
            _ => format!("Install {runtime} or a compatible container runtime"),
        };

        Error::ContainerNotAvailable {
            runtime: runtime.to_string(),
            install_hint,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linker_not_found_macos_windows() {
        let err = Error::linker_not_found("x86_64-w64-mingw32-gcc", "x86_64-pc-windows-gnu", "macos");
        match err {
            Error::LinkerMissing { install_hint, .. } => {
                assert!(install_hint.contains("mingw-w64"));
            }
            _ => panic!("Expected LinkerMissing error"),
        }
    }

    #[test]
    fn test_container_not_found_linux() {
        let err = Error::container_not_found("docker", "linux");
        match err {
            Error::ContainerNotAvailable { install_hint, .. } => {
                assert!(install_hint.contains("docker.io"));
            }
            _ => panic!("Expected ContainerNotAvailable error"),
        }
    }

    #[test]
    fn test_suggestion_invalid_target() {
        let err = Error::InvalidTarget {
            target: "x86-linux".to_string(),
            suggestions: vec!["x86_64-unknown-linux-gnu".to_string()],
        };
        let suggestion = err.suggestion();
        assert!(suggestion.is_some());
        assert!(suggestion.unwrap().contains("Did you mean"));
    }

    #[test]
    fn test_hint_target_not_found() {
        let err = Error::TargetNotFound("invalid-target".to_string());
        let hint = err.hint();
        assert!(hint.is_some());
        assert!(hint.unwrap().contains("xcargo target list"));
    }
}
