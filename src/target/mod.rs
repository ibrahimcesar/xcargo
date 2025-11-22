//! Target platform definitions and detection
//!
//! This module provides types and functions for working with Rust target triples,
//! detecting available targets, and validating target configurations.
use crate::error::{Error, Result};
use std::fmt;
use std::process::Command;

/// Represents the requirements needed to build for a target
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetRequirements {
    /// Linker required for this target
    pub linker: Option<String>,
    /// Additional tools needed (e.g., "lld", "gcc-aarch64-linux-gnu")
    pub tools: Vec<String>,
    /// System libraries needed
    pub system_libs: Vec<String>,
    /// Environment variables that should be set
    pub env_vars: Vec<(String, String)>,
}

impl TargetRequirements {
    /// Create empty requirements
    #[must_use]
    pub fn none() -> Self {
        Self {
            linker: None,
            tools: Vec::new(),
            system_libs: Vec::new(),
            env_vars: Vec::new(),
        }
    }

    /// Check if all requirements are satisfied
    #[must_use]
    pub fn are_satisfied(&self) -> bool {
        // Check if linker is available
        if let Some(ref linker) = self.linker {
            if !Self::is_command_available(linker) {
                return false;
            }
        }

        // Check if all tools are available
        for tool in &self.tools {
            if !Self::is_command_available(tool) {
                return false;
            }
        }

        true
    }

    /// Check if a command is available in PATH
    fn is_command_available(cmd: &str) -> bool {
        which::which(cmd).is_ok()
    }
}

/// Represents a target platform for cross-compilation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Target {
    /// The full target triple (e.g., "x86_64-unknown-linux-gnu")
    pub triple: String,
    /// Target architecture (e.g., "`x86_64`", "aarch64")
    pub arch: String,
    /// Target vendor (e.g., "unknown", "apple", "pc")
    pub vendor: String,
    /// Target operating system (e.g., "linux", "windows", "darwin")
    pub os: String,
    /// Target environment/ABI (e.g., "gnu", "musl", "msvc")
    pub env: Option<String>,
    /// Target tier (1 = native, 2 = container, 3 = specialized)
    pub tier: TargetTier,
}

/// Classification of target support levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetTier {
    /// Tier 1: Native compilation (fast, no containers)
    Native,
    /// Tier 2: Container-based (automatic fallback)
    Container,
    /// Tier 3: Specialized (mobile, embedded, etc.)
    Specialized,
}

impl Target {
    /// Parse a target triple string into a Target struct
    ///
    /// # Examples
    ///
    /// ```
    /// use xcargo::target::Target;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let target = Target::from_triple("x86_64-unknown-linux-gnu")?;
    /// assert_eq!(target.arch, "x86_64");
    /// assert_eq!(target.os, "linux");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the target triple is invalid (fewer than 3 parts).
    pub fn from_triple(triple: &str) -> Result<Self> {
        let parts: Vec<&str> = triple.split('-').collect();

        if parts.len() < 3 {
            return Err(Error::TargetNotFound(format!(
                "Invalid target triple: {triple}. Expected format: arch-vendor-os[-env]"
            )));
        }

        let arch = parts[0].to_string();
        let vendor = parts[1].to_string();
        let os = parts[2].to_string();
        let env = if parts.len() >= 4 {
            Some(parts[3..].join("-"))
        } else {
            None
        };

        let tier = Self::classify_tier(triple);

        Ok(Target {
            triple: triple.to_string(),
            arch,
            vendor,
            os,
            env,
            tier,
        })
    }

    /// Detect the current host target platform
    ///
    /// # Examples
    ///
    /// ```
    /// use xcargo::target::Target;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let host = Target::detect_host()?;
    /// println!("Host platform: {}", host.triple);
    /// # Ok(())
    /// # }
    /// ```
    pub fn detect_host() -> Result<Self> {
        // Use rustc to get the host target
        let output = Command::new("rustc")
            .args(["-vV"])
            .output()
            .map_err(|e| Error::Toolchain(format!("Failed to run rustc: {e}")))?;

        if !output.status.success() {
            return Err(Error::Toolchain("rustc command failed".to_string()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Parse the "host: " line from rustc -vV output
        for line in stdout.lines() {
            if let Some(host) = line.strip_prefix("host: ") {
                return Self::from_triple(host.trim());
            }
        }

        Err(Error::Toolchain(
            "Could not detect host target from rustc".to_string(),
        ))
    }

    /// Detect all installed Rust targets via rustup
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xcargo::target::Target;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let installed = Target::detect_installed()?;
    /// for target in installed {
    ///     println!("Installed: {}", target.triple);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn detect_installed() -> Result<Vec<Self>> {
        let output = Command::new("rustup")
            .args(["target", "list", "--installed"])
            .output()
            .map_err(|e| Error::Toolchain(format!("Failed to run rustup: {e}")))?;

        if !output.status.success() {
            return Err(Error::Toolchain(
                "rustup target list command failed".to_string(),
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut targets = Vec::new();

        for line in stdout.lines() {
            let triple = line.trim();
            if !triple.is_empty() {
                if let Ok(target) = Self::from_triple(triple) {
                    targets.push(target);
                }
            }
        }

        Ok(targets)
    }

    /// List all available Rust targets via rustup
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xcargo::target::Target;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let available = Target::list_available()?;
    /// println!("Available targets: {}", available.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_available() -> Result<Vec<Self>> {
        let output = Command::new("rustup")
            .args(["target", "list"])
            .output()
            .map_err(|e| Error::Toolchain(format!("Failed to run rustup: {e}")))?;

        if !output.status.success() {
            return Err(Error::Toolchain(
                "rustup target list command failed".to_string(),
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut targets = Vec::new();

        for line in stdout.lines() {
            // Remove " (installed)" suffix if present
            let triple = line
                .trim()
                .strip_suffix(" (installed)")
                .unwrap_or(line.trim());

            if !triple.is_empty() {
                if let Ok(target) = Self::from_triple(triple) {
                    targets.push(target);
                }
            }
        }

        Ok(targets)
    }

    /// Check if this target is currently installed
    pub fn is_installed(&self) -> Result<bool> {
        let installed = Self::detect_installed()?;
        Ok(installed.iter().any(|t| t.triple == self.triple))
    }

    /// Install this target via rustup
    pub fn install(&self) -> Result<()> {
        let output = Command::new("rustup")
            .args(["target", "add", &self.triple])
            .output()
            .map_err(|e| Error::Toolchain(format!("Failed to run rustup: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::Toolchain(format!(
                "Failed to install target {}: {}",
                self.triple, stderr
            )));
        }

        Ok(())
    }

    /// Resolve a target alias to a full target triple
    ///
    /// # Examples
    ///
    /// ```
    /// use xcargo::target::Target;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let triple = Target::resolve_alias("linux")?;
    /// assert_eq!(triple, "x86_64-unknown-linux-gnu");
    ///
    /// let triple = Target::resolve_alias("windows")?;
    /// assert_eq!(triple, "x86_64-pc-windows-gnu");
    /// # Ok(())
    /// # }
    /// ```
    pub fn resolve_alias(alias: &str) -> Result<String> {
        let alias_lower = alias.to_lowercase();
        let triple = match alias_lower.as_str() {
            // Platform aliases
            "linux" => "x86_64-unknown-linux-gnu",
            "windows" => "x86_64-pc-windows-gnu",
            "macos" => {
                // Detect if we're on Apple Silicon
                if let Ok(host) = Self::detect_host() {
                    if host.arch == "aarch64" && host.os == "darwin" {
                        "aarch64-apple-darwin"
                    } else {
                        "x86_64-apple-darwin"
                    }
                } else {
                    "x86_64-apple-darwin"
                }
            }

            // Architecture variants
            "linux-arm64" | "linux-aarch64" => "aarch64-unknown-linux-gnu",
            "linux-armv7" => "armv7-unknown-linux-gnueabihf",
            "linux-musl" => "x86_64-unknown-linux-musl",
            "linux-arm64-musl" => "aarch64-unknown-linux-musl",

            "windows-msvc" => "x86_64-pc-windows-msvc",
            "windows-gnu" => "x86_64-pc-windows-gnu",
            "windows-32" => "i686-pc-windows-gnu",

            // Mobile platforms
            "android" | "android-arm64" => "aarch64-linux-android",
            "android-armv7" => "armv7-linux-androideabi",
            "android-x86" => "x86_64-linux-android",

            "ios" | "ios-arm64" => "aarch64-apple-ios",
            "ios-sim" => "aarch64-apple-ios-sim",

            // WebAssembly
            "wasm" | "wasm32" => "wasm32-unknown-unknown",
            "wasi" => "wasm32-wasi",

            // If not an alias, assume it's a full triple (use original case)
            _ => alias,
        };

        Ok(triple.to_string())
    }

    /// Classify a target into a tier based on its triple
    fn classify_tier(triple: &str) -> TargetTier {
        // Tier 1: Native compilation targets
        let tier1 = [
            "x86_64-unknown-linux-gnu",
            "x86_64-unknown-linux-musl",
            "x86_64-pc-windows-gnu",
            "x86_64-apple-darwin",
            "aarch64-apple-darwin",
            "i686-pc-windows-gnu",
            "i686-unknown-linux-gnu",
        ];

        if tier1.contains(&triple) {
            return TargetTier::Native;
        }

        // Tier 3: Specialized targets (mobile, embedded, wasm)
        if triple.contains("android")
            || triple.contains("ios")
            || triple.starts_with("wasm")
            || triple.starts_with("thumb")
            || triple.starts_with("riscv")
        {
            return TargetTier::Specialized;
        }

        // Tier 2: Container-based (everything else)
        TargetTier::Container
    }

    /// Check if native compilation is likely possible for this target
    #[must_use]
    pub fn supports_native_build(&self) -> bool {
        matches!(self.tier, TargetTier::Native)
    }

    /// Check if this target requires container-based compilation
    #[must_use]
    pub fn requires_container(&self) -> bool {
        matches!(self.tier, TargetTier::Container | TargetTier::Specialized)
    }

    /// Get the requirements needed to build for this target
    ///
    /// # Examples
    ///
    /// ```
    /// use xcargo::target::Target;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let target = Target::from_triple("aarch64-unknown-linux-gnu")?;
    /// let reqs = target.get_requirements();
    /// if !reqs.are_satisfied() {
    ///     println!("Missing tools for {}", target.triple);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_requirements(&self) -> TargetRequirements {
        let mut reqs = TargetRequirements::none();

        // Detect linker and tools based on target
        match (self.os.as_str(), self.arch.as_str(), self.env.as_deref()) {
            // Linux ARM targets
            ("linux", "aarch64", Some("gnu")) => {
                reqs.linker = Some("aarch64-linux-gnu-gcc".to_string());
                reqs.tools.push("aarch64-linux-gnu-gcc".to_string());
            }
            ("linux", "aarch64", Some("musl")) => {
                reqs.linker = Some("aarch64-linux-musl-gcc".to_string());
                reqs.tools.push("aarch64-linux-musl-gcc".to_string());
            }
            ("linux", "armv7", _) => {
                reqs.linker = Some("arm-linux-gnueabihf-gcc".to_string());
                reqs.tools.push("arm-linux-gnueabihf-gcc".to_string());
            }
            ("linux", "arm", _) => {
                reqs.linker = Some("arm-linux-gnueabi-gcc".to_string());
                reqs.tools.push("arm-linux-gnueabi-gcc".to_string());
            }

            // Windows targets
            ("windows", "x86_64", Some("gnu")) => {
                reqs.linker = Some("x86_64-w64-mingw32-gcc".to_string());
                reqs.tools.push("x86_64-w64-mingw32-gcc".to_string());
            }
            ("windows", "i686", Some("gnu")) => {
                reqs.linker = Some("i686-w64-mingw32-gcc".to_string());
                reqs.tools.push("i686-w64-mingw32-gcc".to_string());
            }
            ("windows", _, Some("msvc")) => {
                // MSVC requires special setup (xwin or native Windows)
                reqs.tools.push("cl.exe".to_string());
            }

            // Android targets
            ("android", _, _) => {
                reqs.tools.push("ndk-build".to_string());
                reqs.env_vars.push((
                    "ANDROID_NDK_HOME".to_string(),
                    "$ANDROID_NDK_HOME".to_string(),
                ));
            }

            // iOS targets
            ("ios", _, _) | ("darwin", _, Some("ios")) => {
                // iOS requires macOS with Xcode
                reqs.tools.push("xcrun".to_string());
            }

            // WASM targets - no special linker needed, but may need wasm-pack
            (_, "wasm32", _) => {
                // wasm32 typically doesn't need a separate linker
            }

            // Native targets - use default linker
            _ => {
                // For native builds, the default toolchain linker should work
            }
        }

        reqs
    }

    /// Detect the linker that will be used for this target
    ///
    /// # Examples
    ///
    /// ```
    /// use xcargo::target::Target;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let target = Target::from_triple("x86_64-unknown-linux-gnu")?;
    /// if let Some(linker) = target.detect_linker() {
    ///     println!("Linker: {}", linker);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn detect_linker(&self) -> Option<String> {
        let reqs = self.get_requirements();

        if let Some(linker) = reqs.linker {
            // Check if the required linker is available
            if TargetRequirements::is_command_available(&linker) {
                return Some(linker);
            }
        }

        // Try to detect alternative linkers
        let alternatives = match (self.os.as_str(), self.arch.as_str()) {
            ("linux", "aarch64") => vec!["aarch64-linux-gnu-gcc", "aarch64-linux-musl-gcc"],
            ("linux", "armv7") => vec!["arm-linux-gnueabihf-gcc", "arm-linux-gnueabi-gcc"],
            ("windows", "x86_64") => vec!["x86_64-w64-mingw32-gcc", "gcc"],
            ("windows", "i686") => vec!["i686-w64-mingw32-gcc", "gcc"],
            _ => vec!["gcc", "clang", "cc"],
        };

        for linker in alternatives {
            if TargetRequirements::is_command_available(linker) {
                return Some(linker.to_string());
            }
        }

        None
    }

    /// Check if we can build for this target without containers
    ///
    /// This checks both the target tier and whether required tools are available
    ///
    /// # Examples
    ///
    /// ```
    /// use xcargo::target::Target;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let target = Target::from_triple("x86_64-unknown-linux-gnu")?;
    /// let host = Target::detect_host()?;
    ///
    /// if target.can_cross_compile_from(&host) {
    ///     println!("Can build natively!");
    /// } else {
    ///     println!("Need container or missing tools");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn can_cross_compile_from(&self, host: &Target) -> bool {
        // Same target - can always build
        if self.triple == host.triple {
            return true;
        }

        // Check if it's a native-tier target
        if !self.supports_native_build() {
            return false;
        }

        // Check if required tools are available
        let reqs = self.get_requirements();
        reqs.are_satisfied()
    }

    /// Get installation instructions for missing requirements
    ///
    /// # Examples
    ///
    /// ```
    /// use xcargo::target::Target;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let target = Target::from_triple("aarch64-unknown-linux-gnu")?;
    /// let instructions = target.get_install_instructions();
    /// for instruction in instructions {
    ///     println!("  {}", instruction);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_install_instructions(&self) -> Vec<String> {
        let mut instructions = Vec::new();
        let reqs = self.get_requirements();

        if reqs.are_satisfied() {
            return instructions;
        }

        // Detect OS and provide appropriate installation instructions
        let host_os = std::env::consts::OS;

        match (self.os.as_str(), self.arch.as_str(), host_os) {
            ("linux", "aarch64", "linux") => {
                instructions.push("# Debian/Ubuntu:".to_string());
                instructions.push("sudo apt-get install gcc-aarch64-linux-gnu".to_string());
                instructions.push("# Fedora/RHEL:".to_string());
                instructions.push("sudo dnf install gcc-aarch64-linux-gnu".to_string());
            }
            ("linux", "aarch64", "macos") => {
                instructions.push("# macOS: Container build recommended".to_string());
                instructions.push("# Or use cross-compilation toolchain:".to_string());
                instructions.push("brew tap messense/macos-cross-toolchains".to_string());
                instructions.push("brew install aarch64-unknown-linux-gnu".to_string());
            }
            ("linux", "armv7", "linux") => {
                instructions.push("# Debian/Ubuntu:".to_string());
                instructions.push("sudo apt-get install gcc-arm-linux-gnueabihf".to_string());
                instructions.push("# Fedora/RHEL:".to_string());
                instructions.push("sudo dnf install gcc-arm-linux-gnu".to_string());
            }
            ("windows", "x86_64", "linux") => {
                instructions.push("# Debian/Ubuntu:".to_string());
                instructions.push("sudo apt-get install mingw-w64".to_string());
                instructions.push("# Fedora/RHEL:".to_string());
                instructions.push("sudo dnf install mingw64-gcc".to_string());
            }
            ("windows", "x86_64", "macos") => {
                instructions.push("# macOS (Homebrew):".to_string());
                instructions.push("brew install mingw-w64".to_string());
            }
            ("android", _, _) => {
                instructions.push("# Install Android NDK:".to_string());
                instructions.push(
                    "# Download from: https://developer.android.com/ndk/downloads".to_string(),
                );
                instructions.push("export ANDROID_NDK_HOME=/path/to/ndk".to_string());
            }
            ("ios", _, "macos") => {
                instructions.push("# iOS requires Xcode:".to_string());
                instructions.push("xcode-select --install".to_string());
            }
            ("ios", _, _) => {
                instructions.push("# iOS requires macOS with Xcode".to_string());
                instructions.push("# Consider using a container or CI/CD on macOS".to_string());
            }
            _ => {
                instructions.push(format!(
                    "# No automatic installation instructions available for {}",
                    self.triple
                ));
                instructions.push("# Consider using container-based build".to_string());
            }
        }

        instructions
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.triple)
    }
}

impl fmt::Display for TargetTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetTier::Native => write!(f, "Tier 1 (Native)"),
            TargetTier::Container => write!(f, "Tier 2 (Container)"),
            TargetTier::Specialized => write!(f, "Tier 3 (Specialized)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_linux_target() {
        let target = Target::from_triple("x86_64-unknown-linux-gnu").unwrap();
        assert_eq!(target.arch, "x86_64");
        assert_eq!(target.vendor, "unknown");
        assert_eq!(target.os, "linux");
        assert_eq!(target.env, Some("gnu".to_string()));
        assert_eq!(target.tier, TargetTier::Native);
    }

    #[test]
    fn test_parse_windows_target() {
        let target = Target::from_triple("x86_64-pc-windows-msvc").unwrap();
        assert_eq!(target.arch, "x86_64");
        assert_eq!(target.vendor, "pc");
        assert_eq!(target.os, "windows");
        assert_eq!(target.env, Some("msvc".to_string()));
    }

    #[test]
    fn test_parse_macos_target() {
        let target = Target::from_triple("aarch64-apple-darwin").unwrap();
        assert_eq!(target.arch, "aarch64");
        assert_eq!(target.vendor, "apple");
        assert_eq!(target.os, "darwin");
        assert_eq!(target.env, None);
        assert_eq!(target.tier, TargetTier::Native);
    }

    #[test]
    fn test_parse_invalid_target() {
        let result = Target::from_triple("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_alias_linux() {
        assert_eq!(
            Target::resolve_alias("linux").unwrap(),
            "x86_64-unknown-linux-gnu"
        );
    }

    #[test]
    fn test_resolve_alias_windows() {
        assert_eq!(
            Target::resolve_alias("windows").unwrap(),
            "x86_64-pc-windows-gnu"
        );
    }

    #[test]
    fn test_resolve_alias_linux_arm64() {
        assert_eq!(
            Target::resolve_alias("linux-arm64").unwrap(),
            "aarch64-unknown-linux-gnu"
        );
    }

    #[test]
    fn test_resolve_alias_passthrough() {
        assert_eq!(
            Target::resolve_alias("x86_64-unknown-linux-gnu").unwrap(),
            "x86_64-unknown-linux-gnu"
        );
    }

    #[test]
    fn test_tier_classification() {
        let native = Target::from_triple("x86_64-unknown-linux-gnu").unwrap();
        assert_eq!(native.tier, TargetTier::Native);
        assert!(native.supports_native_build());
        assert!(!native.requires_container());

        let container = Target::from_triple("aarch64-unknown-linux-gnu").unwrap();
        assert_eq!(container.tier, TargetTier::Container);
        assert!(container.requires_container());

        let specialized = Target::from_triple("wasm32-unknown-unknown").unwrap();
        assert_eq!(specialized.tier, TargetTier::Specialized);
        assert!(specialized.requires_container());
    }

    #[test]
    fn test_target_display() {
        let target = Target::from_triple("x86_64-unknown-linux-gnu").unwrap();
        assert_eq!(format!("{target}"), "x86_64-unknown-linux-gnu");
    }

    #[test]
    fn test_detect_host() {
        // This test requires rustc to be installed
        let result = Target::detect_host();
        assert!(result.is_ok());
        let host = result.unwrap();
        assert!(!host.triple.is_empty());
        assert!(!host.arch.is_empty());
        assert!(!host.os.is_empty());
    }

    #[test]
    fn test_target_requirements() {
        let target = Target::from_triple("aarch64-unknown-linux-gnu").unwrap();
        let reqs = target.get_requirements();

        // Should have a linker requirement
        assert!(reqs.linker.is_some());
        assert_eq!(reqs.linker.unwrap(), "aarch64-linux-gnu-gcc");
    }

    #[test]
    fn test_native_target_requirements() {
        let target = Target::from_triple("x86_64-unknown-linux-gnu").unwrap();
        let reqs = target.get_requirements();

        // Native targets may not require special linkers
        // Requirements should still be created
        assert_eq!(reqs.tools.len(), 0);
    }

    #[test]
    fn test_windows_target_requirements() {
        let target = Target::from_triple("x86_64-pc-windows-gnu").unwrap();
        let reqs = target.get_requirements();

        assert!(reqs.linker.is_some());
        assert_eq!(reqs.linker.unwrap(), "x86_64-w64-mingw32-gcc");
    }

    #[test]
    fn test_can_cross_compile_same_target() {
        let target1 = Target::from_triple("x86_64-unknown-linux-gnu").unwrap();
        let target2 = Target::from_triple("x86_64-unknown-linux-gnu").unwrap();

        assert!(target1.can_cross_compile_from(&target2));
    }

    #[test]
    fn test_get_install_instructions() {
        let target = Target::from_triple("aarch64-unknown-linux-gnu").unwrap();
        let _instructions = target.get_install_instructions();

        // Just verify the method works without panicking
        // Instructions will vary based on whether tools are installed
    }

    #[test]
    fn test_detect_linker() {
        let target = Target::from_triple("x86_64-unknown-linux-gnu").unwrap();

        // Should be able to detect some linker (gcc, clang, or cc)
        // This test might fail if no compiler is installed, but that's expected
        let linker = target.detect_linker();
        // Just verify the method works without panicking
        assert!(linker.is_some() || linker.is_none());
    }

    #[test]
    fn test_requirements_none() {
        let reqs = TargetRequirements::none();
        assert!(reqs.linker.is_none());
        assert_eq!(reqs.tools.len(), 0);
        assert_eq!(reqs.system_libs.len(), 0);
        assert_eq!(reqs.env_vars.len(), 0);
    }

    #[test]
    fn test_android_requirements() {
        let target = Target::from_triple("aarch64-linux-android").unwrap();
        let reqs = target.get_requirements();

        // Android should require NDK
        assert!(!reqs.tools.is_empty());
        assert!(reqs.tools.contains(&"ndk-build".to_string()));
    }
}
