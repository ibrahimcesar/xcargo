---
sidebar_position: 1
---

# Design Philosophy

The core principles and philosophy behind xcargo's design.

## Mission Statement

> **Make cross-compilation in Rust boring (in a good way)**

Cross-compilation should be a solved problem. Developers shouldn't need to think about linkers, toolchains, or container configurations. They should just build for their target platforms and get on with their work.

## Core Principles

### 1. Zero Friction

**Problem**: Cross-compilation in Rust requires manual setup of toolchains, linkers, and often Docker containers.

**Solution**: xcargo detects requirements automatically and guides users through any necessary setup.

```bash
# Before xcargo
rustup target add aarch64-unknown-linux-gnu
sudo apt-get install gcc-aarch64-linux-gnu
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
cargo build --target aarch64-unknown-linux-gnu

# With xcargo
xcargo build --target linux-arm64
```

### 2. Native First

**Rationale**: Container-based builds are ~2-3x slower than native builds due to:
- Volume mounting overhead
- Container startup time
- File system performance

**Implementation**: xcargo attempts native cross-compilation first, using containers only when necessary.

**Trade-off**: Native cross-compilation requires platform-specific toolchains to be installed. We accept this trade-off because:
- Installation is one-time per development machine
- xcargo provides clear installation instructions
- Speed improvement is significant for iterative development

### 3. Transparency

**Problem**: Tools that "just work" often hide important details, making debugging difficult.

**Solution**: xcargo shows what it's doing at each step:

```bash
$ xcargo build --target windows
🔍 Detecting target: x86_64-pc-windows-gnu
🔧 Linker: x86_64-w64-mingw32-gcc (found)
🚀 Building natively (no container needed)
   Compiling myapp v0.1.0
```

Users understand:
- Which target triple is being used
- Whether native or container build
- What linker is active

### 4. Smart Defaults

**Philosophy**: The common case should be effortless; the uncommon case should be possible.

**Examples**:
- Target aliases (`linux` → `x86_64-unknown-linux-gnu`)
- Automatic tier classification
- Sensible container runtime selection (youki → docker → podman)

**Override Path**: Every default can be overridden:

```toml
[build]
force_container = true  # Always use containers

[targets."x86_64-pc-windows-gnu"]
linker = "custom-linker"  # Custom linker
```

### 5. Composability

**Design**: xcargo is designed to work with existing tools:

- ✅ **Works with Cargo**: Leverages Cargo for actual compilation
- ✅ **Works with rustup**: Uses rustup for target management
- ✅ **Works with CI/CD**: Easy integration with GitHub Actions, etc.
- ✅ **Works as wrapper**: Can wrap cargo commands

Not a replacement, but an enhancement to the existing ecosystem.

## Non-Goals

### What xcargo is NOT

1. **Not a build system**
   - xcargo is a cross-compilation helper, not a replacement for Cargo
   - It invokes Cargo under the hood

2. **Not a package manager**
   - xcargo doesn't manage Rust crates
   - It manages cross-compilation toolchains and strategies

3. **Not a compiler**
   - xcargo doesn't compile Rust code
   - It orchestrates the compilation process

4. **Not Docker-required**
   - Unlike some tools, xcargo doesn't require Docker
   - Container support is optional and automatic

## Design Influences

### Inspiration From

- **[cross](https://github.com/cross-rs/cross)**: Container-based approach, but xcargo prioritizes native builds
- **[cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild)**: Zig linker approach, but xcargo uses native toolchains
- **[rustup](https://github.com/rust-lang/rustup)**: Excellent CLI/UX design
- **[Cargo](https://github.com/rust-lang/cargo)**: Simple, powerful defaults

### Divergence Points

| Aspect | cross | cargo-zigbuild | xcargo |
|--------|-------|----------------|--------|
| **Primary strategy** | Container | Zig linker | Native toolchains |
| **Fallback** | None | None | Container |
| **Setup** | Docker required | Zig required | Auto-detect |
| **Speed** | Slower | Medium | Fast (native) |

## Evolution

xcargo's design will evolve, but these principles remain:

✅ **Will always**: Prioritize native builds for speed  
✅ **Will always**: Be transparent about operations  
✅ **Will always**: Provide container fallback  
✅ **Will always**: Have sensible defaults  

❌ **Won't**: Require specific tools (Docker, Zig, etc.)  
❌ **Won't**: Hide what's happening  
❌ **Won't**: Replace Cargo  

## Community Input

Design decisions are documented and discussed openly:

- RFCs for major changes
- GitHub Discussions for proposals
- Issues for specific problems

We welcome feedback on design trade-offs!

## Next Steps

- [Native-First Strategy](./native-first.md) - Deep dive on build strategy
- [Tier System](./tier-system.md) - How targets are classified
- [Trade-offs](./trade-offs.md) - Technical trade-offs explained
