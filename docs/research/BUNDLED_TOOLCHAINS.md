# Bundled Toolchains Design

**Goal:** Zero-dependency cross-compilation - no Docker, no external toolchains, just `cargo install xcargo` and it works.

## ⚠️ Critical Discovery: Platform-Specific Toolchains Required

**Update (2025-11-18)**: Testing revealed that toolchains must match the **host platform**, not just the target:

```
Bootlin toolchains are Linux ELF binaries → Only run on Linux hosts
macOS users need Mach-O binaries → Different toolchain source required
Windows users need PE binaries → Different toolchain source required
```

**Revised Strategy**: Start with **Linux-only bundled toolchains** (v0.3), add macOS/Windows support later.

See [TOOLCHAIN_TESTING_FINDINGS.md](./TOOLCHAIN_TESTING_FINDINGS.md) for detailed test results.

## Architecture

```
┌─────────────────────────────────┐
│ xcargo build --target linux     │
└──────────┬──────────────────────┘
           │
           ▼
   ┌───────────────────┐
   │ Check local cache │
   │ ~/.xcargo/        │
   │ toolchains/       │
   └───────┬───────────┘
           │
           ▼
      Toolchain exists?
           │
      ┌────┴────┐
      │         │
     Yes       No
      │         │
      │         ▼
      │    ┌────────────────┐
      │    │ Download from  │
      │    │ GitHub releases│
      │    │ (~20-50MB)     │
      │    └───────┬────────┘
      │            │
      │            ▼
      │    ┌────────────────┐
      │    │ Extract & cache│
      │    └───────┬────────┘
      │            │
      └────────────┘
           │
           ▼
   ┌───────────────────┐
   │ Build with cached │
   │ toolchain         │
   └───────────────────┘
```

## Toolchain Components

For each target, we need:

### 1. Cross-Compiler (GCC or Clang)
```
x86_64-linux-gnu-gcc        (~15MB)
aarch64-linux-gnu-gcc       (~18MB)
x86_64-w64-mingw32-gcc      (~25MB)
```

### 2. Sysroot (Headers + Libraries)
```
libc headers                (~2MB)
system libraries            (~5-10MB)
```

### 3. Binutils
```
ld, ar, as, objcopy         (~3MB)
```

**Total per target: ~20-50MB**

## Implementation Phases

### Phase 1: Research ✓
- [x] Identify approach (bundled toolchains)
- [x] Research existing toolchain sources (Bootlin, musl-cross-make, etc.)
- [x] **Test cross-compilation with manual toolchains** ⚠️ **Linux-only**
- [x] Determine platform-specific requirements
- [ ] Test on Linux platform (GitHub Actions or VM)
- [ ] Research macOS-compatible toolchains

### Phase 2: Core Infrastructure
- [ ] Create `src/toolchains/` module
- [ ] Implement toolchain manifest (TOML with URLs, checksums)
- [ ] Download manager with progress bars
- [ ] Cache management (~/.xcargo/toolchains/)
- [ ] Extraction and setup

### Phase 3: Toolchain Building
- [ ] Build minimal toolchains for tier 1 targets:
  - x86_64-unknown-linux-gnu
  - x86_64-unknown-linux-musl
  - aarch64-unknown-linux-gnu
  - x86_64-pc-windows-gnu
- [ ] Create reproducible build scripts
- [ ] Generate checksums

### Phase 4: Integration
- [ ] Integrate with Builder.build()
- [ ] Auto-detect when bundled toolchain available
- [ ] Fallback to native/container if unavailable
- [ ] Environment variable setup (CC, LD, etc.)

### Phase 5: Distribution
- [ ] Host toolchains on GitHub Releases
- [ ] CDN mirror for faster downloads
- [ ] Version management
- [ ] Update mechanism

## Toolchain Sources

### Option A: Pre-built from distros
```
Ubuntu/Debian: gcc-*-cross packages
Alpine: musl-cross-make toolchains
Homebrew: mingw-w64 builds
```

### Option B: Zig's LLVM/Clang
```
Use Zig's bundled cross-compilers
Requires Zig, but smaller downloads
May have compatibility issues
```

### Option C: Custom builds
```
Build minimal toolchains ourselves
Full control over size and features
Most work, but best results
```

**Recommendation: Start with Option A (pre-built), move to C for optimization**

## Cache Structure

```
~/.xcargo/
├── toolchains/
│   ├── x86_64-unknown-linux-gnu/
│   │   ├── bin/
│   │   │   ├── x86_64-linux-gnu-gcc
│   │   │   ├── x86_64-linux-gnu-ld
│   │   │   └── ...
│   │   ├── lib/
│   │   │   └── gcc/...
│   │   └── sysroot/
│   │       ├── usr/include/
│   │       └── usr/lib/
│   ├── aarch64-unknown-linux-gnu/
│   └── x86_64-pc-windows-gnu/
├── cache/
│   └── downloads/
│       └── x86_64-linux-gnu-v1.0.0.tar.xz
└── manifest.toml
```

## Manifest Format

```toml
version = "1.0.0"

[[toolchain]]
target = "x86_64-unknown-linux-gnu"
version = "1.0.0"
url = "https://github.com/ibrahimcesar/xcargo/releases/download/toolchains/x86_64-linux-gnu.tar.xz"
checksum = "sha256:abc123..."
size = 25165824  # bytes

[[toolchain]]
target = "aarch64-unknown-linux-gnu"
version = "1.0.0"
url = "https://github.com/ibrahimcesar/xcargo/releases/download/toolchains/aarch64-linux-gnu.tar.xz"
checksum = "sha256:def456..."
size = 28311552
```

## Configuration

Add to xcargo.toml:

```toml
[toolchains]
# Prefer bundled toolchains (default)
mode = "bundled"

# Or use system toolchains
# mode = "system"

# Or use containers
# mode = "container"

# Cache directory (default: ~/.xcargo/toolchains)
cache_dir = "~/.xcargo/toolchains"

# Auto-download missing toolchains (default: true)
auto_download = true

# Mirror URLs for faster downloads
mirrors = [
    "https://cdn.xcargo.dev/toolchains",
    "https://github.com/ibrahimcesar/xcargo/releases/download/toolchains",
]
```

## Usage

```bash
# Just works - downloads toolchain on first use
xcargo build --target x86_64-unknown-linux-gnu

# List available bundled toolchains
xcargo toolchain list-remote

# Pre-download toolchains
xcargo toolchain download x86_64-unknown-linux-gnu

# Clean cache
xcargo toolchain clean

# Show cache usage
xcargo toolchain cache-info
```

## Advantages

✅ **Zero friction**: No Docker, no manual toolchain installation
✅ **Fast**: Download once, cache forever
✅ **Small**: 20-50MB per target vs 500MB+ container images
✅ **Reliable**: No daemon, no privileges, works everywhere
✅ **Offline-friendly**: Once cached, works offline

## Challenges

⚠️ **Building toolchains**: Need reproducible builds
⚠️ **Hosting costs**: Bandwidth for downloads
⚠️ **Maintenance**: Keep toolchains updated
⚠️ **Size**: Each target adds to total download size
⚠️ **Platform support**: Need toolchains for macOS, Linux, Windows hosts

## Existing Toolchain Sources

### 1. Ubuntu/Debian Cross Toolchains
**Source:** https://packages.ubuntu.com/

```bash
# Available packages:
gcc-x86-64-linux-gnu
gcc-aarch64-linux-gnu
gcc-arm-linux-gnueabihf
gcc-mingw-w64 (for Windows targets)
```

**Pros:**
- ✅ Well-tested, reliable
- ✅ Regular security updates
- ✅ Complete toolchains

**Cons:**
- ⚠️ Need to extract from .deb packages
- ⚠️ Tied to Ubuntu/Debian versions

### 2. musl-cross-make Toolchains
**Source:** https://musl.cc/ and https://github.com/richfelker/musl-cross-make

Pre-built musl-based toolchains (~15-25MB each):
- x86_64-linux-musl
- aarch64-linux-musl
- arm-linux-musleabihf
- And many more

**Pros:**
- ✅ Static linking (no libc dependencies)
- ✅ Small size (~15-25MB)
- ✅ Pre-built and ready to download
- ✅ Great for Linux targets

**Cons:**
- ⚠️ musl only (not glibc)
- ⚠️ Community-maintained

### 3. Homebrew Linux Toolchains
**Source:** https://formulae.brew.sh/

```bash
brew install x86_64-elf-gcc
brew install aarch64-elf-gcc
brew install mingw-w64
```

**Pros:**
- ✅ Easy to extract from bottles
- ✅ Cross-platform (macOS, Linux)

**Cons:**
- ⚠️ Homebrew dependency

### 4. crosstool-NG Pre-built Toolchains
**Source:** https://crosstool-ng.github.io/

Build-your-own toolchains with reproducible scripts.

**Pros:**
- ✅ Customizable
- ✅ Reproducible builds
- ✅ Many target configurations

**Cons:**
- ⚠️ Need to build them ourselves
- ⚠️ Build time (~30-60 min per target)

### 5. Bootlin Pre-built Toolchains
**Source:** https://toolchains.bootlin.com/

Huge collection of pre-built cross-compilation toolchains!

```
x86-64 targets:
- x86-64--glibc--stable
- x86-64--musl--stable
- x86-64--uclibc--stable

ARM64 targets:
- aarch64--glibc--stable
- aarch64--musl--stable

ARM targets:
- armv7-eabihf--glibc--stable
- armv7-eabihf--musl--stable

And 100+ more combinations!
```

**Pros:**
- ✅ Huge selection (100+ toolchains)
- ✅ Multiple libc options (glibc, musl, uclibc)
- ✅ Well-maintained
- ✅ Direct download links
- ✅ **BEST OPTION** for initial implementation

**Cons:**
- ⚠️ Large downloads (~100-200MB before extraction)
- ⚠️ May need trimming to reduce size

### 6. Zig's Bundled Compilers
**Source:** https://ziglang.org/

Zig bundles LLVM/Clang for cross-compilation.

**Pros:**
- ✅ One download for all targets
- ✅ No external dependencies
- ✅ Works like `zig cc`

**Cons:**
- ⚠️ Requires Zig installation (~60MB)
- ⚠️ Different from traditional GCC toolchains
- ⚠️ May have compatibility issues with some crates

## Recommended Approach

**Phase 1: Quick Win with Bootlin Toolchains**
1. Use Bootlin's pre-built toolchains
2. Download and extract minimal components
3. Repackage into smaller archives (~20-50MB)
4. Host on GitHub Releases

**Phase 2: Optimize with musl-cross-make**
1. Build minimal musl toolchains
2. Strip debug symbols and unused components
3. Further reduce size to ~15-25MB per target

**Phase 3: Custom Minimal Toolchains**
1. Use crosstool-NG to build exactly what we need
2. Minimize size by excluding documentation, extras
3. Target <20MB per toolchain

## Example: Using Bootlin Toolchains

```bash
# Download x86-64 glibc toolchain
wget https://toolchains.bootlin.com/downloads/releases/toolchains/x86-64/tarballs/x86-64--glibc--stable-2023.11-1.tar.bz2

# Extract
tar xf x86-64--glibc--stable-2023.11-1.tar.bz2

# Contents:
x86-64--glibc--stable-2023.11-1/
├── bin/           # Cross-compiler binaries
│   ├── x86_64-buildroot-linux-gnu-gcc
│   ├── x86_64-buildroot-linux-gnu-ld
│   └── ...
├── lib/           # Runtime libraries
├── include/       # Headers
└── sysroot/       # Target sysroot

# Repackage minimal version (just what we need)
tar czf xcargo-toolchain-x86_64-linux-gnu.tar.gz \
  bin/x86_64-buildroot-linux-gnu-gcc \
  bin/x86_64-buildroot-linux-gnu-ld \
  bin/x86_64-buildroot-linux-gnu-ar \
  lib/gcc/ \
  sysroot/
```

## Next Steps

1. ✅ **Research existing toolchains**: Bootlin toolchains identified as best option
2. **Prototype download system**: Test HTTP download + caching
3. **Test manual cross-compilation**: Verify minimal requirements
4. **Build first toolchain**: Download Bootlin x86_64-linux-gnu, repackage minimal version
5. **Implement core module**: `src/toolchains/mod.rs`

## References

- [musl-cross-make](https://github.com/richfelker/musl-cross-make) - Minimal musl toolchains
- [crosstool-ng](https://crosstool-ng.github.io/) - Toolchain generator
- [Zig's approach](https://andrewkelley.me/post/zig-cc-powerful-drop-in-replacement-gcc-clang.html)
- [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild)
- [Ubuntu cross packages](https://packages.ubuntu.com/search?suite=all&section=all&arch=any&keywords=gcc-*-cross&searchon=names)
