---
sidebar_position: 3
---

# Quick Start

Get up and running with xcargo in minutes.

## Basic Workflow

### 1. Initialize Your Project

```bash
# Initialize cross-compilation for your Rust project
xcargo init
```

This creates an `xcargo.toml` configuration file with sensible defaults.

### 2. Add Target Platforms

```bash
# Add common targets using aliases
xcargo target add linux windows macos

# Or use full target triples
xcargo target add x86_64-unknown-linux-gnu
```

### 3. Check System Requirements

```bash
# Check what's needed for your targets
xcargo doctor

# Output:
# ✅ x86_64-unknown-linux-gnu: Ready
# ❌ x86_64-pc-windows-gnu: Missing linker
#     Install: sudo apt-get install mingw-w64
```

### 4. Build for Targets

```bash
# Build for a specific target
xcargo build --target windows

# Build for all configured targets
xcargo build --all

# Release build
xcargo build --all --release
```

## Example Session

Here's a complete example of cross-compiling a Rust project:

```bash
# Start with a new project
cargo new my-app
cd my-app

# Initialize xcargo
xcargo init

# Add targets
xcargo target add linux windows macos

# Check requirements
xcargo doctor

# Install missing tools (if needed)
sudo apt-get install mingw-w64

# Build for all platforms
xcargo build --all --release

# Binaries are in target/{triple}/release/
ls target/*/release/my-app*
```

## Using as Cargo Wrapper

xcargo can also wrap cargo commands:

```bash
# Run cargo commands through xcargo
xcargo cargo build --target x86_64-pc-windows-gnu
xcargo cargo test --target aarch64-unknown-linux-gnu
```

## Configuration File

The `xcargo.toml` file allows you to configure defaults:

```toml
[targets]
default = [
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-gnu",
    "x86_64-apple-darwin",
]

[build]
parallel = true
cache = true

[container]
runtime = "auto"  # auto, youki, docker, podman
```

## Next Steps

- [Basic Usage Guide](./guides/basic-usage.md) - Detailed usage instructions
- [Target Management](./guides/target-management.md) - Learn about target aliases and management
- [CI/CD Integration](./guides/ci-cd-integration.md) - Set up automated builds
