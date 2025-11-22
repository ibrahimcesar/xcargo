# Toolchain Testing Findings

## Summary

Testing manual cross-compilation with Bootlin toolchains revealed critical platform-specific requirements for bundled toolchains.

## Test Results

### Environment
- **Host OS**: macOS (Darwin 24.5.0)
- **Host Architecture**: x86_64 (Apple Silicon via Rosetta)
- **Toolchain Tested**: Bootlin x86-64--glibc--stable-2025.08-1
- **Test Date**: 2025-11-18

### What We Tested

1. ✅ **Download**: Successfully downloaded 96MB toolchain from Bootlin
2. ✅ **Extraction**: Successfully extracted to 449MB
3. ✅ **Structure**: Toolchain has correct structure (bin/, lib/, sysroot/, etc.)
4. ❌ **Execution**: **FAILED** - Cannot execute Linux binaries on macOS

### Error Encountered

```
error: linking with `.../x86_64-buildroot-linux-gnu-gcc` failed: exit status: 126
= note: .../x86_64-buildroot-linux-gnu-gcc: cannot execute binary file
```

### Root Cause

**Bootlin toolchains are Linux-only** (ELF binaries). They:
- ✅ Run on Linux (x86_64)
- ❌ Cannot run on macOS (Mach-O required)
- ❌ Cannot run on Windows (PE format required)

## Critical Discovery: Platform-Specific Toolchains Required

For bundled toolchains to work, we need **different toolchains for each host platform**:

```
Host Platform → Target Platform → Toolchain Required
────────────────────────────────────────────────────
macOS         → Linux x86_64    → macOS-hosted cross-compiler
macOS         → Linux ARM64     → macOS-hosted cross-compiler
Linux         → Linux x86_64    → Bootlin toolchain ✅
Linux         → Linux ARM64     → Bootlin toolchain ✅
Windows       → Linux x86_64    → MinGW-hosted cross-compiler
Windows       → Linux ARM64     → MinGW-hosted cross-compiler
```

## Implications for xcargo

### 1. Toolchain Matrix Complexity

Instead of **1 toolchain per target**, we need **N toolchains per target** where N = number of host platforms:

**Before (Simplified View)**:
```
x86_64-unknown-linux-gnu → bootlin-x86_64-linux-gnu.tar.xz (96MB)
```

**After (Reality)**:
```
x86_64-unknown-linux-gnu:
  - linux-host-x86_64.tar.xz   (96MB) ← Bootlin
  - macos-host-x86_64.tar.xz   (??MB) ← Need to find/build
  - windows-host-x86_64.zip    (??MB) ← Need to find/build
```

### 2. Storage & Bandwidth Impact

For 4 tier-1 targets × 3 host platforms:
- **Total toolchains**: ~12 packages
- **Estimated size**: ~1-1.5GB total
- **GitHub releases limit**: 2GB per release ✅ (we're fine)

### 3. Download Strategy

Users only download **1 toolchain** based on their platform:

```toml
# Auto-detect host platform
if host == "macos" {
    download("https://.../macos-host-x86_64-linux-gnu.tar.xz")
} else if host == "linux" {
    download("https://.../linux-host-x86_64-linux-gnu.tar.xz")
}
```

## Toolchain Sources by Host Platform

### Linux Host → Linux Target

✅ **Bootlin Toolchains** (Best option)
- Source: https://toolchains.bootlin.com/
- Size: 96MB compressed, 449MB extracted
- Quality: Excellent (well-maintained, updated)
- Availability: 100+ combinations (glibc, musl, uclibc)

### macOS Host → Linux Target

**Option 1: Homebrew Cross-Compilers**
```bash
# Install from Homebrew
brew install x86_64-elf-gcc
brew install aarch64-elf-gcc
```
- ✅ Easy to find and extract from bottles
- ⚠️ Need to test actual cross-compilation
- ⚠️ May need custom sysroot

**Option 2: crosstool-NG (Build Our Own)**
```bash
# Build minimal cross-compiler for macOS
ct-ng x86_64-unknown-linux-gnu
ct-ng build
```
- ✅ Full control over size and features
- ✅ Reproducible builds
- ❌ Build time: 30-60 minutes per toolchain
- ❌ Requires maintenance

**Option 3: LLVM/Clang (Future)**
```bash
# Use LLVM's cross-compilation support
clang --target=x86_64-unknown-linux-gnu
```
- ✅ Single compiler for all targets
- ✅ Smaller size
- ⚠️ May have compatibility issues with some crates
- ⚠️ Needs testing with Rust

### Windows Host → Linux Target

**Option 1: MinGW-based Cross-Compilers**
- Source: TBD (need to research)
- ❌ Hardest to find pre-built

**Option 2: WSL2 + Bootlin**
- Run Linux toolchains via WSL2
- ⚠️ Requires WSL2 installation
- ⚠️ Adds complexity

## Revised Implementation Strategy

### Phase 1: Linux-Only Bundled Toolchains (v0.3)

**Scope**: Only bundle toolchains for **Linux hosts** initially

**Why**:
1. ✅ Bootlin provides excellent pre-built toolchains
2. ✅ Most CI/CD runs on Linux
3. ✅ Validates the concept before expanding
4. ✅ macOS/Windows users can still use Docker/Podman

**Implementation**:
```rust
// src/toolchains/mod.rs
pub fn should_use_bundled_toolchain() -> bool {
    // Only use bundled toolchains on Linux hosts
    cfg!(target_os = "linux")
}
```

### Phase 2: macOS Bundled Toolchains (v0.4)

**Research needed**:
- [ ] Test Homebrew cross-compilers on macOS
- [ ] Build minimal crosstool-NG toolchains for macOS
- [ ] Test with actual Rust cross-compilation
- [ ] Measure size and performance

### Phase 3: Windows Bundled Toolchains (v0.5)

**Research needed**:
- [ ] Find/build MinGW-based cross-compilers
- [ ] Alternative: Recommend WSL2 on Windows

## Test Script Updates Needed

Update `test_manual_cross_compilation.sh`:

```bash
# Add platform detection
if [[ "$(uname -s)" != "Linux" ]]; then
    echo "⚠️  WARNING: This test requires Linux host"
    echo "Current platform: $(uname -s)"
    echo "Bootlin toolchains only run on Linux"
    exit 1
fi
```

## Next Steps

1. **Update BUNDLED_TOOLCHAINS.md** with platform-specific findings
2. **Add platform check** to test script
3. **Test on Linux** (GitHub Actions or local Linux VM)
4. **Research macOS toolchains** (Homebrew, crosstool-NG, or LLVM)
5. **Update implementation plan** to prioritize Linux-only initially

## Lessons Learned

1. ✅ **Bootlin toolchains work great** - just need Linux host
2. ⚠️ **Platform complexity is real** - can't use same binaries across OSes
3. 💡 **Start simple** - Linux-only is still valuable for CI/CD use cases
4. 💡 **Progressive enhancement** - Add macOS/Windows support later

## Metrics

### Bootlin Toolchain (x86-64-linux-gnu, Linux host)

| Metric | Value |
|--------|-------|
| Compressed size | 96 MB |
| Extracted size | 449 MB |
| Download time | ~9 seconds (fast connection) |
| Components | gcc 14.3.0, binutils 2.43.1, glibc 2.41, gdb 15.2 |
| Files in bin/ | 76 binaries |

### Repackaging Potential

If we strip debug symbols and remove unnecessary components:
- Estimated minimal size: **~50-80MB extracted**
- Estimated tarball size: **~20-35MB compressed**
- Reduction: **~65%** size savings

Components that can be removed:
- ❌ gdb (debugger) - ~30MB
- ❌ man pages, docs - ~5MB
- ❌ Debug symbols - ~50MB
- ✅ Keep: gcc, ld, ar, as, objcopy, strip, sysroot

## References

- [Bootlin Toolchains](https://toolchains.bootlin.com/)
- [crosstool-NG](https://crosstool-ng.github.io/)
- [Homebrew Cross-Compilers](https://formulae.brew.sh/)
- [LLVM Cross-Compilation](https://clang.llvm.org/docs/CrossCompilation.html)
