//! xcargo CLI entry point

use clap::{Parser, Subcommand};
use inquire::{Confirm, InquireError, MultiSelect, Select};
use std::path::Path;
use xcargo::build::{BuildOptions, Builder, CargoOperation};
use xcargo::config::Config;
use xcargo::error::Error;
use xcargo::output::{helpers, tips};
use xcargo::target::Target;
use xcargo::toolchain::ToolchainManager;

/// Result type for main using xcargo's error type
type Result<T> = std::result::Result<T, Error>;

/// Convert InquireError to our Error type
fn prompt_err(e: InquireError) -> Error {
    Error::Prompt(e.to_string())
}

/// Print error with suggestion and hint, then exit with proper code
fn exit_with_error(error: &Error) -> ! {
    helpers::error(format!("{}", error));

    if let Some(hint) = error.hint() {
        helpers::hint(hint);
    }

    if let Some(suggestion) = error.suggestion() {
        helpers::tip(suggestion);
    }

    std::process::exit(error.exit_code())
}

/// xcargo - Cross-compilation, zero friction 🎯
#[derive(Parser)]
#[command(name = "xcargo")]
#[command(author, version, about, long_about = None)]
#[command(after_help = "TIP: Run 'xcargo build --target <triple>' to cross-compile")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Build for target platform(s)
    Build {
        /// Target triple (e.g., x86_64-pc-windows-gnu)
        #[arg(short, long)]
        target: Option<String>,

        /// Build for all configured targets
        #[arg(long, conflicts_with = "target")]
        all: bool,

        /// Build in release mode
        #[arg(short, long)]
        release: bool,

        /// Use container for build (requires --features container)
        #[arg(long)]
        container: bool,

        /// Force using Zig for cross-compilation
        #[arg(long, conflicts_with = "no_zig")]
        zig: bool,

        /// Disable Zig cross-compilation (use native toolchain or container)
        #[arg(long, conflicts_with = "zig")]
        no_zig: bool,

        /// Toolchain to use (e.g., stable, nightly)
        #[arg(long)]
        toolchain: Option<String>,

        /// Additional cargo arguments
        #[arg(last = true)]
        cargo_args: Vec<String>,
    },

    /// Manage targets
    Target {
        #[command(subcommand)]
        action: TargetAction,
    },

    /// Initialize xcargo for a project
    Init {
        /// Interactive setup wizard
        #[arg(short, long)]
        interactive: bool,
    },

    /// Display configuration
    Config {
        /// Show default config
        #[arg(long)]
        default: bool,
    },

    /// Check target(s) for errors without building
    Check {
        /// Target triple (e.g., x86_64-pc-windows-gnu)
        #[arg(short, long)]
        target: Option<String>,

        /// Check all configured targets
        #[arg(long, conflicts_with = "target")]
        all: bool,

        /// Force using Zig for cross-compilation
        #[arg(long, conflicts_with = "no_zig")]
        zig: bool,

        /// Disable Zig cross-compilation
        #[arg(long, conflicts_with = "zig")]
        no_zig: bool,

        /// Toolchain to use (e.g., stable, nightly)
        #[arg(long)]
        toolchain: Option<String>,

        /// Additional cargo arguments
        #[arg(last = true)]
        cargo_args: Vec<String>,
    },

    /// Run tests for target(s)
    Test {
        /// Target triple (e.g., x86_64-pc-windows-gnu)
        #[arg(short, long)]
        target: Option<String>,

        /// Test all configured targets
        #[arg(long, conflicts_with = "target")]
        all: bool,

        /// Release mode
        #[arg(short, long)]
        release: bool,

        /// Force using Zig for cross-compilation
        #[arg(long, conflicts_with = "no_zig")]
        zig: bool,

        /// Disable Zig cross-compilation
        #[arg(long, conflicts_with = "zig")]
        no_zig: bool,

        /// Toolchain to use (e.g., stable, nightly)
        #[arg(long)]
        toolchain: Option<String>,

        /// Additional cargo arguments
        #[arg(last = true)]
        cargo_args: Vec<String>,
    },

    /// Show version information
    Version,
}

#[derive(Subcommand)]
enum TargetAction {
    /// Add a target
    Add {
        /// Target name or triple
        target: String,

        /// Toolchain to add target to
        #[arg(long, default_value = "stable")]
        toolchain: String,
    },

    /// List targets
    List {
        /// Show only installed targets
        #[arg(long)]
        installed: bool,

        /// Toolchain to list targets for
        #[arg(long)]
        toolchain: Option<String>,
    },

    /// Show target information
    Info {
        /// Target triple
        target: String,
    },
}

/// Run basic non-interactive setup
fn run_basic_setup() -> Result<()> {
    helpers::section("Initialize xcargo");

    if Path::new("xcargo.toml").exists() {
        helpers::warning("xcargo.toml already exists");
        let overwrite = Confirm::new("Overwrite existing configuration?")
            .with_default(false)
            .prompt()
            .map_err(prompt_err)?;

        if !overwrite {
            helpers::info("Setup cancelled");
            return Ok(());
        }
    }

    let host = Target::detect_host()?;
    let mut config = Config::default();
    config.targets.default = vec![host.triple.clone()];

    config.save("xcargo.toml")?;

    helpers::success("Created xcargo.toml with default configuration");
    helpers::tip(format!("Default target: {}", host.triple));
    helpers::hint("Use 'xcargo init --interactive' for guided setup");

    Ok(())
}

/// Run interactive TUI setup wizard
fn run_interactive_setup() -> Result<()> {
    use xcargo::output::colors;

    println!(
        "\n{}{}✨ xcargo Interactive Setup{}",
        colors::BOLD,
        colors::CYAN,
        colors::RESET
    );
    println!(
        "{}Let's configure cross-compilation for your project!{}\n",
        colors::DIM,
        colors::RESET
    );

    // Check for existing config
    if Path::new("xcargo.toml").exists() {
        helpers::warning("xcargo.toml already exists");
        let overwrite = Confirm::new("Overwrite existing configuration?")
            .with_default(false)
            .prompt()
            .map_err(prompt_err)?;

        if !overwrite {
            helpers::info("Setup cancelled");
            return Ok(());
        }
    }

    // Detect host
    let host = Target::detect_host()?;
    helpers::success(format!("Detected host platform: {}", host.triple));
    println!();

    // Select target platforms
    let target_options = [
        ("Linux x86_64", "x86_64-unknown-linux-gnu"),
        ("Linux x86_64 (musl)", "x86_64-unknown-linux-musl"),
        ("Linux ARM64", "aarch64-unknown-linux-gnu"),
        ("Windows x86_64 (GNU)", "x86_64-pc-windows-gnu"),
        ("Windows x86_64 (MSVC)", "x86_64-pc-windows-msvc"),
        ("macOS x86_64", "x86_64-apple-darwin"),
        ("macOS ARM64 (M1/M2)", "aarch64-apple-darwin"),
        ("WebAssembly", "wasm32-unknown-unknown"),
    ];

    let selected_names = MultiSelect::new(
        "Which targets do you want to build for?",
        target_options.iter().map(|(name, _)| *name).collect(),
    )
    .with_help_message("Use ↑↓ to navigate, Space to select, Enter to confirm")
    .prompt()
    .map_err(prompt_err)?;

    let selected_targets: Vec<String> = selected_names
        .iter()
        .filter_map(|&selected_name| {
            target_options
                .iter()
                .find(|(name, _)| name == &selected_name)
                .map(|(_, triple)| triple.to_string())
        })
        .collect();

    if selected_targets.is_empty() {
        helpers::warning("No targets selected, using host target");
    }

    println!();

    // Parallel builds
    let parallel = Confirm::new("Enable parallel builds?")
        .with_default(true)
        .with_help_message("Build multiple targets concurrently for faster builds")
        .prompt()
        .map_err(prompt_err)?;

    // Build caching
    let cache = Confirm::new("Enable build caching?")
        .with_default(true)
        .with_help_message("Cache build artifacts to speed up subsequent builds")
        .prompt()
        .map_err(prompt_err)?;

    // Container strategy
    let container_options = vec![
        "Auto (use containers only when necessary)",
        "Always use containers",
        "Never use containers",
    ];

    let container_choice = Select::new("Container build strategy:", container_options)
        .with_help_message("Containers ensure reproducible builds")
        .prompt()
        .map_err(prompt_err)?;

    let use_when = match container_choice {
        "Auto (use containers only when necessary)" => "target.os != host.os",
        "Always use containers" => "always",
        "Never use containers" => "never",
        _ => "target.os != host.os",
    };

    println!();
    helpers::progress("Creating configuration...");

    // Build configuration
    let mut config = Config::default();
    let host_triple = host.triple.clone();
    config.targets.default = if selected_targets.is_empty() {
        vec![host_triple.clone()]
    } else {
        selected_targets.clone()
    };
    config.build.parallel = parallel;
    config.build.cache = cache;
    config.container.use_when = use_when.to_string();

    // Save configuration
    config.save("xcargo.toml")?;

    println!();
    helpers::success("✨ Configuration created successfully!");
    println!();

    // Summary
    helpers::section("Configuration Summary");
    println!("Targets: {}", selected_targets.join(", "));
    println!(
        "Parallel builds: {}",
        if parallel { "enabled" } else { "disabled" }
    );
    println!(
        "Build cache: {}",
        if cache { "enabled" } else { "disabled" }
    );
    println!("Container strategy: {}", use_when);
    println!();

    // Next steps
    helpers::section("Next Steps");
    helpers::tip("Run 'xcargo build' to build for your host platform");
    helpers::tip("Run 'xcargo build --all' to build for all configured targets");
    helpers::tip("Run 'xcargo target add <triple>' to add more targets");
    println!();

    // Offer to install targets
    let install_now = Confirm::new("Install selected targets now?")
        .with_default(true)
        .prompt()
        .map_err(prompt_err)?;

    if install_now && !selected_targets.is_empty() {
        println!();
        helpers::progress("Installing targets...");
        let manager = ToolchainManager::new()?;

        for target in &selected_targets {
            if target != &host_triple {
                match manager.ensure_target("stable", target) {
                    Ok(()) => helpers::success(format!("Installed {}", target)),
                    Err(e) => helpers::warning(format!("Failed to install {}: {}", target, e)),
                }
            }
        }

        println!();
        helpers::success("Setup complete! You're ready to cross-compile 🚀");
    } else {
        helpers::success("Setup complete! Install targets later with 'xcargo target add <triple>'");
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        exit_with_error(&e);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build {
            target,
            all,
            release,
            container,
            zig,
            no_zig,
            toolchain,
            cargo_args,
        } => {
            let builder = Builder::new()?;

            // Determine Zig preference: None = auto, Some(true) = force, Some(false) = disable
            let use_zig = if zig {
                Some(true)
            } else if no_zig {
                Some(false)
            } else {
                None
            };

            let options = BuildOptions {
                target: target.clone(),
                release,
                cargo_args,
                toolchain,
                verbose: cli.verbose,
                use_container: container,
                use_zig,
                operation: CargoOperation::Build,
            };

            if all {
                // Build for all configured targets
                let config = Config::discover()?.map(|(c, _)| c).unwrap_or_default();

                if config.targets.default.is_empty() {
                    helpers::error("No default targets configured");
                    helpers::hint("Add targets to xcargo.toml: [targets] default = [\"x86_64-unknown-linux-gnu\"]");
                    helpers::tip(tips::CONFIG_FILE);
                    std::process::exit(1);
                }

                // Use parallel builds if enabled in config
                if config.build.parallel {
                    let rt = tokio::runtime::Runtime::new()?;
                    rt.block_on(builder.build_all_parallel(&config.targets.default, &options))?;
                } else {
                    builder.build_all(&config.targets.default, &options)?;
                }
            } else {
                builder.build(&options)?;
            }
        }

        Commands::Check {
            target,
            all,
            zig,
            no_zig,
            toolchain,
            cargo_args,
        } => {
            let builder = Builder::new()?;

            let use_zig = if zig {
                Some(true)
            } else if no_zig {
                Some(false)
            } else {
                None
            };

            let options = BuildOptions {
                target: target.clone(),
                release: false,
                cargo_args,
                toolchain,
                verbose: cli.verbose,
                use_container: false,
                use_zig,
                operation: CargoOperation::Check,
            };

            if all {
                let config = Config::discover()?.map(|(c, _)| c).unwrap_or_default();

                if config.targets.default.is_empty() {
                    helpers::error("No default targets configured");
                    helpers::hint("Add targets to xcargo.toml: [targets] default = [\"x86_64-unknown-linux-gnu\"]");
                    std::process::exit(1);
                }

                if config.build.parallel {
                    let rt = tokio::runtime::Runtime::new()?;
                    rt.block_on(builder.build_all_parallel(&config.targets.default, &options))?;
                } else {
                    builder.build_all(&config.targets.default, &options)?;
                }
            } else {
                builder.build(&options)?;
            }
        }

        Commands::Test {
            target,
            all,
            release,
            zig,
            no_zig,
            toolchain,
            cargo_args,
        } => {
            let builder = Builder::new()?;

            let use_zig = if zig {
                Some(true)
            } else if no_zig {
                Some(false)
            } else {
                None
            };

            let options = BuildOptions {
                target: target.clone(),
                release,
                cargo_args,
                toolchain,
                verbose: cli.verbose,
                use_container: false,
                use_zig,
                operation: CargoOperation::Test,
            };

            if all {
                let config = Config::discover()?.map(|(c, _)| c).unwrap_or_default();

                if config.targets.default.is_empty() {
                    helpers::error("No default targets configured");
                    helpers::hint("Add targets to xcargo.toml: [targets] default = [\"x86_64-unknown-linux-gnu\"]");
                    std::process::exit(1);
                }

                if config.build.parallel {
                    let rt = tokio::runtime::Runtime::new()?;
                    rt.block_on(builder.build_all_parallel(&config.targets.default, &options))?;
                } else {
                    builder.build_all(&config.targets.default, &options)?;
                }
            } else {
                builder.build(&options)?;
            }
        }

        Commands::Target { action } => match action {
            TargetAction::Add { target, toolchain } => {
                helpers::section("Add Target");

                let manager = ToolchainManager::new()?;
                let target_triple = Target::resolve_alias(&target)?;

                helpers::progress(format!(
                    "Adding target {} to toolchain {}...",
                    target_triple, toolchain
                ));

                manager.install_target(&toolchain, &target_triple)?;

                helpers::success(format!("Target {} added successfully", target_triple));
                helpers::tip(format!(
                    "Use 'xcargo build --target {}' to build for this target",
                    target_triple
                ));
            }

            TargetAction::List {
                installed,
                toolchain,
            } => {
                helpers::section("Available Targets");

                if installed {
                    let manager = ToolchainManager::new()?;
                    let tc = toolchain.unwrap_or_else(|| "stable".to_string());

                    helpers::info(format!("Installed targets for toolchain '{}':", tc));
                    println!();

                    match manager.list_targets(&tc) {
                        Ok(targets) => {
                            if targets.is_empty() {
                                println!("  No targets installed");
                            } else {
                                for target in targets {
                                    println!("  • {}", target);
                                }
                            }
                        }
                        Err(e) => {
                            helpers::error(format!("Failed to list targets: {}", e));
                            std::process::exit(1);
                        }
                    }
                } else {
                    println!("Common cross-compilation targets:\n");

                    println!("Linux:");
                    println!("  • x86_64-unknown-linux-gnu   (Linux x86_64)");
                    println!("  • x86_64-unknown-linux-musl  (Linux x86_64, statically linked)");
                    println!("  • aarch64-unknown-linux-gnu  (Linux ARM64)");
                    println!();

                    println!("Windows:");
                    println!("  • x86_64-pc-windows-gnu      (Windows x86_64, MinGW)");
                    println!("  • x86_64-pc-windows-msvc     (Windows x86_64, MSVC)");
                    println!();

                    println!("macOS:");
                    println!("  • x86_64-apple-darwin        (macOS x86_64)");
                    println!("  • aarch64-apple-darwin       (macOS ARM64, M1/M2)");
                    println!();

                    helpers::hint("Use 'xcargo target list --installed' to see installed targets");
                    helpers::tip("Use 'xcargo target add <triple>' to install a new target");
                }
            }

            TargetAction::Info { target } => {
                helpers::section("Target Information");

                let target_triple = Target::resolve_alias(&target)?;
                match Target::from_triple(&target_triple) {
                    Ok(target) => {
                        println!("Triple:       {}", target.triple);
                        println!("Architecture: {}", target.arch);
                        println!("OS:           {}", target.os);
                        println!(
                            "Environment:  {}",
                            target.env.as_deref().unwrap_or("default")
                        );
                        println!("Tier:         {:?}", target.tier);
                        println!();

                        let requirements = target.get_requirements();
                        if requirements.linker.is_some()
                            || !requirements.tools.is_empty()
                            || !requirements.system_libs.is_empty()
                        {
                            helpers::info("Requirements:");
                            if let Some(linker) = requirements.linker {
                                println!("  Linker: {}", linker);
                            }
                            if !requirements.tools.is_empty() {
                                println!("  Tools: {}", requirements.tools.join(", "));
                            }
                            if !requirements.system_libs.is_empty() {
                                println!("  System libs: {}", requirements.system_libs.join(", "));
                            }
                            println!();
                        }

                        let host = Target::detect_host()?;
                        if target.can_cross_compile_from(&host) {
                            helpers::success("Can cross-compile from this host");
                        } else {
                            helpers::warning("May require container for cross-compilation");
                        }

                        println!();
                        helpers::tip(format!(
                            "Add this target: xcargo target add {}",
                            target.triple
                        ));
                        helpers::tip(format!(
                            "Build for this target: xcargo build --target {}",
                            target.triple
                        ));
                    }
                    Err(e) => {
                        helpers::error(format!("Invalid target: {}", e));
                        std::process::exit(1);
                    }
                }
            }
        },

        Commands::Init { interactive } => {
            if interactive {
                run_interactive_setup()?;
            } else {
                run_basic_setup()?;
            }
        }

        Commands::Config { default } => {
            helpers::section("Configuration");

            if default {
                let config = Config::default();
                match config.to_toml() {
                    Ok(toml) => {
                        println!("{}", toml);
                        println!();
                        helpers::tip("Save this to xcargo.toml to customize your build");
                    }
                    Err(e) => {
                        helpers::error(format!("Failed to generate config: {}", e));
                        std::process::exit(1);
                    }
                }
            } else {
                match Config::discover() {
                    Ok(Some((config, path))) => {
                        helpers::info(format!("Configuration from: {}", path.display()));
                        println!();
                        match config.to_toml() {
                            Ok(toml) => println!("{}", toml),
                            Err(e) => {
                                helpers::error(format!("Failed to serialize config: {}", e));
                                std::process::exit(1);
                            }
                        }
                    }
                    Ok(None) => {
                        helpers::info("No xcargo.toml found, using defaults");
                        println!();
                        let config = Config::default();
                        match config.to_toml() {
                            Ok(toml) => println!("{}", toml),
                            Err(e) => {
                                helpers::error(format!("Failed to generate config: {}", e));
                                std::process::exit(1);
                            }
                        }
                        println!();
                        helpers::tip(tips::CONFIG_FILE);
                    }
                    Err(e) => {
                        helpers::error(format!("Failed to load config: {}", e));
                        std::process::exit(1);
                    }
                }
            }
        }

        Commands::Version => {
            println!("xcargo {}", env!("CARGO_PKG_VERSION"));
            println!("Cross-compilation, zero friction 🎯");
            println!();
            println!("https://github.com/ibrahimcesar/xcargo");
        }
    }

    Ok(())
}
