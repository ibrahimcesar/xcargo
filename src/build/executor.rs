//! Build execution and orchestration

use crate::config::Config;
use crate::error::{Error, Result};
use crate::output::{helpers, tips};
use crate::target::Target;
use crate::toolchain::zig::ZigToolchain;
use crate::toolchain::ToolchainManager;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use super::options::{BuildOptions, CargoOperation};

/// Build executor
pub struct Builder {
    /// Toolchain manager
    toolchain_manager: ToolchainManager,

    /// Configuration
    config: Config,

    /// Zig toolchain (if available)
    zig_toolchain: Option<ZigToolchain>,
}

impl Builder {
    /// Create a new builder
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xcargo::build::Builder;
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let builder = Builder::new()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Result<Self> {
        let toolchain_manager = ToolchainManager::new()?;
        let config = Config::discover()?.map(|(c, _)| c).unwrap_or_default();

        // Try to detect Zig for cross-compilation
        let zig_toolchain = ZigToolchain::detect().ok().flatten();

        Ok(Self {
            toolchain_manager,
            config,
            zig_toolchain,
        })
    }

    /// Create a builder with a specific configuration
    pub fn with_config(config: Config) -> Result<Self> {
        let toolchain_manager = ToolchainManager::new()?;
        let zig_toolchain = ZigToolchain::detect().ok().flatten();

        Ok(Self {
            toolchain_manager,
            config,
            zig_toolchain,
        })
    }

    /// Check if a Cargo.toml exists in current directory or parent directories
    fn has_cargo_toml() -> bool {
        let mut current_dir = std::env::current_dir().ok();

        while let Some(dir) = current_dir {
            let cargo_toml = dir.join("Cargo.toml");
            if cargo_toml.exists() {
                return true;
            }

            current_dir = dir.parent().map(|p| p.to_path_buf());
        }

        false
    }

    /// Build the current project
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xcargo::build::{Builder, BuildOptions};
    ///
    /// # fn example() -> xcargo::Result<()> {
    /// let builder = Builder::new()?;
    /// let options = BuildOptions {
    ///     target: Some("x86_64-pc-windows-gnu".to_string()),
    ///     release: true,
    ///     ..Default::default()
    /// };
    /// builder.build(&options)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(&self, options: &BuildOptions) -> Result<()> {
        helpers::section(format!("xcargo {}", options.operation.as_str()));

        // Check for Cargo.toml early to provide helpful error
        if !Self::has_cargo_toml() {
            helpers::error("No Cargo.toml found in current directory or parent directories");
            helpers::tip("Initialize a new Rust project with: cargo init");
            helpers::tip("Or navigate to an existing Rust project directory");
            return Err(Error::Config(
                "No Cargo.toml found. This doesn't appear to be a Rust project.".to_string(),
            ));
        }

        // Determine target
        let target_triple = if let Some(target) = &options.target {
            target.clone()
        } else if let Some(default_target) = self.config.targets.default.first() {
            helpers::info(format!(
                "Using default target from config: {default_target}"
            ));
            default_target.clone()
        } else {
            let host = Target::detect_host()?;
            helpers::info(format!("No target specified, using host: {}", host.triple));
            host.triple
        };

        // Parse target
        let target = Target::from_triple(&target_triple)?;
        helpers::progress(format!(
            "{} for target: {}",
            options.operation.description(),
            target.triple
        ));

        // Check if we should use container build
        let should_use_container =
            options.use_container || self.should_use_container_for_target(&target)?;

        if should_use_container {
            return self.build_with_container(&target, options);
        }

        // Check if Zig can handle this cross-compilation
        let zig_env = self.try_zig_cross_compilation(&target, options)?;
        let using_zig = zig_env.is_some();

        // Determine toolchain
        let toolchain = if let Some(tc) = &options.toolchain {
            tc.clone()
        } else {
            "stable".to_string()
        };

        // Ensure target is installed
        helpers::progress("Checking toolchain and target...".to_string());
        self.toolchain_manager.prepare_target(&toolchain, &target)?;
        helpers::success("Toolchain and target ready");

        // Show tips based on target
        if target.os != Target::detect_host()?.os {
            if using_zig {
                helpers::tip("Cross-compiling using Zig toolchain");
            } else {
                helpers::tip("Cross-compiling to a different OS");
                if self.config.container.use_when == "target.os != host.os" {
                    helpers::hint("Container builds not yet implemented - using native toolchain");
                }
            }
        }

        // Get target-specific configuration
        let target_config = self.config.get_target_config(&target.triple);

        // Check linker configuration and availability (skip if using Zig)
        let linker = if using_zig {
            None // Zig provides its own linker
        } else if let Some(config) = target_config {
            config.linker.clone()
        } else {
            let requirements = target.get_requirements();
            requirements.linker
        };

        // Verify linker exists if specified (and not using Zig)
        if !using_zig {
            if let Some(ref linker_path) = linker {
                if let Ok(path) = which::which(linker_path) {
                    if options.verbose {
                        helpers::info(format!(
                            "Using linker: {} ({})",
                            linker_path,
                            path.display()
                        ));
                    }
                } else {
                    helpers::warning(format!(
                        "Configured linker '{linker_path}' not found in PATH"
                    ));

                    let requirements = target.get_requirements();
                    if !requirements.tools.is_empty() {
                        helpers::hint(format!("Required tools: {}", requirements.tools.join(", ")));
                    }

                    // Suggest platform-specific installation
                    let host = Target::detect_host()?;
                    self.suggest_linker_installation(&host, &target);

                    helpers::tip("The build may fail if the linker is not available");
                }
            } else {
                // No linker configured - check if one is recommended
                let requirements = target.get_requirements();
                if let Some(suggested_linker) = requirements.linker {
                    // Check if the suggested linker is available
                    if which::which(&suggested_linker).is_ok() {
                        if options.verbose {
                            helpers::info(format!("Using default linker: {suggested_linker}"));
                        }
                    } else {
                        helpers::hint(format!("Recommended linker '{suggested_linker}' not found"));

                        let host = Target::detect_host()?;
                        self.suggest_linker_installation(&host, &target);

                        helpers::tip(format!(
                            "Configure in xcargo.toml: [targets.\"{}\"] linker = \"{}\"",
                            target.triple, suggested_linker
                        ));
                    }
                }
            }
        }

        // Build cargo command
        helpers::progress(format!("Running cargo {}...", options.operation.as_str()));
        let mut cmd = Command::new("cargo");

        // Apply Zig environment if using Zig for cross-compilation
        if let Some(ref env) = zig_env {
            for (key, value) in env {
                cmd.env(key, value);
                if options.verbose {
                    helpers::info(format!("Setting {}={}", key, value.display()));
                }
            }
        }

        // Set environment variables for linker and custom env vars (only if not using Zig)
        if !using_zig {
            if let Some(ref linker_path) = linker {
                // Convert target triple to CARGO env var format
                // e.g., x86_64-pc-windows-gnu -> CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER
                let env_var = format!(
                    "CARGO_TARGET_{}_LINKER",
                    target.triple.to_uppercase().replace('-', "_")
                );
                cmd.env(&env_var, linker_path);

                if options.verbose {
                    helpers::info(format!("Setting {env_var}={linker_path}"));
                }
            }
        }

        // Add custom environment variables from target config
        if let Some(config) = target_config {
            for (key, value) in &config.env {
                cmd.env(key, value);
                if options.verbose {
                    helpers::info(format!("Setting {key}={value}"));
                }
            }

            // Add custom rustflags if specified
            if let Some(ref rustflags) = config.rustflags {
                let rustflags_str = rustflags.join(" ");
                cmd.env("RUSTFLAGS", &rustflags_str);
                if options.verbose {
                    helpers::info(format!("Setting RUSTFLAGS={rustflags_str}"));
                }
            }
        }

        // Add toolchain override if specified
        if options.toolchain.is_some() {
            cmd.arg(format!("+{toolchain}"));
        }

        cmd.arg(options.operation.as_str());

        // Add target
        cmd.arg("--target").arg(&target.triple);

        // Add release flag
        if options.release {
            cmd.arg("--release");
        }

        // Add verbose flag
        if options.verbose
            || self
                .config
                .build
                .cargo_flags
                .contains(&"--verbose".to_string())
        {
            cmd.arg("--verbose");
        }

        // Add additional cargo flags from config
        for flag in &self.config.build.cargo_flags {
            if flag != "--verbose" || !options.verbose {
                cmd.arg(flag);
            }
        }

        // Add additional args from options
        for arg in &options.cargo_args {
            cmd.arg(arg);
        }

        if options.verbose {
            helpers::info(format!("Executing: {cmd:?}"));
        }

        // Execute build
        let status = cmd
            .status()
            .map_err(|e| Error::Build(format!("Failed to execute cargo: {e}")))?;

        if status.success() {
            println!(); // Empty line for spacing
            helpers::success(format!(
                "{} completed for {}",
                options.operation.description(),
                target.triple
            ));

            // Show helpful tips (only for build/test, not check)
            if options.operation != CargoOperation::Check {
                if options.release {
                    helpers::tip(format!(
                        "Release build artifacts are in target/{}/release/",
                        target.triple
                    ));
                } else {
                    helpers::tip(format!(
                        "Debug build artifacts are in target/{}/debug/",
                        target.triple
                    ));
                }

                // Additional tips based on target
                if target.os == "windows" && Target::detect_host()?.os != "windows" {
                    helpers::tip(format!(
                        "Test Windows binaries with Wine: wine target/{}/debug/your-app.exe",
                        target.triple
                    ));
                }

                if target.os == "linux" && Target::detect_host()?.os != "linux" {
                    helpers::tip(
                        "Consider using a Linux VM or container to test the binary".to_string(),
                    );
                }
            }

            Ok(())
        } else {
            println!();
            helpers::error(format!(
                "{} failed for target {}",
                options.operation.description(),
                target.triple
            ));

            // Provide helpful error context
            if linker.is_none() {
                let requirements = target.get_requirements();
                if let Some(suggested_linker) = requirements.linker {
                    println!();
                    helpers::hint("This target requires a cross-compilation linker");
                    helpers::tip(format!("Install the linker: {suggested_linker}"));
                    helpers::tip("Then configure it in xcargo.toml:".to_string());
                    println!("\n  [targets.\"{}\"]", target.triple);
                    println!("  linker = \"{suggested_linker}\"");

                    if !requirements.tools.is_empty() {
                        println!();
                        helpers::hint(format!(
                            "Additional required tools: {}",
                            requirements.tools.join(", ")
                        ));
                    }

                    // Provide OS-specific installation instructions
                    let host_os = Target::detect_host()?.os;
                    println!();
                    helpers::section("Installation Instructions");

                    match (host_os.as_str(), target.os.as_str()) {
                        ("macos", "linux") => {
                            helpers::tip(
                                "Install via Homebrew: brew install SomeLinuxCrossCompiler"
                                    .to_string(),
                            );
                            helpers::tip(
                                "Or use a container-based build (coming soon)".to_string(),
                            );
                        }
                        ("macos", "windows") => {
                            helpers::tip(
                                "Install via Homebrew: brew install mingw-w64".to_string(),
                            );
                            helpers::tip("Then set: [targets.\"x86_64-pc-windows-gnu\"] linker = \"x86_64-w64-mingw32-gcc\"".to_string());
                        }
                        ("linux", "windows") => {
                            helpers::tip(
                                "Install via package manager: sudo apt install mingw-w64"
                                    .to_string(),
                            );
                            helpers::tip("Then set: [targets.\"x86_64-pc-windows-gnu\"] linker = \"x86_64-w64-mingw32-gcc\"".to_string());
                        }
                        ("linux", "macos") => {
                            helpers::tip(
                                "macOS cross-compilation from Linux requires osxcross".to_string(),
                            );
                            helpers::tip(
                                "See: https://github.com/tpoechtrager/osxcross".to_string(),
                            );
                        }
                        (_, _) => {
                            helpers::tip(format!(
                                "Cross-compiling from {} to {} may require specific toolchains",
                                host_os, target.os
                            ));
                        }
                    }
                }
            } else if let Some(ref linker_path) = linker {
                if which::which(linker_path).is_err() {
                    println!();
                    helpers::hint(format!(
                        "The configured linker '{linker_path}' is not in your PATH"
                    ));
                    helpers::tip("Install it or update your xcargo.toml configuration".to_string());
                }
            }

            println!();
            helpers::tip("Run with --verbose to see detailed error output".to_string());

            Err(Error::Build(format!(
                "{} failed for target {}",
                options.operation.description(),
                target.triple
            )))
        }
    }

    /// Build for multiple targets (sequential)
    pub fn build_all(&self, targets: &[String], options: &BuildOptions) -> Result<()> {
        helpers::section(format!(
            "xcargo {} (multiple targets)",
            options.operation.as_str()
        ));
        helpers::info(format!(
            "{} for {} targets",
            options.operation.description(),
            targets.len()
        ));

        let mut successes = Vec::new();
        let mut failures = Vec::new();

        for (idx, target) in targets.iter().enumerate() {
            println!("\n[{}/{}] Target: {}", idx + 1, targets.len(), target);
            println!("{}", "─".repeat(50));

            let mut target_options = options.clone();
            target_options.target = Some(target.clone());

            match self.build(&target_options) {
                Ok(()) => successes.push(target.clone()),
                Err(e) => {
                    helpers::error(format!("Failed to build {target}: {e}"));
                    failures.push(target.clone());
                }
            }
        }

        println!("\n");
        helpers::section("Build Summary");
        helpers::success(format!("{} target(s) built successfully", successes.len()));

        if !failures.is_empty() {
            helpers::error(format!("{} target(s) failed", failures.len()));
            for target in &failures {
                helpers::error(format!("  - {target}"));
            }
            return Err(Error::Build("Some targets failed to build".to_string()));
        }

        helpers::tip(tips::PARALLEL_BUILDS);
        Ok(())
    }

    /// Try to use Zig for cross-compilation if available and supported
    ///
    /// Returns Some(env) if Zig can handle this cross-compilation, None otherwise.
    /// Respects the `use_zig` option: None = auto, Some(true) = force, Some(false) = disable
    fn try_zig_cross_compilation(
        &self,
        target: &Target,
        options: &BuildOptions,
    ) -> Result<Option<HashMap<String, PathBuf>>> {
        // Check if Zig is explicitly disabled
        if options.use_zig == Some(false) {
            if options.verbose {
                helpers::info("Zig disabled via --no-zig flag");
            }
            return Ok(None);
        }

        // Check if Zig is explicitly forced
        let force_zig = options.use_zig == Some(true);

        // Determine if we're cross-compiling to a different OS
        let host = Target::detect_host()?;
        let is_cross_os = target.os != host.os;

        // For auto mode, only attempt Zig for cross-compilation (different OS)
        if !force_zig && !is_cross_os {
            return Ok(None);
        }

        // Check if Zig is available and supports this target
        if let Some(ref zig) = self.zig_toolchain {
            if zig.supports_target(target) {
                helpers::info(format!(
                    "Zig {} detected, using for cross-compilation",
                    zig.version()
                ));
                let env = zig.environment_for_target(target)?;
                return Ok(Some(env));
            } else if force_zig {
                return Err(Error::Toolchain(format!(
                    "Zig does not support target '{}'. Supported targets: x86_64-linux-gnu, aarch64-linux-gnu, armv7-linux-gnueabihf",
                    target.triple
                )));
            }
            // Zig available but doesn't support this target - not an error in auto mode
            if options.verbose {
                helpers::info(format!(
                    "Zig doesn't support target '{}', falling back to native toolchain",
                    target.triple
                ));
            }
        } else {
            // Zig not available
            if force_zig {
                return Err(Error::Toolchain(
                    "Zig not found. Install Zig to use --zig flag: brew install zig (macOS) or scoop install zig (Windows)".to_string()
                ));
            } else if is_cross_os && ZigToolchain::supports_target_name(&target.triple) {
                // Graceful degradation: Zig could help but isn't available
                helpers::hint("Zig is not installed but could simplify this cross-compilation");
                let install_hint = match host.os.as_str() {
                    "macos" => "Install with: brew install zig",
                    "linux" => "Install with: snap install zig --classic --beta",
                    "windows" => "Install with: scoop install zig",
                    _ => "Install Zig: https://ziglang.org/download/",
                };
                helpers::tip(format!("{install_hint} (then use --zig flag)"));
            }
        }

        Ok(None)
    }

    /// Suggest platform-specific installation instructions for a linker
    fn suggest_linker_installation(&self, host: &Target, target: &Target) {
        let host_os = host.os.as_str();
        let target_os = target.os.as_str();

        match (host_os, target_os) {
            ("macos", "linux") => {
                helpers::tip(
                    "For Linux cross-compilation on macOS, consider using Zig: brew install zig",
                );
                helpers::tip(format!(
                    "Then build with: xcargo build --target {} --zig",
                    target.triple
                ));
            }
            ("macos", "windows") => {
                helpers::tip("Install MinGW for Windows cross-compilation: brew install mingw-w64");
            }
            ("linux", "windows") => {
                helpers::tip("Install MinGW: sudo apt install mingw-w64 (Debian/Ubuntu)");
                helpers::tip("Or: sudo dnf install mingw64-gcc (Fedora)");
            }
            ("linux", "macos") => {
                helpers::tip("macOS cross-compilation requires osxcross: https://github.com/tpoechtrager/osxcross");
            }
            ("windows", "linux") => {
                helpers::tip(
                    "For Linux cross-compilation on Windows, consider using WSL or containers",
                );
            }
            (_, _) => {
                helpers::tip(format!(
                    "Install cross-compilation tools for {host_os} → {target_os}"
                ));
            }
        }
    }

    /// Determine if a container build should be used for this target
    fn should_use_container_for_target(&self, target: &Target) -> Result<bool> {
        #[cfg(not(feature = "container"))]
        {
            let _ = target; // Suppress unused warning
            return Ok(false);
        }

        #[cfg(feature = "container")]
        {
            let host = Target::detect_host()?;

            // Check config's use_when condition
            match self.config.container.use_when.as_str() {
                "always" => Ok(true),
                "never" => Ok(false),
                "target.os != host.os" => Ok(target.os != host.os),
                _ => Ok(false),
            }
        }
    }

    /// Build using a container
    #[cfg(feature = "container")]
    fn build_with_container(&self, target: &Target, options: &BuildOptions) -> Result<()> {
        use crate::container::{ContainerBuilder, ContainerConfig, RuntimeType};

        helpers::section("xcargo container build");
        helpers::info(format!("Building {} using container", target.triple));

        // Determine runtime type from config
        let runtime_type =
            RuntimeType::from_str(&self.config.container.runtime).unwrap_or(RuntimeType::Auto);

        // Create container builder
        let container_builder = ContainerBuilder::new(runtime_type)
            .map_err(|e| {
                helpers::error(format!("Failed to initialize container runtime: {e}"));
                helpers::hint("Make sure Docker or Podman is installed and running");

                let host_os = Target::detect_host().ok().map(|h| h.os).unwrap_or_default();
                match host_os.as_str() {
                    "macos" => {
                        helpers::tip("Install Docker Desktop: https://www.docker.com/products/docker-desktop");
                        helpers::tip("Or install Podman: brew install podman && podman machine init && podman machine start");
                    }
                    "linux" => {
                        helpers::tip("Install Docker: https://docs.docker.com/engine/install/");
                        helpers::tip("Or install Podman: sudo apt install podman  (or your distro's package manager)");
                    }
                    "windows" => {
                        helpers::tip("Install Docker Desktop: https://www.docker.com/products/docker-desktop");
                        helpers::tip("Or install Podman: https://podman.io/getting-started/installation");
                    }
                    _ => {
                        helpers::tip("Install Docker or Podman to use container builds");
                    }
                }

                e
            })?;

        if !container_builder.is_available() {
            helpers::error(format!(
                "Container runtime '{}' is not available",
                container_builder.runtime_name()
            ));
            helpers::hint("Make sure the container runtime is installed and running");
            return Err(Error::Container(
                "Container runtime not available".to_string(),
            ));
        }

        helpers::success(format!(
            "Using container runtime: {}",
            container_builder.runtime_name()
        ));

        // Select appropriate image
        let image = container_builder
            .select_image(&target.triple)
            .map_err(|e| {
                helpers::error(format!("Failed to select container image: {e}"));

                // Suggest alternatives based on the error
                if target.os == "macos" {
                    helpers::hint("macOS cross-compilation requires osxcross or building on macOS");
                    helpers::tip("Consider using GitHub Actions macOS runners for macOS builds");
                } else if target.triple.starts_with("wasm") {
                    helpers::hint("WebAssembly doesn't require containers - use native build");
                    helpers::tip("Run without --container flag");
                } else {
                    helpers::hint("This target may not have a pre-built container image");
                    helpers::tip("You can specify a custom image in xcargo.toml");
                }

                e
            })?;

        helpers::info(format!("Using image: {}", image.full_name()));

        // Build container config
        let mut container_config = ContainerConfig::default();
        container_config.runtime = runtime_type;
        container_config.image = image.full_name();

        // Add custom environment variables from target config
        if let Some(target_config) = self.config.get_target_config(&target.triple) {
            for (key, value) in &target_config.env {
                container_config.env.push((key.clone(), value.clone()));
            }
        }

        // Execute container build
        helpers::progress("Pulling container image...");

        let mut cargo_args = options.cargo_args.clone();
        if options.release {
            cargo_args.insert(0, "--release".to_string());
        }
        if options.verbose {
            cargo_args.insert(0, "--verbose".to_string());
        }

        container_builder.build(&target.triple, &cargo_args, &container_config)?;

        println!(); // Empty line for spacing
        helpers::success(format!("Container build completed for {}", target.triple));

        // Show helpful tips
        if options.release {
            helpers::tip(format!(
                "Release build artifacts are in target/{}/release/",
                target.triple
            ));
        } else {
            helpers::tip(format!(
                "Debug build artifacts are in target/{}/debug/",
                target.triple
            ));
        }

        Ok(())
    }

    /// Build using a container (fallback when feature not enabled)
    #[cfg(not(feature = "container"))]
    fn build_with_container(&self, _target: &Target, _options: &BuildOptions) -> Result<()> {
        helpers::error("Container support not enabled");
        helpers::hint("Rebuild xcargo with: cargo install xcargo --features container");
        helpers::tip("Or use native build without --container flag");
        Err(Error::Container(
            "Container feature not enabled".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_new() {
        // This test will succeed if rustup is installed
        let builder = Builder::new();
        if builder.is_err() {
            // Skip test if rustup is not available
            return;
        }
        assert!(builder.is_ok());
    }
}
