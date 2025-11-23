---
sidebar_position: 3
---

# Cross-Compilation Guide

Complete guide to cross-compiling Rust projects with xcargo across different platforms and architectures.

## Overview

xcargo simplifies cross-compilation by automatically:
- Detecting target platforms and their requirements
- Installing necessary toolchains
- Configuring linkers and compilers
- Managing build artifacts

## Common Scenarios

### Scenario 1: Building Windows Executables on macOS/Linux

**Goal**: Create Windows executables from a Unix-based development environment.

**Steps**:

```bash
# 1. Add Windows target
rustup target add x86_64-pc-windows-gnu

# 2. Build using Zig (easiest, no MinGW needed)
xcargo build --target x86_64-pc-windows-gnu --zig --release

# Alternative: Install MinGW and build natively
# macOS
brew install mingw-w64
xcargo build --target x86_64-pc-windows-gnu --release

# Linux (Debian/Ubuntu)
sudo apt install mingw-w64
xcargo build --target x86_64-pc-windows-gnu --release
```

**Output**:
```
target/x86_64-pc-windows-gnu/release/yourapp.exe
```

**Testing on macOS/Linux**:
```bash
# Install Wine to test Windows binaries
brew install wine-stable  # macOS
sudo apt install wine     # Linux

# Run the executable
wine target/x86_64-pc-windows-gnu/release/yourapp.exe
```

### Scenario 2: Building Linux Binaries on macOS

**Goal**: Create Linux executables from macOS for server deployment.

**Steps**:

```bash
# 1. Add Linux target
rustup target add x86_64-unknown-linux-gnu

# 2. Build using Zig (recommended on macOS)
xcargo build --target x86_64-unknown-linux-gnu --zig --release

# Alternative: Use container-based build
xcargo build --target x86_64-unknown-linux-gnu --container --release
```

**Output**:
```
target/x86_64-unknown-linux-gnu/release/yourapp
```

**Testing with Docker**:
```bash
# Test the binary in a Linux container
docker run --rm -v $(pwd):/app -w /app ubuntu:latest \
  ./target/x86_64-unknown-linux-gnu/release/yourapp
```

### Scenario 3: Building for ARM64/Apple Silicon

**Goal**: Create binaries for Apple Silicon Macs or ARM64 Linux servers.

**Steps**:

```bash
# For macOS ARM64 (from Intel Mac)
rustup target add aarch64-apple-darwin
xcargo build --target aarch64-apple-darwin --release

# For Linux ARM64 (from x86_64)
rustup target add aarch64-unknown-linux-gnu

# Install ARM cross-compiler
# macOS
brew install --cask gcc-arm-embedded

# Linux
sudo apt install gcc-aarch64-linux-gnu

# Build
xcargo build --target aarch64-unknown-linux-gnu --release
```

**Configure custom linker** (if needed):

```toml
# xcargo.toml
[[targets]]
triple = "aarch64-unknown-linux-gnu"
linker = "aarch64-linux-gnu-gcc"
```

### Scenario 4: Building Static Binaries with musl

**Goal**: Create fully static Linux binaries with no runtime dependencies.

**Steps**:

```bash
# 1. Add musl target
rustup target add x86_64-unknown-linux-musl

# 2. Install musl tools
# macOS
brew install filosottile/musl-cross/musl-cross

# Linux
sudo apt install musl-tools

# 3. Build static binary
xcargo build --target x86_64-unknown-linux-musl --release

# Verify it's static
ldd target/x86_64-unknown-linux-musl/release/yourapp
# Output: "not a dynamic executable"
```

**Use case**: Docker containers, embedded systems, or portable Linux binaries.

### Scenario 5: Multi-Platform Release Build

**Goal**: Build for all major platforms at once.

**Steps**:

```bash
# 1. Create xcargo.toml with all targets
cat > xcargo.toml << 'EOF'
[project]
name = "my-app"

[[targets]]
triple = "x86_64-pc-windows-gnu"

[[targets]]
triple = "x86_64-unknown-linux-gnu"

[[targets]]
triple = "x86_64-apple-darwin"

[[targets]]
triple = "aarch64-apple-darwin"

[build]
parallel = true
profile = "release"
EOF

# 2. Install all targets
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# 3. Build all targets in parallel
xcargo build --all --zig

# 4. Find all binaries
find target -name "my-app*" -type f -executable
```

**Output structure**:
```
target/
├── x86_64-pc-windows-gnu/release/my-app.exe
├── x86_64-unknown-linux-gnu/release/my-app
├── x86_64-apple-darwin/release/my-app
└── aarch64-apple-darwin/release/my-app
```

### Scenario 6: Building for WebAssembly

**Goal**: Compile Rust to WebAssembly for browser or WASI environments.

**Steps**:

```bash
# 1. Add WASM target
rustup target add wasm32-unknown-unknown

# 2. Build
xcargo build --target wasm32-unknown-unknown --release

# 3. Optional: Optimize with wasm-opt
cargo install wasm-opt
wasm-opt -Oz \
  target/wasm32-unknown-unknown/release/yourapp.wasm \
  -o yourapp-optimized.wasm
```

**For web applications**:

```bash
# Use wasm-pack for web bindings
cargo install wasm-pack

# Build for web
wasm-pack build --target web --release
```

### Scenario 7: Building for Android

**Goal**: Cross-compile for Android ARM64 devices.

**Steps**:

```bash
# 1. Install Android NDK
# Download from: https://developer.android.com/ndk/downloads

# 2. Add Android target
rustup target add aarch64-linux-android

# 3. Configure linker in xcargo.toml
cat > xcargo.toml << 'EOF'
[[targets]]
triple = "aarch64-linux-android"
linker = "/path/to/ndk/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang"
EOF

# 4. Build
xcargo build --target aarch64-linux-android --release
```

### Scenario 8: Building for iOS

**Goal**: Create iOS libraries for Swift integration.

**Steps**:

```bash
# 1. Add iOS targets
rustup target add aarch64-apple-ios      # ARM64 devices
rustup target add x86_64-apple-ios       # Simulator (Intel)
rustup target add aarch64-apple-ios-sim  # Simulator (Apple Silicon)

# 2. Build library
xcargo build --target aarch64-apple-ios --release

# 3. Create universal library for simulators
lipo -create \
  target/x86_64-apple-ios/release/libyourlib.a \
  target/aarch64-apple-ios-sim/release/libyourlib.a \
  -output libyourlib-sim.a
```

## Cross-Compilation Strategies

### Strategy 1: Native Toolchains (Fastest)

Install platform-specific cross-compilers on your host system.

**Pros**:
- Fastest build times
- Direct control over compiler flags
- No additional runtime overhead

**Cons**:
- Complex installation process
- Platform-specific setup
- Version management challenges

**When to use**: Production builds, CI/CD pipelines, frequent builds

**Example**:
```bash
# macOS → Windows
brew install mingw-w64
xcargo build --target x86_64-pc-windows-gnu
```

### Strategy 2: Zig Cross-Compilation (Recommended)

Use Zig's built-in C/C++ cross-compiler.

**Pros**:
- Single tool installation
- Works across platforms
- Excellent Linux cross-compilation
- No separate toolchain management

**Cons**:
- Requires Zig installation
- Some edge cases may need native toolchains
- Additional abstraction layer

**When to use**: Development, macOS → Linux, quick prototyping

**Example**:
```bash
brew install zig  # One-time setup
xcargo build --target x86_64-unknown-linux-gnu --zig
```

### Strategy 3: Container-Based Builds

Use Docker/Podman containers with pre-configured toolchains.

**Pros**:
- Reproducible builds
- Isolated environments
- Pre-configured toolchains
- Great for CI/CD

**Cons**:
- Slower build times
- Requires Docker/Podman
- Larger disk usage

**When to use**: CI/CD, reproducible releases, complex dependencies

**Example**:
```bash
xcargo build --target x86_64-unknown-linux-gnu --container
```

## Platform-Specific Guides

### macOS Host

**Building for Windows**:
```bash
# Option 1: Zig (easiest)
brew install zig
xcargo build --target x86_64-pc-windows-gnu --zig

# Option 2: MinGW
brew install mingw-w64
xcargo build --target x86_64-pc-windows-gnu
```

**Building for Linux**:
```bash
# Option 1: Zig (recommended)
brew install zig
xcargo build --target x86_64-unknown-linux-gnu --zig

# Option 2: Container
xcargo build --target x86_64-unknown-linux-gnu --container
```

**Building for Intel Mac** (from Apple Silicon):
```bash
rustup target add x86_64-apple-darwin
xcargo build --target x86_64-apple-darwin
```

### Linux Host

**Building for Windows**:
```bash
# Install MinGW
sudo apt install mingw-w64  # Debian/Ubuntu
sudo dnf install mingw64-gcc  # Fedora/RHEL

# Build
rustup target add x86_64-pc-windows-gnu
xcargo build --target x86_64-pc-windows-gnu
```

**Building for macOS**:
```bash
# macOS cross-compilation from Linux is complex
# Use container or build on macOS
xcargo build --target x86_64-apple-darwin --container
```

**Building for ARM**:
```bash
# Install ARM toolchain
sudo apt install gcc-aarch64-linux-gnu

# Build
rustup target add aarch64-unknown-linux-gnu
xcargo build --target aarch64-unknown-linux-gnu
```

### Windows Host

**Building for Linux**:
```bash
# Option 1: WSL2 (recommended)
wsl --install
# Then use Linux instructions inside WSL2

# Option 2: Container with Docker Desktop
xcargo build --target x86_64-unknown-linux-gnu --container
```

**Building for macOS**:
```bash
# Not directly supported
# Use CI/CD or build on macOS
```

## Troubleshooting

### Linker Errors

**Problem**: `linker 'cc' not found` or similar

**Solution**:
```bash
# Check what linker is needed
xcargo doctor

# Install appropriate toolchain or use Zig
xcargo build --target <triple> --zig
```

### Missing System Libraries

**Problem**: Build fails with missing library errors

**Solutions**:

1. **Use static linking** (musl):
```bash
rustup target add x86_64-unknown-linux-musl
xcargo build --target x86_64-unknown-linux-musl
```

2. **Use container** with pre-installed dependencies:
```bash
xcargo build --target <triple> --container
```

3. **Install cross-compilation development libraries**:
```bash
# Debian/Ubuntu
sudo dpkg --add-architecture arm64
sudo apt install libssl-dev:arm64
```

### Target Not Installed

**Problem**: `error: target 'x86_64-pc-windows-gnu' not installed`

**Solution**:
```bash
rustup target add x86_64-pc-windows-gnu
xcargo build --target x86_64-pc-windows-gnu
```

See the [Troubleshooting Guide](troubleshooting.md) for more common issues.

## Performance Tips

### 1. Enable Parallel Builds

```toml
# xcargo.toml
[build]
parallel = true
```

```bash
# Or via CLI
xcargo build --all
```

### 2. Use Incremental Compilation

```toml
# Cargo.toml
[profile.dev]
incremental = true
```

### 3. Share Cargo Cache

Container builds automatically share the Cargo registry cache:
```
~/.cargo/registry → /usr/local/cargo/registry
```

### 4. Use Faster Linkers

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

### 5. Optimize Release Builds

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Cross-Platform Build

on: [push, pull_request]

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-pc-windows-gnu
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Install xcargo
        run: cargo install xcargo

      - name: Install Zig (Linux only)
        if: runner.os == 'Linux'
        run: |
          wget https://ziglang.org/download/0.15.2/zig-linux-x86_64-0.15.2.tar.xz
          tar xf zig-linux-x86_64-0.15.2.tar.xz
          echo "$(pwd)/zig-linux-x86_64-0.15.2" >> $GITHUB_PATH

      - name: Build
        run: xcargo build --target ${{ matrix.target }} --release --zig

      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.target }}
          path: target/${{ matrix.target }}/release/*
```

See [CI/CD Integration Guide](ci-cd-integration.md) for more examples.

## Best Practices

### 1. Test on Target Platforms

Always test binaries on actual target platforms or emulators:

```bash
# Windows binary on Linux
wine target/x86_64-pc-windows-gnu/release/app.exe

# Linux binary in container
docker run --rm -v $(pwd):/app ubuntu ./target/.../app

# ARM binary with QEMU
qemu-aarch64 target/aarch64-unknown-linux-gnu/release/app
```

### 2. Document Target Requirements

In your README.md:

```markdown
## Supported Platforms

- Windows x86_64 (GNU)
- Linux x86_64 (glibc 2.31+)
- macOS x86_64 (10.15+)
- macOS ARM64 (11.0+)

## Building from Source

Install xcargo and build for your platform:

\`\`\`bash
cargo install xcargo
xcargo build --target <your-target> --release
\`\`\`
```

### 3. Use Feature Flags for Platform-Specific Code

```rust
// lib.rs
#[cfg(target_os = "windows")]
fn platform_specific() {
    // Windows implementation
}

#[cfg(target_os = "linux")]
fn platform_specific() {
    // Linux implementation
}

#[cfg(target_os = "macos")]
fn platform_specific() {
    // macOS implementation
}
```

### 4. Automate Release Builds

Create a script for multi-platform releases:

```bash
#!/bin/bash
# release.sh

VERSION="v1.0.0"
TARGETS=(
    "x86_64-pc-windows-gnu"
    "x86_64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
)

for target in "${TARGETS[@]}"; do
    echo "Building for $target..."
    xcargo build --target "$target" --release --zig

    # Package
    cd "target/$target/release"
    tar czf "../../../myapp-$VERSION-$target.tar.gz" myapp*
    cd ../../..
done

echo "Release artifacts:"
ls -lh *.tar.gz
```

### 5. Version Pin Your Toolchains

```toml
# rust-toolchain.toml
[toolchain]
channel = "1.82.0"
components = ["rustfmt", "clippy"]
targets = [
    "x86_64-pc-windows-gnu",
    "x86_64-unknown-linux-gnu",
    "x86_64-apple-darwin",
]
```

## Common Target Triples Reference

### Desktop

| Platform | Target Triple | Notes |
|----------|---------------|-------|
| Windows x64 (GNU) | `x86_64-pc-windows-gnu` | MinGW, easiest cross-compile |
| Windows x64 (MSVC) | `x86_64-pc-windows-msvc` | Requires MSVC toolchain |
| Windows x86 | `i686-pc-windows-gnu` | 32-bit Windows |
| Linux x64 (glibc) | `x86_64-unknown-linux-gnu` | Standard Linux |
| Linux x64 (musl) | `x86_64-unknown-linux-musl` | Static binaries |
| macOS x64 | `x86_64-apple-darwin` | Intel Macs |
| macOS ARM64 | `aarch64-apple-darwin` | Apple Silicon |

### Servers & Embedded

| Platform | Target Triple | Notes |
|----------|---------------|-------|
| Linux ARM64 | `aarch64-unknown-linux-gnu` | ARM servers, Raspberry Pi 4+ |
| Linux ARMv7 | `armv7-unknown-linux-gnueabihf` | Raspberry Pi 2/3 |
| Linux ARMv6 | `arm-unknown-linux-gnueabihf` | Raspberry Pi 1/Zero |

### Mobile

| Platform | Target Triple | Notes |
|----------|---------------|-------|
| Android ARM64 | `aarch64-linux-android` | Modern Android devices |
| iOS ARM64 | `aarch64-apple-ios` | iPhone/iPad |
| iOS Simulator (Intel) | `x86_64-apple-ios` | Intel Mac simulator |
| iOS Simulator (ARM) | `aarch64-apple-ios-sim` | Apple Silicon simulator |

### WebAssembly

| Platform | Target Triple | Notes |
|----------|---------------|-------|
| WASM (browser) | `wasm32-unknown-unknown` | Browser/WASI |
| WASM (WASI) | `wasm32-wasi` | Server-side WASM |

## Security Considerations

When cross-compiling, especially using containers, follow these security best practices:

### Container Security 🔒

If using container-based builds (`--container` flag), ensure:

1. **Verify Images** - Use official Rust images or pin to specific digests:
   ```toml
   [container]
   # Pin to specific digest for security
   image = "rust:1.70@sha256:abc123..."
   ```

2. **Rootless Containers** - Use rootless Docker/Podman when possible
3. **Offline Builds** - Vendor dependencies for sensitive builds
4. **Image Scanning** - Scan images for vulnerabilities before use

See [Container Security Guide](container-security.md) for comprehensive best practices.

### Build Environment Trust

xcargo trusts:
- Your xcargo.toml configuration
- System-installed toolchains (rustc, rustup)
- Container images you specify

**Always:**
- ✅ Verify toolchain installations from official sources
- ✅ Review xcargo.toml before using third-party projects
- ✅ Keep Docker/Podman updated for container builds

For security issues, see our [Security Policy](../../SECURITY.md).

## Further Reading

- [Container Security Guide](container-security.md) - **🔒 Best practices for secure container builds**
- [Target Management Guide](target-management.md) - Managing targets and toolchains
- [CI/CD Integration](ci-cd-integration.md) - Using xcargo in continuous integration
- [Troubleshooting Guide](troubleshooting.md) - Common issues and solutions
- [Configuration Reference](../reference/configuration.md) - xcargo.toml options
- [Rust Platform Support](https://doc.rust-lang.org/nightly/rustc/platform-support.html) - Official Rust target documentation
