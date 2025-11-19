# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-01-18

### Added

- **Parallel Builds** - Build multiple targets concurrently for 2-3x speedup
  - Async runtime using tokio for true parallel execution
  - Automatic parallel builds when `build.parallel = true` in config
  - CPU usage shows true multi-core utilization (279%+ on multi-target builds)
  - New `build_all_parallel()` method with Arc/Mutex for thread-safe state

- **Linker Configuration Support** - Critical for successful cross-compilation
  - Per-target linker configuration in xcargo.toml
  - Automatic `CARGO_TARGET_*_LINKER` environment variable setup
  - Custom environment variables per target (CC, AR, etc.)
  - Custom RUSTFLAGS support per target
  - Smart linker detection with PATH verification
  - Warnings when configured linkers are missing

- **Enhanced Error Messages** - Platform-specific help for cross-compilation
  - Detects linker failures and provides context
  - OS-specific installation instructions (macOS, Linux, Windows)
  - Shows exact xcargo.toml configuration needed
  - Helpful tips for common cross-compilation scenarios
  - Suggests `--verbose` for detailed debugging

- **GitHub Actions CI/CD** - Comprehensive testing and multi-platform builds
  - Test job on Linux, macOS, Windows with stable & beta Rust
  - Formatting checks with rustfmt
  - Linting with clippy (warnings as errors)
  - Multi-platform builds for 6 targets
  - Code coverage with tarpaulin and Codecov integration
  - Artifact uploads for all built binaries
  - Dependency caching for faster builds

- **Verbose Mode Enhancements**
  - Shows all environment variables being set
  - Displays CARGO_TARGET_*_LINKER configuration
  - Shows custom env vars and RUSTFLAGS
  - Helps debug cross-compilation issues

- **Better Success Messages**
  - Platform-specific testing tips (Wine for Windows, VMs for Linux)
  - Clear artifact location information
  - Next-step suggestions after successful builds

### Changed

- Improved target requirements detection
- Enhanced output formatting and colors
- Better error context throughout the codebase

### Fixed

- Linker detection for cross-compilation targets
- Environment variable handling in builds
- Parallel build race conditions with proper Arc/Mutex usage

### Documentation

- Added comprehensive linker configuration section in README
- Windows cross-compilation setup guide
- Linux cross-compilation notes
- Installation instructions for mingw-w64
- Updated comparison table with new features
- Moved parallel builds and linker config from planned to working features

## [0.1.0] - 2025-01-17

### Added

- Initial release of xcargo
- Target detection and validation
- Toolchain management via rustup
- Basic cross-compilation support
- Configuration system (xcargo.toml)
- Interactive TUI setup wizard
- Beautiful colored output with tips and hints
- Target aliases (linux, windows, macos)
- Self-building capability
- MIT License

### Features

- Zero-configuration for most targets
- Automatic toolchain and target installation
- Smart detection of requirements
- Configuration file discovery in parent directories
- Per-target custom configuration
- Build profiles for different scenarios
- Container runtime configuration (planned)

[0.2.0]: https://github.com/ibrahimcesar/xcargo/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ibrahimcesar/xcargo/releases/tag/v0.1.0
