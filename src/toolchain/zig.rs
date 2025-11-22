//! Zig-based cross-compilation support

use crate::error::{Error, Result};
use crate::target::Target;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Zig toolchain for cross-compilation
pub struct ZigToolchain {
    /// Path to zig binary
    zig_path: PathBuf,

    /// Zig version
    version: String,

    /// Cache directory for wrapper scripts
    cache_dir: PathBuf,
}

impl ZigToolchain {
    /// Detect if Zig is installed and return a `ZigToolchain` instance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xcargo::toolchain::zig::ZigToolchain;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// if let Some(zig) = ZigToolchain::detect()? {
    ///     println!("Zig found: {}", zig.version());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn detect() -> Result<Option<Self>> {
        // Try to find zig in PATH
        let zig_path = match which::which("zig") {
            Ok(path) => path,
            Err(_) => return Ok(None),
        };

        // Get version
        let output = Command::new(&zig_path)
            .arg("version")
            .output()
            .map_err(|e| Error::Toolchain(format!("Failed to get Zig version: {e}")))?;

        if !output.status.success() {
            return Ok(None);
        }

        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();

        // Use ~/.xcargo/zig-wrappers as cache directory
        let cache_dir = dirs::home_dir()
            .ok_or_else(|| Error::Toolchain("Could not determine home directory".to_string()))?
            .join(".xcargo")
            .join("zig-wrappers");

        Ok(Some(Self {
            zig_path,
            version,
            cache_dir,
        }))
    }

    /// Get the Zig version
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get the path to the Zig binary
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.zig_path
    }

    /// Check if Zig supports a target by triple name (static method)
    ///
    /// This can be called without having a `ZigToolchain` instance.
    #[must_use]
    pub fn supports_target_name(triple: &str) -> bool {
        match triple {
            // Linux targets (well-supported)
            "x86_64-unknown-linux-gnu" => true,
            "aarch64-unknown-linux-gnu" => true,
            "armv7-unknown-linux-gnueabihf" => true,
            "i686-unknown-linux-gnu" => true,
            "arm-unknown-linux-gnueabihf" => true,

            // musl targets (known issues with static linking)
            "x86_64-unknown-linux-musl" => true, // Works but may have duplicate symbol issues
            "aarch64-unknown-linux-musl" => true,

            // Windows targets
            "x86_64-pc-windows-gnu" => true,
            "i686-pc-windows-gnu" => true,

            // macOS targets (not supported - Zig can't build for macOS on non-macOS)
            triple if triple.contains("apple-darwin") => false,

            // WebAssembly (may work but untested)
            triple if triple.contains("wasm32") => false,

            // Unknown target
            _ => false,
        }
    }

    /// Check if Zig can cross-compile to a target
    ///
    /// Zig supports many targets out of the box. This function checks if the
    /// target is supported by Zig.
    #[must_use]
    pub fn supports_target(&self, target: &Target) -> bool {
        // Zig supports most Linux targets
        // Known supported targets:
        // - x86_64-unknown-linux-gnu
        // - x86_64-unknown-linux-musl (with caveats)
        // - aarch64-unknown-linux-gnu
        // - aarch64-unknown-linux-musl
        // - armv7-unknown-linux-gnueabihf
        // - i686-unknown-linux-gnu

        Self::supports_target_name(&target.triple)
    }

    /// Get the Zig target triple for a Rust target
    ///
    /// Converts Rust target triple to Zig target triple format
    fn zig_target_for_rust_target(target: &Target) -> Option<String> {
        match target.triple.as_str() {
            "x86_64-unknown-linux-gnu" => Some("x86_64-linux-gnu".to_string()),
            "x86_64-unknown-linux-musl" => Some("x86_64-linux-musl".to_string()),
            "aarch64-unknown-linux-gnu" => Some("aarch64-linux-gnu".to_string()),
            "aarch64-unknown-linux-musl" => Some("aarch64-linux-musl".to_string()),
            "armv7-unknown-linux-gnueabihf" => Some("arm-linux-gnueabihf".to_string()),
            "arm-unknown-linux-gnueabihf" => Some("arm-linux-gnueabihf".to_string()),
            "i686-unknown-linux-gnu" => Some("i386-linux-gnu".to_string()),
            "x86_64-pc-windows-gnu" => Some("x86_64-windows-gnu".to_string()),
            "i686-pc-windows-gnu" => Some("i686-windows-gnu".to_string()),
            _ => None,
        }
    }

    /// Create wrapper scripts for a target
    ///
    /// Creates executable wrapper scripts that invoke `zig cc -target <target>` and `zig ar`.
    /// These wrappers are needed because Cargo expects a single executable path for CC/AR,
    /// not a command with arguments.
    pub fn create_wrappers(&self, target: &Target) -> Result<HashMap<String, PathBuf>> {
        let zig_target = Self::zig_target_for_rust_target(target).ok_or_else(|| {
            Error::Toolchain(format!("Target {} not supported by Zig", target.triple))
        })?;

        // Create cache directory
        fs::create_dir_all(&self.cache_dir).map_err(|e| {
            Error::Toolchain(format!("Failed to create Zig wrapper cache directory: {e}"))
        })?;

        let mut wrappers = HashMap::new();

        // Create CC wrapper
        let cc_wrapper_path = self.cache_dir.join(format!("{}-cc", target.triple));
        let cc_wrapper_content = if cfg!(windows) {
            format!("@echo off\nzig cc -target {zig_target} %*\n")
        } else {
            format!("#!/bin/sh\nexec zig cc -target {zig_target} \"$@\"\n")
        };

        fs::write(&cc_wrapper_path, cc_wrapper_content)
            .map_err(|e| Error::Toolchain(format!("Failed to create CC wrapper: {e}")))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&cc_wrapper_path)
                .map_err(|e| Error::Toolchain(format!("Failed to get wrapper permissions: {e}")))?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&cc_wrapper_path, perms)
                .map_err(|e| Error::Toolchain(format!("Failed to set wrapper permissions: {e}")))?;
        }

        wrappers.insert("CC".to_string(), cc_wrapper_path.clone());
        wrappers.insert("LINKER".to_string(), cc_wrapper_path);

        // Create AR wrapper (same for all targets)
        let ar_wrapper_path = self.cache_dir.join("zig-ar");
        if !ar_wrapper_path.exists() {
            let ar_wrapper_content = if cfg!(windows) {
                "@echo off\nzig ar %*\n"
            } else {
                "#!/bin/sh\nexec zig ar \"$@\"\n"
            };

            fs::write(&ar_wrapper_path, ar_wrapper_content)
                .map_err(|e| Error::Toolchain(format!("Failed to create AR wrapper: {e}")))?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&ar_wrapper_path)
                    .map_err(|e| {
                        Error::Toolchain(format!("Failed to get AR wrapper permissions: {e}"))
                    })?
                    .permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&ar_wrapper_path, perms).map_err(|e| {
                    Error::Toolchain(format!("Failed to set AR wrapper permissions: {e}"))
                })?;
            }
        }

        wrappers.insert("AR".to_string(), ar_wrapper_path);

        Ok(wrappers)
    }

    /// Get environment variables for cross-compiling to a target
    ///
    /// Returns a `HashMap` of environment variables that should be set when
    /// cross-compiling to the target using Zig.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xcargo::toolchain::zig::ZigToolchain;
    /// use xcargo::target::Target;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let zig = ZigToolchain::detect()?.expect("Zig not found");
    /// let target = Target::from_triple("x86_64-unknown-linux-gnu")?;
    /// let env = zig.environment_for_target(&target)?;
    ///
    /// for (key, value) in env {
    ///     println!("{}={}", key, value.display());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn environment_for_target(&self, target: &Target) -> Result<HashMap<String, PathBuf>> {
        if !self.supports_target(target) {
            return Err(Error::Toolchain(format!(
                "Target {} is not supported by Zig",
                target.triple
            )));
        }

        // Create wrapper scripts
        let wrappers = self.create_wrappers(target)?;

        let mut env = HashMap::new();

        // Set CC and AR
        if let Some(cc) = wrappers.get("CC") {
            env.insert("CC".to_string(), cc.clone());
        }
        if let Some(ar) = wrappers.get("AR") {
            env.insert("AR".to_string(), ar.clone());
        }

        // Set CARGO_TARGET_*_LINKER
        if let Some(linker) = wrappers.get("LINKER") {
            let linker_env_var = format!(
                "CARGO_TARGET_{}_LINKER",
                target.triple.to_uppercase().replace('-', "_")
            );
            env.insert(linker_env_var, linker.clone());
        }

        Ok(env)
    }

    /// Clean up wrapper scripts cache
    pub fn clean_cache(&self) -> Result<()> {
        if self.cache_dir.exists() {
            fs::remove_dir_all(&self.cache_dir)
                .map_err(|e| Error::Toolchain(format!("Failed to clean Zig wrapper cache: {e}")))?;
        }
        Ok(())
    }

    /// Get a summary of Zig's capabilities
    #[must_use]
    pub fn info(&self) -> String {
        format!(
            "Zig {} ({})\nSupports: Linux (x86_64, aarch64, armv7), Windows (x86_64, i686)\nLimitations: musl may have linking issues, macOS/wasm not supported",
            self.version,
            self.zig_path.display()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_zig() {
        // Only runs if Zig is installed
        if let Ok(Some(zig)) = ZigToolchain::detect() {
            assert!(!zig.version().is_empty());
            assert!(zig.path().exists());
        }
    }

    #[test]
    fn test_supports_target() {
        if let Ok(Some(zig)) = ZigToolchain::detect() {
            // Linux targets should be supported
            let target = Target::from_triple("x86_64-unknown-linux-gnu").unwrap();
            assert!(zig.supports_target(&target));

            // macOS targets should not be supported (on non-macOS hosts)
            let target = Target::from_triple("x86_64-apple-darwin").unwrap();
            assert!(!zig.supports_target(&target));
        }
    }

    #[test]
    fn test_zig_target_conversion() {
        let target = Target::from_triple("x86_64-unknown-linux-gnu").unwrap();
        let zig_target = ZigToolchain::zig_target_for_rust_target(&target);
        assert_eq!(zig_target, Some("x86_64-linux-gnu".to_string()));

        let target = Target::from_triple("aarch64-unknown-linux-gnu").unwrap();
        let zig_target = ZigToolchain::zig_target_for_rust_target(&target);
        assert_eq!(zig_target, Some("aarch64-linux-gnu".to_string()));
    }

    #[test]
    fn test_create_wrappers() {
        if let Ok(Some(zig)) = ZigToolchain::detect() {
            let target = Target::from_triple("x86_64-unknown-linux-gnu").unwrap();
            let wrappers = zig.create_wrappers(&target);

            if wrappers.is_ok() {
                let wrappers = wrappers.unwrap();
                assert!(wrappers.contains_key("CC"));
                assert!(wrappers.contains_key("AR"));
                assert!(wrappers.contains_key("LINKER"));

                // Cleanup
                let _ = zig.clean_cache();
            }
        }
    }
}
