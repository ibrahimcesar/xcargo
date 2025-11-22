---
sidebar_position: 2
---

# Installation

Learn how to install xcargo on your system.

## Prerequisites

- **Rust** 1.70 or later
- **rustup** (for target management)
- **Git** (for building from source)

## Install from crates.io

:::info Coming Soon
xcargo is not yet published to crates.io. Use the source installation method below.
:::

Once published, you'll be able to install with:

```bash
cargo install xcargo
```

## Install from Source

Clone the repository and build from source:

```bash
# Clone the repository
git clone https://github.com/ibrahimcesar/xcargo
cd xcargo

# Build and install
cargo install --path .
```

## Verify Installation

Check that xcargo is installed correctly:

```bash
xcargo --version
```

You should see output like:

```
xcargo 0.1.0
```

## Platform-Specific Notes

### Linux

xcargo works best on Linux for cross-compilation. Most cross-compilation toolchains are readily available:

```bash
# Debian/Ubuntu - Install common cross-compilation tools
sudo apt-get install gcc-aarch64-linux-gnu gcc-arm-linux-gnueabihf mingw-w64

# Fedora/RHEL
sudo dnf install gcc-aarch64-linux-gnu gcc-arm-linux-gnu mingw64-gcc
```

### macOS

On macOS, you can install cross-compilation toolchains via Homebrew:

```bash
# Install mingw for Windows cross-compilation
brew install mingw-w64

# For Linux cross-compilation (optional)
brew tap messense/macos-cross-toolchains
brew install aarch64-unknown-linux-gnu
```

### Windows

xcargo can be used on Windows, though cross-compilation from Windows has some limitations. Consider using WSL2 for the best experience.

## Next Steps

- [Quick Start](./quick-start.md) - Learn basic usage
- [Target Management](./guides/target-management.md) - Manage compilation targets
