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
- 🦎 **Zig Integration** - Auto-detect Zig for zero-config cross-compilation (macOS/Windows → Linux)
- 🐳 **Container Support** - Docker/Podman integration for complex cross-compilation
- 🌍 **Many Targets** - Linux, Windows, macOS, WebAssembly, and more
- 🤖 **CI/CD Ready** - Perfect for GitHub Actions, GitLab CI

## 📦 Installation

### Quick Install (Recommended)

#### macOS / Linux

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-installer.sh | sh
```

#### Windows (PowerShell)

```powershell
irm https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-installer.ps1 | iex
```

### Package Managers

#### Homebrew (macOS / Linux)

```bash
brew install ibrahimcesar/tap/xcargo
```

#### Cargo (from source)

```bash
# Coming soon: cargo install xcargo

# Install from GitHub
cargo install --git https://github.com/ibrahimcesar/xcargo
```

### Prebuilt Binaries

Download prebuilt binaries from the [latest release](https://github.com/ibrahimcesar/xcargo/releases/latest):

- **macOS (Apple Silicon)**: [xcargo-aarch64-apple-darwin.tar.xz](https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-aarch64-apple-darwin.tar.xz)
- **macOS (Intel)**: [xcargo-x86_64-apple-darwin.tar.xz](https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-x86_64-apple-darwin.tar.xz)
- **Linux (glibc)**: [xcargo-x86_64-unknown-linux-gnu.tar.xz](https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-x86_64-unknown-linux-gnu.tar.xz)
- **Linux (musl)**: [xcargo-x86_64-unknown-linux-musl.tar.xz](https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-x86_64-unknown-linux-musl.tar.xz)
- **Windows (MSVC)**: [xcargo-x86_64-pc-windows-msvc.zip](https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-x86_64-pc-windows-msvc.zip)

All downloads include SHA256 checksums for verification.

See the full [Installation Guide](https://ibrahimcesar.github.io/xcargo/installation) for more options.

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

# Use container for build (requires --features container)
xcargo build --target x86_64-unknown-linux-gnu --container
```

### Zero-Config Cross-Compilation with Zig

xcargo automatically detects [Zig](https://ziglang.org/) and uses it for cross-compilation when building for a different OS. No configuration needed!

```bash
# Install Zig (optional - enables native cross-compilation)
# macOS
brew install zig

# Windows
scoop install zig
# or: choco install zig

# Linux
# Download from https://ziglang.org/download/
```

**With Zig installed, cross-compilation just works:**

```bash
# On macOS or Windows, build for Linux - no Docker needed!
xcargo build --target x86_64-unknown-linux-gnu

# Output:
# ℹ Zig 0.15.2 detected, using for cross-compilation
# 💡 Cross-compiling using Zig toolchain
# ✓ Build completed for x86_64-unknown-linux-gnu
```

**Supported targets with Zig:**
- ✅ `x86_64-unknown-linux-gnu`
- ✅ `aarch64-unknown-linux-gnu`
- ✅ `armv7-unknown-linux-gnueabihf`
- ⚠️ `x86_64-unknown-linux-musl` (may have issues)

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
# Container runtime: auto, docker, podman
# Note: youki (pure Rust OCI runtime) will be supported in a future release
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

## 🔧 Linker Configuration

For successful cross-compilation, you often need to configure linkers. xcargo makes this easy:

### Windows Cross-Compilation (from macOS/Linux)

```toml
[targets."x86_64-pc-windows-gnu"]
linker = "x86_64-w64-mingw32-gcc"

[targets."x86_64-pc-windows-gnu".env]
CC = "x86_64-w64-mingw32-gcc"
AR = "x86_64-w64-mingw32-ar"
```

**Install on macOS:** `brew install mingw-w64`
**Install on Linux:** `sudo apt install mingw-w64`

### Linux Cross-Compilation (from macOS)

```toml
[targets."x86_64-unknown-linux-gnu"]
linker = "x86_64-linux-gnu-gcc"

[targets."x86_64-unknown-linux-gnu".env]
CC = "x86_64-linux-gnu-gcc"
```

**Note:** Linux cross-compilation from macOS often requires containers

### What xcargo does automatically:
- ✅ Verifies linker exists in PATH before building
- ✅ Sets `CARGO_TARGET_*_LINKER` environment variable
- ✅ Applies custom environment variables (`CC`, `AR`, etc.)
- ✅ Shows helpful errors with installation instructions if linker is missing

## 🐳 Container Builds

For complex cross-compilation scenarios where native toolchains are difficult to set up, xcargo supports container-based builds using Docker or Podman.

### Installation with Container Support

```bash
# Install xcargo with container feature
cargo install xcargo --features container

# Ensure you have Docker or Podman installed
docker --version  # or: podman --version
```

### When to Use Containers

Container builds are ideal when:
- 🌐 Cross-compiling between different operating systems (e.g., macOS → Linux)
- 🔧 Native toolchains are difficult to install or configure
- 🎯 You need reproducible builds across different development machines
- 📦 Your project has complex system dependencies

### Basic Usage

```bash
# Build using a container
xcargo build --target x86_64-unknown-linux-gnu --container

# Container builds work with all flags
xcargo build --target aarch64-unknown-linux-gnu --container --release
```

### Automatic Container Detection

Configure xcargo to automatically use containers when needed:

```toml
# xcargo.toml
[container]
# Container runtime: auto, docker, podman
runtime = "auto"

# Automatically use containers when cross-compiling to different OS
use_when = "target.os != host.os"

# Or always use containers
# use_when = "always"

# Or never use containers
# use_when = "never"

# Image pull policy: always, if-not-present, never
pull_policy = "if-not-present"
```

With this configuration, xcargo will automatically use containers when building for a different OS:

```bash
# On macOS, this will automatically use a container
xcargo build --target x86_64-unknown-linux-gnu

# On Linux, this will use native toolchain
xcargo build --target x86_64-unknown-linux-gnu
```

### Supported Container Targets

xcargo uses pre-built images from [cross-rs](https://github.com/cross-rs/cross) for these targets:

**Linux:**
- `x86_64-unknown-linux-gnu`
- `x86_64-unknown-linux-musl`
- `aarch64-unknown-linux-gnu`
- `aarch64-unknown-linux-musl`
- `armv7-unknown-linux-gnueabihf`
- `arm-unknown-linux-gnueabihf`

**Windows:**
- `x86_64-pc-windows-gnu`

**Android:**
- `aarch64-linux-android`
- `armv7-linux-androideabi`
- `x86_64-linux-android`
- `i686-linux-android`

**Note:** macOS and WebAssembly targets don't use containers:
- macOS cross-compilation requires osxcross or building on macOS
- WebAssembly builds work natively without containers

### Container Requirements

**Docker:**
```bash
# macOS
brew install --cask docker
# Or download from https://www.docker.com/products/docker-desktop

# Linux
sudo apt install docker.io  # Ubuntu/Debian
sudo dnf install docker      # Fedora
sudo systemctl start docker
sudo usermod -aG docker $USER  # Add yourself to docker group
```

**Podman (Docker-compatible alternative):**
```bash
# macOS
brew install podman
podman machine init
podman machine start

# Linux
sudo apt install podman     # Ubuntu/Debian
sudo dnf install podman      # Fedora
```

### Advanced Configuration

```toml
[container]
# Prefer specific runtime
runtime = "podman"

# Custom registry for images
registry = "my-registry.com/cross-images"

# Always use containers for reproducible builds
use_when = "always"

# Per-target container configuration
[targets."x86_64-unknown-linux-gnu"]
linker = "x86_64-linux-gnu-gcc"

[targets."x86_64-unknown-linux-gnu".env]
CC = "x86_64-linux-gnu-gcc"
CUSTOM_VAR = "value"
```

### How Container Builds Work

1. **Runtime Detection** - Finds Docker or Podman on your system
2. **Image Selection** - Chooses the appropriate cross-rs image for your target
3. **Volume Mounting** - Mounts your project and cargo cache into the container
4. **Build Execution** - Runs `cargo build` inside the container
5. **Artifact Extraction** - Build artifacts appear in your local `target/` directory

```
┌─────────────────────────────────┐
│ xcargo build --container        │
└──────────┬──────────────────────┘
           │
           ▼
   ┌───────────────────┐
   │ Detect container  │
   │ runtime (Docker/  │
   │ Podman)           │
   └───────┬───────────┘
           │
           ▼
   ┌───────────────────┐
   │ Pull cross-rs     │
   │ image for target  │
   └───────┬───────────┘
           │
           ▼
   ┌───────────────────┐
   │ Mount project &   │
   │ cargo cache       │
   └───────┬───────────┘
           │
           ▼
   ┌───────────────────┐
   │ Run cargo build   │
   │ in container      │
   └───────┬───────────┘
           │
           ▼
   ┌───────────────────┐
   │ Extract artifacts │
   │ to local target/  │
   └───────────────────┘
```

### Troubleshooting

**Container runtime not found:**
```bash
# Check if Docker/Podman is installed and running
docker info
# or
podman info
```

**Permission denied errors (Linux):**
```bash
# Add your user to the docker group
sudo usermod -aG docker $USER
# Then log out and back in
```

**Image pull failures:**
```bash
# Manually pull the image
docker pull ghcr.io/cross-rs/x86_64-unknown-linux-gnu:latest

# Check your network/proxy settings
docker info | grep -i proxy
```

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

**Current Version:** 0.2.0

✅ **Working Features:**
- Target detection and validation
- Toolchain management via rustup
- Basic cross-compilation
- Configuration system (xcargo.toml)
- Interactive TUI setup wizard
- Beautiful colored output with tips
- Self-building capability (xcargo builds itself!)
- **Parallel target compilation** (2-3x speedup with `parallel = true`)
- **Linker configuration** (automatic CARGO_TARGET_*_LINKER setup)
- **Container builds** (Docker/Podman integration with `--features container`)
- Smart error messages with platform-specific help
- GitHub Actions CI/CD integration

🚧 **Planned Features:**
- **Bundled cross-compilation toolchains** - Zero-dependency builds without Docker (download minimal toolchains on-demand)
- Pure Rust OCI runtime (youki integration) as optional feature
- Native dependency management
- Build caching improvements
- Custom container image support

## 🗺️ Roadmap

### Phase 1: Core Cross-Compilation (v0.1-0.2) ✅
- ✅ Target detection and validation
- ✅ Toolchain management via rustup
- ✅ Configuration system (xcargo.toml)
- ✅ Interactive TUI setup wizard
- ✅ Parallel builds for 2-3x speedup
- ✅ Linker configuration
- ✅ Container builds (Docker/Podman)
- ✅ Binary signing (minisign)

### Phase 2: Zero-Dependency Builds (v0.3) 🚧
**Goal:** Make cross-compilation work out of the box with zero external dependencies.

- [ ] **Bundled toolchain system**
  - Download minimal cross-compilation toolchains on-demand (~20-50MB per target)
  - Cache in `~/.xcargo/toolchains/`
  - No Docker, no manual toolchain installation required
  - Fallback to containers for complex targets
- [ ] Build and host pre-compiled toolchains for tier 1 targets
- [ ] Automatic toolchain updates

**Trade-offs:**
- ✅ Better UX: Just works, no setup needed
- ✅ Smaller downloads: 20-50MB vs 500MB+ containers
- ✅ Offline-friendly: Works after first download
- ⚠️ More complexity: Need to build/maintain toolchains
- ⚠️ Hosting costs: Bandwidth for toolchain downloads

### Phase 3: Enhanced Security & Distribution (v0.4)
**Platform-Specific Code Signing** 🎯

Currently, xcargo binaries are signed with **minisign** (free, cross-platform). For enhanced platform integration, we're considering:

**macOS Code Signing:**
- **What:** Sign with Apple Developer certificate, enable Gatekeeper
- **Benefit:** Better macOS user experience, no "unidentified developer" warnings
- **Cost:** $99/year Apple Developer Program
- **Status:** Planned, pending community interest

**Windows Authenticode:**
- **What:** Sign with code signing certificate, satisfy SmartScreen
- **Benefit:** Better Windows user experience, no security warnings
- **Cost:** $100-500/year for certificate
- **Status:** Planned, pending community interest

**Want platform-specific signing?**
- 👍 React to [#123](https://github.com/ibrahimcesar/xcargo/issues/123) if you want macOS signing
- 👍 React to [#124](https://github.com/ibrahimcesar/xcargo/issues/124) if you want Windows signing
- 💬 Share your use case in the issues

> **Note:** Platform signing requires paid certificates. We'll implement these features when there's sufficient community interest to justify the ongoing costs. Minisign signatures will always be provided as a free, cross-platform verification method.

**Other v0.4 features:**
- [ ] `xcargo sign` - Help users sign their own binaries
- [ ] Native dependency detection and management
- [ ] Advanced build caching
- [ ] Build profiles and presets

### Phase 4: Advanced Features (v0.5+)
- [ ] youki integration (pure Rust OCI runtime)
- [ ] Custom toolchain registry
- [ ] Build reproducibility guarantees
- [ ] SBOM (Software Bill of Materials) generation
- [ ] Integration with cargo-dist
- [ ] Plugin system for custom targets

### Community-Driven Priorities

We prioritize features based on community feedback. Share your needs:
- 🐛 [Report bugs](https://github.com/ibrahimcesar/xcargo/issues/new?template=bug_report.md)
- 💡 [Request features](https://github.com/ibrahimcesar/xcargo/issues/new?template=feature_request.md)
- 💬 [Join discussions](https://github.com/ibrahimcesar/xcargo/discussions)

## 🆚 Comparison

| Feature | xcargo | cross | cargo-zigbuild |
|---------|--------|-------|----------------|
| **Native-first** | ✅ | ❌ | ⚠️ Via Zig |
| **Auto-install targets** | ✅ | ❌ | ❌ |
| **Interactive setup** | ✅ | ❌ | ❌ |
| **Parallel builds** | ✅ | ❌ | ❌ |
| **Beautiful output** | ✅ | ⚠️ | ⚠️ |
| **Configuration file** | ✅ | ✅ | ❌ |
| **Container support** | ✅ | ✅ | ❌ |
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
- [Container Security Guide](docs/guides/container-security.md) - 🔒 Best practices for secure builds

## 🔒 Security

xcargo is built with security in mind:

- ✅ **Memory safe** - Written in Rust with zero unsafe code
- ✅ **Input validation** - All inputs are validated and sanitized
- ✅ **No shell execution** - Direct process execution prevents injection attacks
- ✅ **Audited dependencies** - All dependencies are regularly scanned
- ✅ **Security evaluation** - Comprehensive security review completed for v1.0.0

**Security resources:**
- [Security Policy](SECURITY.md) - Vulnerability reporting process
- [Security Evaluation](.github/SECURITY_EVALUATION.md) - Detailed security analysis
- [Container Security Guide](docs/guides/container-security.md) - Secure container builds

**Report security issues:** security@xcargo.dev or via [GitHub Security Advisories](https://github.com/ibrahimcesar/xcargo/security/advisories)

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
