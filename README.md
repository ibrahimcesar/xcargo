<div align="center">

# xcargo 🎯

_Cross-compilation, zero friction_

**xcargo** is a Rust cross-compilation tool that just works. Automatic toolchain management, beautiful output, and zero-configuration cross-compilation.

[![Crates.io](https://img.shields.io/crates/v/xcargo.svg)](https://crates.io/crates/xcargo)
[![Documentation](https://docs.rs/xcargo/badge.svg)](https://docs.rs/xcargo)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[Installation](#-installation) | [Quick Start](#-quick-start) | [Documentation](https://ibrahimcesar.github.io/xcargo) | [Examples](#-usage-examples)

</div>

## ✨ Features

- 🎯 **Zero Configuration** - Works out of the box for most targets
- 🔧 **Auto-Installation** - Automatically installs missing toolchains and targets
- 🎨 **Beautiful Output** - Colored messages with helpful tips and hints
- ⚡ **Smart Detection** - Figures out what you need automatically
- 📦 **Interactive Setup** - TUI wizard for easy project configuration
- 🚀 **Parallel Builds** - Build multiple targets concurrently for 2-3x speedup
- 🌍 **Many Targets** - Linux, Windows, macOS, WebAssembly, and more
- 🤖 **CI/CD Ready** - Perfect for GitHub Actions, GitLab CI

## 🚀 Quick Start

### Installation

```bash
# Install from crates.io (recommended)
cargo install xcargo

# Or build from source
git clone https://github.com/ibrahimcesar/xcargo
cd xcargo
cargo build --release
```

### Interactive Setup

The easiest way to get started is with the interactive setup wizard:

```bash
xcargo init --interactive
```

This will guide you through:
- ✨ Selecting target platforms
- ⚙️ Configuring parallel builds
- 🔧 Setting up caching
- 🐳 Choosing container strategy
- 📦 Installing targets automatically

### First Build

```bash
# Build for your current platform
xcargo build

# Build for a specific target
xcargo build --target x86_64-pc-windows-gnu

# Build for all configured targets
xcargo build --all

# Release build
xcargo build --target x86_64-unknown-linux-gnu --release
```

## 💡 Usage Examples

### Basic Cross-Compilation

```bash
# Build for Windows from any platform
xcargo build --target x86_64-pc-windows-gnu

# Build for Linux ARM
xcargo build --target aarch64-unknown-linux-gnu

# Build for macOS (M1/M2)
xcargo build --target aarch64-apple-darwin

# Build for WebAssembly
xcargo build --target wasm32-unknown-unknown
```

### Target Management

```bash
# List common cross-compilation targets
xcargo target list

# Show installed targets
xcargo target list --installed

# Get detailed info about a target
xcargo target info x86_64-pc-windows-gnu

# Add a new target
xcargo target add x86_64-unknown-linux-musl
```

### Configuration

```bash
# Show current configuration
xcargo config

# Show default configuration template
xcargo config --default

# Initialize with defaults
xcargo init

# Interactive setup wizard
xcargo init --interactive
```

## ⚙️ Configuration File

Create an `xcargo.toml` in your project root:

```toml
[targets]
# Default targets to build when no target is specified
default = [
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-gnu",
]

# Per-target custom configuration
[targets."x86_64-pc-windows-gnu"]
linker = "x86_64-w64-mingw32-gcc"

[targets."x86_64-pc-windows-gnu".env]
CC = "x86_64-w64-mingw32-gcc"

[build]
# Enable parallel builds for multiple targets (2-3x faster!)
parallel = true

# Enable build caching
cache = true

# Force container usage (not yet implemented)
force_container = false

# Additional cargo flags to pass to all builds
cargo_flags = []

[container]
# Container runtime: auto, youki, docker, podman
runtime = "auto"

# When to use containers
use_when = "target.os != host.os"

# Image pull policy
pull_policy = "if-not-present"

# Build profiles for different scenarios
[profiles.release-all]
targets = [
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-gnu",
    "x86_64-apple-darwin",
    "aarch64-unknown-linux-gnu",
    "aarch64-apple-darwin",
]
```

## 🎯 Supported Targets

xcargo supports all Rust targets. Common ones include:

**Linux**
- `x86_64-unknown-linux-gnu` - Linux x86_64
- `x86_64-unknown-linux-musl` - Linux x86_64 (static)
- `aarch64-unknown-linux-gnu` - Linux ARM64

**Windows**
- `x86_64-pc-windows-gnu` - Windows x86_64 (MinGW)
- `x86_64-pc-windows-msvc` - Windows x86_64 (MSVC)

**macOS**
- `x86_64-apple-darwin` - macOS x86_64
- `aarch64-apple-darwin` - macOS ARM64 (M1/M2)

**WebAssembly**
- `wasm32-unknown-unknown` - WebAssembly

Run `xcargo target list` to see all common targets with descriptions.

## 🔧 How It Works

1. **Target Detection** - Analyzes the target triple and determines requirements
2. **Toolchain Check** - Verifies the Rust toolchain and target are installed
3. **Auto-Installation** - Installs missing components via rustup
4. **Smart Building** - Uses native builds when possible, suggests containers when needed
5. **Helpful Output** - Shows tips, hints, and next steps

```
┌──────────────────────────────┐
│ xcargo build --target linux  │
└──────────┬───────────────────┘
           │
           ▼
   ┌───────────────┐
   │ Detect target │
   │ requirements  │
   └───────┬───────┘
           │
           ▼
   ┌────────────────┐
   │ Check toolchain│
   │ & install if   │
   │ missing        │
   └───────┬────────┘
           │
           ▼
   ┌────────────────┐
   │ Execute cargo  │
   │ build with     │
   │ proper flags   │
   └────────────────┘
```

## 🤖 CI/CD Integration

### GitHub Actions

```yaml
name: Cross-Platform Build

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install xcargo
        run: cargo install xcargo

      - name: Build for all targets
        run: xcargo build --all
```

### GitLab CI

```yaml
build:
  image: rust:latest
  script:
    - cargo install xcargo
    - xcargo build --all
  artifacts:
    paths:
      - target/*/release/*
```

## 🎨 Beautiful Output

xcargo provides helpful, colored output with tips and hints:

```
✨ xcargo Interactive Setup
Let's configure cross-compilation for your project!

✓ Detected host platform: aarch64-apple-darwin

? Which targets do you want to build for?
  ↑↓ to navigate, Space to select, Enter to confirm
  [ ] Linux x86_64
  [✓] Windows x86_64 (GNU)
  [✓] macOS ARM64 (M1/M2)

✓ Configuration created successfully!

📋 Configuration Summary
────────────────────────
Targets: x86_64-pc-windows-gnu, aarch64-apple-darwin
Parallel builds: enabled
Build cache: enabled
Container strategy: target.os != host.os

💡 Tip: Run 'xcargo build' to build for your host platform
💡 Tip: Run 'xcargo build --all' to build for all configured targets
```

## 📊 Status

**Current Version:** 0.1.0

✅ **Working Features:**
- Target detection and validation
- Toolchain management via rustup
- Basic cross-compilation
- Configuration system (xcargo.toml)
- Interactive TUI setup wizard
- Beautiful colored output with tips
- Self-building capability (xcargo builds itself!)
- **Parallel target compilation** (2-3x speedup with `parallel = true`)
- GitHub Actions CI/CD integration

🚧 **Planned Features:**
- Container builds (Docker/Podman/youki)
- Native dependency management
- Custom linker configuration
- Build caching improvements

## 🆚 Comparison

| Feature | xcargo | cross | cargo-zigbuild |
|---------|--------|-------|----------------|
| **Native-first** | ✅ | ❌ | ⚠️ Via Zig |
| **Auto-install targets** | ✅ | ❌ | ❌ |
| **Interactive setup** | ✅ | ❌ | ❌ |
| **Parallel builds** | ✅ | ❌ | ❌ |
| **Beautiful output** | ✅ | ⚠️ | ⚠️ |
| **Configuration file** | ✅ | ✅ | ❌ |
| **Container fallback** | 🚧 Planned | ✅ | ❌ |
| **Zero config** | ✅ | ❌ | ⚠️ |

## 🤝 Contributing

Contributions are welcome! This is an early-stage project with lots of opportunity to help.

**Ways to contribute:**
- 🐛 Report bugs and suggest features via [GitHub Issues](https://github.com/ibrahimcesar/xcargo/issues)
- 💻 Submit pull requests for fixes or new features
- 📝 Improve documentation
- 🎯 Test on different platforms and targets
- ⭐ Star the repo to show support!

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

## 📚 Documentation

- [Full Documentation](https://ibrahimcesar.github.io/xcargo)
- [API Documentation](https://docs.rs/xcargo)
- [Configuration Reference](https://ibrahimcesar.github.io/xcargo/docs/reference/configuration)
- [Target Guide](https://ibrahimcesar.github.io/xcargo/docs/guides/target-management)

## 📝 License

[MIT](./LICENSE) © Ibrahim Cesar

## 🙏 Acknowledgments

Inspired by excellent tools in the Rust ecosystem:
- [cross](https://github.com/cross-rs/cross) - Container-based cross-compilation
- [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) - Zig linker approach
- [rustup](https://rustup.rs/) - Rust toolchain management

---

<div align="center">

**xcargo** - *Cross-compilation, zero friction* 🎯

Made with ❤️ by [Ibrahim Cesar](https://github.com/ibrahimcesar)

[⭐ Star on GitHub](https://github.com/ibrahimcesar/xcargo) | [📦 View on crates.io](https://crates.io/crates/xcargo) | [📖 Read the Docs](https://ibrahimcesar.github.io/xcargo)

</div>
