---
sidebar_position: 5
---

# Technical Trade-offs

Understanding the technical decisions and their implications.

## Native Builds vs Container Builds

### Native Build Approach

**Advantages**:
- ⚡ **2-3x faster**: No container overhead
- 🔄 **Incremental builds**: Full Cargo incremental compilation support
- 💾 **Less disk usage**: No container images to store
- 🐛 **Easier debugging**: Direct access to build process

**Disadvantages**:
- 🔧 **Toolchain installation**: Requires platform-specific tools
- 🖥️ **Platform limitations**: Some targets can't be cross-compiled natively
- 📦 **System dependencies**: Need to install system packages

**When to use**:
- Development iteration (frequent rebuilds)
- CI/CD with pre-installed toolchains
- Targets with available native toolchains (Linux → Windows, etc.)

### Container Build Approach

**Advantages**:
- 📦 **Self-contained**: All tools bundled in image
- 🎯 **Reproducible**: Consistent environment across machines
- 🌍 **Universal**: Works for any target with an image
- 🔒 **Isolated**: Build doesn't affect host system

**Disadvantages**:
- 🐌 **Slower**: 2-3x build time overhead
- 💾 **Disk usage**: Container images can be large (1-2 GB)
- 🔧 **Setup**: Requires container runtime (though xcargo includes youki)
- 🔄 **Incremental builds**: Less effective due to volume mounting

**When to use**:
- First-time setup (no toolchains installed)
- Specialized targets (embedded, WASM with specific tooling)
- Isolated build environments
- Targets without native cross-compilation support

## Tier System Trade-offs

### Tier 1: Native

**Classification criteria**:
- Common host-to-target combinations
- Widely available cross-compilation toolchains
- Well-tested compilation paths

**Examples**: x86_64-unknown-linux-gnu, x86_64-pc-windows-gnu

**Trade-off**: Limited to well-supported targets, but ensures fast builds.

### Tier 2: Container

**Classification criteria**:
- Less common cross-compilation scenarios
- Toolchains available but not commonly installed
- Requires additional system setup

**Examples**: aarch64-unknown-linux-gnu (from x86_64), RISC-V targets

**Trade-off**: Broader target support at cost of slower builds.

### Tier 3: Specialized

**Classification criteria**:
- Mobile platforms (iOS, Android)
- Embedded systems
- WebAssembly with specific requirements

**Examples**: aarch64-apple-ios, thumbv7em-none-eabi, wasm32-wasi

**Trade-off**: Maximum flexibility but may require platform-specific SDKs.

## Embedded Runtime vs External Container Runtime

### Youki (Embedded)

**Advantages**:
- 📦 **No installation**: Bundled with xcargo
- ⚡ **Fast startup**: Lightweight Rust implementation
- 🔒 **Security**: Smaller attack surface
- 🦀 **Pure Rust**: Consistent with project ecosystem

**Disadvantages**:
- 🆕 **Less mature**: Newer than Docker/Podman
- 🔧 **Limited features**: Subset of OCI runtime spec
- 📝 **Compatibility**: Some advanced Docker features unsupported

**When to use**:
- Fresh installations
- Minimal container usage
- Security-conscious environments

### Docker/Podman (External)

**Advantages**:
- 🏭 **Mature**: Battle-tested and widely used
- 🔧 **Full featured**: Complete container runtime
- 🌍 **Ecosystem**: Vast image repository
- 📚 **Documentation**: Extensive resources

**Disadvantages**:
- 📦 **Requires installation**: Must be installed separately
- 💾 **Resource usage**: More memory/disk overhead
- ⚙️ **Complexity**: More moving parts

**When to use**:
- Already using Docker/Podman
- Need advanced container features
- Existing container-based workflows

## Automatic Detection vs Manual Configuration

### Automatic Detection

**What we detect**:
```rust
- Host target triple
- Installed Rust targets
- Available linkers
- Cross-compilation toolchains
- Container runtimes
```

**Advantages**:
- 🚀 **Fast setup**: No configuration needed
- 🔄 **Always current**: Adapts to system changes
- 🎯 **Accurate**: Based on actual system state

**Disadvantages**:
- ⏱️ **Startup overhead**: Detection takes ~100-200ms
- 🔍 **Opaque**: May not match user expectations
- 💾 **No caching**: Re-detects every run (currently)

**Future improvement**: Cache detection results with invalidation.

### Manual Configuration

**Configuration options**:
```toml
[targets."x86_64-pc-windows-gnu"]
linker = "x86_64-w64-mingw32-gcc"
force_container = false

[build]
parallel = true
jobs = 4
```

**Advantages**:
- 🎛️ **Control**: Explicit configuration
- 🔒 **Reproducible**: Same config → same behavior
- 🐛 **Debugging**: Clear what's configured

**Disadvantages**:
- 📝 **Maintenance**: Must update when system changes
- 🔧 **Complexity**: More to learn and configure

**Balance**: Auto-detect by default, allow manual override.

## Parallel vs Sequential Builds

### Parallel Builds

**Strategy**: Build multiple targets concurrently

```rust
// Pseudo-code
for target in targets {
    spawn_thread(|| build_target(target));
}
```

**Advantages**:
- ⚡ **Faster**: Multiple CPUs utilized
- 🎯 **Efficient**: Minimize total wall-clock time

**Disadvantages**:
- 💾 **Memory usage**: Multiple rustc instances
- 🔥 **CPU heat**: All cores at 100%
- 📊 **Output mixing**: Interleaved build logs

**Mitigation**: Limit concurrent jobs, buffer output

### Sequential Builds

**Strategy**: Build one target at a time

**Advantages**:
- 📊 **Clear output**: Linear build logs
- 💾 **Lower memory**: One rustc at a time
- 🐛 **Easier debugging**: Isolated failures

**Disadvantages**:
- 🐌 **Slower**: Sequential execution
- ⏱️ **Wasted time**: CPU idle during I/O

**Balance**: Parallel by default, sequential for `--verbose` or `--debug`

## Caching Strategies

### Build Artifact Caching

**Current**: Cargo's native incremental compilation

**Future**: Cross-target artifact caching

**Trade-off**: Disk space vs build speed

### Tool Detection Caching

**Planned**: Cache linker/tool detection results

```rust
// Cache structure
{
    "linkers": {
        "x86_64-pc-windows-gnu": {
            "path": "/usr/bin/x86_64-w64-mingw32-gcc",
            "checked_at": "2025-01-15T10:30:00Z"
        }
    }
}
```

**Invalidation**: Time-based (1 hour) or manual (`xcargo clean --cache`)

**Trade-off**: Stale detection vs startup speed

## Error Handling Philosophy

### Fail Fast vs Fallback

**Current approach**: Fail fast with clear error messages

```
❌ Target x86_64-pc-windows-gnu: Missing linker
   Install: sudo apt-get install mingw-w64
```

**Alternative**: Try alternative linkers

```rust
// Could try multiple linkers:
["x86_64-w64-mingw32-gcc", "gcc", "clang"]
```

**Trade-off**: 
- ✅ Fail fast: Clear about what's wrong
- ❌ Auto-fallback: May use wrong linker silently

**Decision**: Fail fast for predictability

## Next Steps

- [Native-First Strategy](./native-first.md) - Build strategy details
- [Tier System](./tier-system.md) - Target classification
- [Container Strategy](./container-strategy.md) - Container approach
