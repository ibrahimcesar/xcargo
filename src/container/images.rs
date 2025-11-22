//! Container image selection for cross-compilation targets

use crate::error::{Error, Result};

/// Container image information
#[derive(Debug, Clone)]
pub struct CrossImage {
    /// Image repository
    pub repository: String,

    /// Image tag
    pub tag: String,

    /// Target triple this image supports
    pub target: String,
}

impl CrossImage {
    /// Get the full image name (repository:tag)
    #[must_use] 
    pub fn full_name(&self) -> String {
        format!("{}:{}", self.repository, self.tag)
    }
}

/// Image selector for choosing appropriate images
pub struct ImageSelector {
    /// Image registry (default: ghcr.io/cross-rs)
    registry: String,
}

impl ImageSelector {
    /// Create a new image selector
    #[must_use] 
    pub fn new() -> Self {
        Self {
            registry: "ghcr.io/cross-rs".to_string(),
        }
    }

    /// Create with custom registry
    #[must_use] 
    pub fn with_registry(registry: String) -> Self {
        Self { registry }
    }

    /// Select appropriate image for a target
    pub fn select_for_target(&self, target: &str) -> Result<CrossImage> {
        let (image_name, tag) = match target {
            // Linux targets
            "x86_64-unknown-linux-gnu" => ("x86_64-unknown-linux-gnu", "latest"),
            "x86_64-unknown-linux-musl" => ("x86_64-unknown-linux-musl", "latest"),
            "aarch64-unknown-linux-gnu" => ("aarch64-unknown-linux-gnu", "latest"),
            "aarch64-unknown-linux-musl" => ("aarch64-unknown-linux-musl", "latest"),
            "armv7-unknown-linux-gnueabihf" => ("armv7-unknown-linux-gnueabihf", "latest"),
            "arm-unknown-linux-gnueabihf" => ("arm-unknown-linux-gnueabihf", "latest"),

            // Windows targets
            "x86_64-pc-windows-gnu" => ("x86_64-pc-windows-gnu", "latest"),

            // macOS targets - cross-rs doesn't have macOS images, would need osxcross
            "x86_64-apple-darwin" | "aarch64-apple-darwin" => {
                return Err(Error::Container(format!(
                    "No container image available for macOS target: {target}\nConsider using osxcross or build on macOS"
                )));
            }

            // Android targets
            "aarch64-linux-android" => ("aarch64-linux-android", "latest"),
            "armv7-linux-androideabi" => ("armv7-linux-androideabi", "latest"),
            "x86_64-linux-android" => ("x86_64-linux-android", "latest"),
            "i686-linux-android" => ("i686-linux-android", "latest"),

            // WebAssembly
            "wasm32-unknown-unknown" => {
                return Err(Error::Container(
                    "WebAssembly doesn't require containers - use native build".to_string(),
                ));
            }

            // Unknown target
            _ => {
                return Err(Error::Container(format!(
                    "No container image mapping for target: {target}\nYou can specify a custom image in xcargo.toml"
                )));
            }
        };

        Ok(CrossImage {
            repository: format!("{}/{}", self.registry, image_name),
            tag: tag.to_string(),
            target: target.to_string(),
        })
    }

    /// List all supported targets with images
    #[must_use] 
    pub fn supported_targets(&self) -> Vec<&str> {
        vec![
            // Linux
            "x86_64-unknown-linux-gnu",
            "x86_64-unknown-linux-musl",
            "aarch64-unknown-linux-gnu",
            "aarch64-unknown-linux-musl",
            "armv7-unknown-linux-gnueabihf",
            "arm-unknown-linux-gnueabihf",
            // Windows
            "x86_64-pc-windows-gnu",
            // Android
            "aarch64-linux-android",
            "armv7-linux-androideabi",
            "x86_64-linux-android",
            "i686-linux-android",
        ]
    }
}

impl Default for ImageSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_linux_target() {
        let selector = ImageSelector::new();
        let image = selector
            .select_for_target("x86_64-unknown-linux-gnu")
            .unwrap();
        assert_eq!(image.target, "x86_64-unknown-linux-gnu");
        assert!(image.full_name().contains("cross-rs"));
    }

    #[test]
    fn test_select_windows_target() {
        let selector = ImageSelector::new();
        let image = selector.select_for_target("x86_64-pc-windows-gnu").unwrap();
        assert_eq!(image.target, "x86_64-pc-windows-gnu");
    }

    #[test]
    fn test_macos_target_returns_error() {
        let selector = ImageSelector::new();
        assert!(selector.select_for_target("x86_64-apple-darwin").is_err());
    }

    #[test]
    fn test_wasm_target_returns_error() {
        let selector = ImageSelector::new();
        assert!(selector
            .select_for_target("wasm32-unknown-unknown")
            .is_err());
    }

    #[test]
    fn test_unknown_target_returns_error() {
        let selector = ImageSelector::new();
        assert!(selector.select_for_target("unknown-target").is_err());
    }

    #[test]
    fn test_supported_targets_not_empty() {
        let selector = ImageSelector::new();
        assert!(!selector.supported_targets().is_empty());
    }
}
