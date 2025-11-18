<div align="center">
  
# Apex 🎯🦀

_The apex cross-compilator_

**apex** is a Rust cross-compilation tool that simplifies building for multiple targets. Automatic toolchain management, intelligent container usage, and zero-configuration cross-compilation.

</div>

## 🎯 What is apex?

Cross-compilation in Rust shouldn't be painful. **apex** automates the entire process:

- **Detects** what you need automatically
- **Installs** toolchains and dependencies
- **Builds** for any target with one command
- **Uses containers** only when necessary (includes embedded runtime)

## ✨ Features (Planned)

- 🎯 **Zero Configuration** - Works out of the box for most targets
- 🔧 **Auto-Detection** - Figures out what toolchains you need
- 🐳 **Smart Containers** - Uses native builds when possible, containers when needed
- ⚡ **Fast** - Parallel builds, intelligent caching
- 🌍 **Many Targets** - Linux, Windows, macOS, mobile, embedded
- 🤖 **CI/CD Ready** - Perfect for GitHub Actions, GitLab CI
- 📦 **Embedded Runtime** - No Docker Desktop required (uses youki)

## 🚧 Status

**Work in Progress** - Early development

Current version: `0.1.0-alpha`

## 🚀 Quick Example (Planned API)
```bash
# Initialize cross-compilation for your project
apex init

# Add target platforms
apex target add windows linux macos

# Check what's needed
apex doctor
# ✅ windows-x86_64: Ready
# ❌ linux-arm64: Missing linker (install: apt install gcc-aarch64-linux-gnu)
# ⚠️  macos-aarch64: Requires macOS host for native compilation

# Build for all configured targets
apex build --all

# Build for specific target
apex build --target x86_64-pc-windows-gnu

# Or use as cargo wrapper
apex cargo build --target x86_64-pc-windows-gnu
```

## 📦 Installation
```bash
# Not yet published - coming soon!
cargo install apex

# Or build from source:
git clone https://github.com/yourusername/apex
cd apex
cargo build --release
```

## 🗺️ Roadmap

### Phase 1: Core (Current)
- [ ] Target detection and validation
- [ ] Toolchain management
- [ ] Basic native cross-compilation
- [ ] Configuration system

### Phase 2: Containers
- [ ] Embedded container runtime (youki)
- [ ] Docker/Podman fallback
- [ ] Image caching
- [ ] Native-first strategy

### Phase 3: Advanced
- [ ] Build profiles (release-all, embedded, mobile)
- [ ] Parallel builds
- [ ] Dependency management (OpenSSL, SQLite, etc.)
- [ ] Custom target definitions

### Phase 4: Integration
- [ ] GitHub Actions integration
- [ ] GitLab CI templates
- [ ] Pre-built binaries distribution
- [ ] GUI/TUI interface

## 🎯 Supported Targets

See [TARGETS.md](TARGETS.md) for the complete list.

**Tier 1 (Native builds):**
- x86_64-unknown-linux-gnu
- x86_64-unknown-linux-musl
- x86_64-pc-windows-gnu
- x86_64-apple-darwin
- aarch64-apple-darwin

**Tier 2 (Container builds):**
- aarch64-unknown-linux-gnu
- armv7-unknown-linux-gnueabihf
- x86_64-pc-windows-msvc
- wasm32-unknown-unknown

**Tier 3 (Specialized):**
- Mobile (Android, iOS)
- Embedded (ARM Cortex-M)

## 🛠️ How It Works

```
┌─────────────────────────────────┐
│ apex build --target windows     │
└────────────┬────────────────────┘
             │
             ▼
     ┌───────────────┐
     │ Can compile   │
     │ natively?     │
     └───┬───────┬───┘
         │       │
      YES│       │NO
         │       │
         ▼       ▼
    ┌────────┐ ┌──────────────────┐
    │ Native │ │ Need container?  │
    │ build  │ │ Check deps...    │
    └────────┘ └────┬─────────────┘
                    │
                    ▼
            ┌───────────────┐
            │ Use youki     │
            │ (embedded)    │
            └───────────────┘
```

## 📖 Usage Examples

### Basic Cross-Compilation
```bash
# Build for Windows from Linux
apex build --target x86_64-pc-windows-gnu

# Build for Linux ARM
apex build --target aarch64-unknown-linux-gnu

# Build for all targets
apex build --all
```

### Configuration File
```toml
# apex.toml
[targets]
default = ["x86_64-unknown-linux-gnu", "x86_64-pc-windows-gnu"]

[profiles.release-all]
targets = [
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-gnu",
    "x86_64-apple-darwin",
]

[build]
parallel = true
cache = true

[container]
runtime = "auto"  # auto, youki, docker, podman
use-when = "target.os != host.os"
```

### CI/CD Integration
```yaml
# .github/workflows/build.yml
name: Cross-Platform Build

on: [push]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install apex
        run: cargo install apex
      
      - name: Build all targets
        run: apex build --all
      
      - name: Upload artifacts
        run: apex release --upload
```

## 🎨 Design Goals

**Make cross-compilation boring (in a good way):**

- ✅ **Just Works™** - Sensible defaults for everything
- ✅ **Fast** - Native when possible, containerized when needed
- ✅ **Smart** - Detects and suggests solutions automatically
- ✅ **Transparent** - Shows exactly what it's doing
- ✅ **Flexible** - Override any behavior when needed

## 🆚 Comparison

| Feature | apex | cross | cargo-zigbuild | Manual |
|---------|------|-------|----------------|--------|
| **Native builds** | ✅ First | ❌ | ⚠️ Via Zig | ✅ |
| **Container fallback** | ✅ | ✅ | ❌ | ❌ |
| **No Docker required** | ✅ youki | ❌ | ✅ | ✅ |
| **Auto-setup** | ✅ | ❌ | ⚠️ | ❌ |
| **Native deps** | ✅ Planned | ⚠️ | ❌ | ⚠️ |
| **CI/CD templates** | ✅ Planned | ⚠️ | ❌ | ❌ |

## 🤝 Contributing

Contributions welcome! This project is in early stages.

**How to help:**
- 🐛 Report issues or suggest features
- 💻 Submit PRs for bug fixes or features
- 📝 Improve documentation
- 🎯 Test on different platforms
- 🔧 Add support for new targets

## 📝 License

[MIT](./LICENSE)

## 🙏 Acknowledgments

Inspired by:
- [cross](https://github.com/cross-rs/cross) - Container-based cross-compilation
- [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) - Zig linker approach
- [xwin](https://github.com/Jake-Shadle/xwin) - Windows SDK management
- [youki](https://github.com/containers/youki) - Container runtime in Rust

---

**apex** - *The apex cross-compilator for Rust* 🎯🦀

*Status: 🚧 Pre-alpha - Architecture planning*

**Star** ⭐ this repo to follow development!
