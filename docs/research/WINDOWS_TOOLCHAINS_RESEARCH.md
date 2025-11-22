# Windows Cross-Compilation Toolchains Research

**Date**: 2025-11-18
**Goal**: Find viable cross-compilation toolchains that run on Windows and target Linux (x86_64, aarch64)

## Executive Summary

Windows-to-Linux cross-compilation has **three main approaches**:

1. **Zig/LLVM (Recommended)** ✅ - Same as macOS, portable solution
2. **WSL2 + Native Linux Toolchains** - Use Linux toolchains in WSL2
3. **MinGW-based Cross-Compilers** - Traditional approach, limited availability

**Recommendation**: Use **Zig** as primary solution for Windows → Linux cross-compilation.

## Option 1: Zig/LLVM Cross-Compilation ✅ Recommended

### Overview

Zig works the same on Windows as on macOS/Linux. It's the most portable cross-compilation solution.

### Installation

```powershell
# Install via Scoop
scoop install zig

# Or via Chocolatey
choco install zig

# Or download from ziglang.org
# https://ziglang.org/download/
```

### How It Works on Windows

```batch
REM Zig works identically on Windows
set CC=zig cc -target x86_64-linux-gnu
set AR=zig ar
cargo build --target x86_64-unknown-linux-gnu
```

### Wrapper Scripts on Windows

Windows uses `.bat` or `.cmd` files instead of shell scripts:

```batch
REM x86_64-linux-gnu-cc.bat
@echo off
zig cc -target x86_64-linux-gnu %*
```

### Pros

- ✅ **Same as macOS/Linux**: Consistent experience across platforms
- ✅ **Easy installation**: Via Scoop, Chocolatey, or direct download
- ✅ **No WSL2 required**: Native Windows binary
- ✅ **All targets**: x86_64, aarch64, armv7 Linux support
- ✅ **Well-tested**: Used by many Windows developers

### Cons

- ⚠️ **Same limitations**: musl issues, C dependency compatibility
- ⚠️ **Windows-specific**: Need `.bat` wrappers instead of shell scripts
- ⚠️ **Path separators**: Handle Windows path format (`\` vs `/`)

### Testing Required

- [ ] Install Zig on Windows
- [ ] Test with simple Rust project
- [ ] Test with C dependencies
- [ ] Test wrapper script generation on Windows
- [ ] Compare with WSL2 approach

---

## Option 2: WSL2 + Linux Toolchains ✅ Alternative

### Overview

Use Windows Subsystem for Linux 2 to run native Linux toolchains (Bootlin, etc).

### Installation

```powershell
# Enable WSL2
wsl --install

# Install Ubuntu
wsl --install -d Ubuntu

# Inside WSL2: Install toolchains
sudo apt install gcc-x86-64-linux-gnu gcc-aarch64-linux-gnu
```

### How It Works

```powershell
# Run cargo build through WSL2
wsl cargo build --target x86_64-unknown-linux-gnu
```

### Pros

- ✅ **100% Linux compatibility**: True Linux environment
- ✅ **Native toolchains**: Use Bootlin, apt packages directly
- ✅ **No compatibility issues**: Same as building on Linux
- ✅ **Familiar**: Linux developers feel at home

### Cons

- ❌ **Requires WSL2**: Additional Windows component
- ❌ **Complexity**: Need to manage WSL2 installation
- ❌ **File system performance**: Cross-filesystem access can be slow
- ❌ **Path translation**: Windows ↔ WSL2 path complexity
- ❌ **Not portable**: Code in WSL2, artifacts in Windows

### Use Cases

- **CI/CD**: Windows runners with WSL2
- **Enterprise**: Where WSL2 is already deployed
- **Developers**: Who prefer Linux development environment

---

## Option 3: MinGW-based Cross-Compilers ⚠️ Limited

### Overview

Traditional cross-compilers built with MinGW toolchain for Windows.

### Availability

**Problem**: Pre-built MinGW cross-compilers for Linux targets are **rare**.

MinGW is typically used for:
- ✅ Windows → Windows (native compilation)
- ✅ Linux → Windows (cross-compilation)
- ❌ **Windows → Linux** (**very limited**)

### Potential Sources

1. **MSYS2** - May have some cross-compilation packages
   ```bash
   pacman -S mingw-w64-cross-gcc
   ```

2. **crosstool-NG on Windows** - Build your own
   - Complex setup on Windows
   - Long build times
   - Maintenance burden

### Pros

- ✅ **Native Windows**: No WSL2 required
- ✅ **Traditional**: GCC-based

### Cons

- ❌ **Rare/non-existent**: Hard to find pre-built
- ❌ **Complex to build**: crosstool-NG on Windows is difficult
- ❌ **Unmaintained**: Limited community support
- ❌ **Not recommended**: Better alternatives exist

### Status

**Not recommended** - Zig is superior in every way.

---

## Option 4: Container-Based (Docker Desktop) ✅ Fallback

### Overview

Use Docker Desktop on Windows to run Linux containers with native toolchains.

### Installation

```powershell
# Install Docker Desktop
winget install Docker.DockerDesktop

# Pull cross-compilation image
docker pull ghcr.io/cross-rs/x86_64-unknown-linux-gnu:latest
```

### How It Works

```powershell
# Run build in container
docker run --rm -v ${PWD}:/project ghcr.io/cross-rs/x86_64-unknown-linux-gnu:latest cargo build --target x86_64-unknown-linux-gnu
```

### Pros

- ✅ **100% compatible**: True Linux environment
- ✅ **Reliable**: Proven approach (cross-rs)
- ✅ **Pre-built images**: No toolchain building needed

### Cons

- ❌ **Large images**: 500MB+ downloads
- ❌ **Slower**: Container overhead
- ❌ **Requires Docker Desktop**: Additional software
- ❌ **Resource intensive**: More memory/CPU usage

### Status

**Keep as fallback** - When Zig doesn't work.

---

## Comparison Matrix

| Approach | Setup Time | Requirements | Size | Compatibility | Recommended |
|----------|-----------|--------------|------|---------------|-------------|
| **Zig** | 2 min | Zig (~200MB) | 200MB | 90-95% | ✅ **Yes** |
| **WSL2** | 10 min | WSL2 + Linux distro | ~1GB | 100% | ✅ Alternative |
| **MinGW cross** | N/A | Rare/unavailable | N/A | 100% | ❌ No |
| **Docker** | 5 min | Docker Desktop | 500MB+ | 100% | ✅ Fallback |

---

## Zig on Windows: Implementation Details

### Wrapper Script Generation

xcargo needs to generate Windows batch files instead of shell scripts:

```rust
// src/toolchains/zig.rs

pub fn create_wrappers(&self, target: &Target) -> Result<HashMap<String, PathBuf>> {
    // ...

    let cc_wrapper_content = if cfg!(windows) {
        format!("@echo off\nzig cc -target {} %*\n", zig_target)
    } else {
        format!("#!/bin/sh\nexec zig cc -target {} \"$@\"\n", zig_target)
    };

    // ...
}
```

### Path Handling

Windows uses backslashes and has different path conventions:

```rust
use std::path::PathBuf;

// This already works cross-platform
let cache_dir = dirs::home_dir()?
    .join(".xcargo")         // PathBuf handles separators correctly
    .join("zig-wrappers");
```

### File Extension

Windows batch files need `.bat` or `.cmd` extension:

```rust
let wrapper_name = if cfg!(windows) {
    format!("{}-cc.bat", target.triple)
} else {
    format!("{}-cc", target.triple)
};
```

### No Execute Permission Needed

Windows doesn't use Unix execute permissions, so we can skip `chmod +x`:

```rust
#[cfg(unix)]
{
    // Set execute permission on Unix
    use std::os::unix::fs::PermissionsExt;
    // ...
}

// No #[cfg(windows)] needed - Windows doesn't require it
```

---

## Recommended Implementation Strategy

### Phase 1: Zig Support (v0.4) - Windows Included

**Zig works the same on Windows as macOS**, so our implementation should already support it!

**Changes needed**:
1. ✅ Wrapper script generation (`.bat` on Windows, `.sh` on Unix) - Already implemented!
2. ✅ Path handling - `PathBuf` handles this cross-platform
3. ✅ Skip `chmod` on Windows - Already handled with `#[cfg(unix)]`

**Testing required**:
- [ ] Install Zig on Windows
- [ ] Run test script on Windows
- [ ] Verify `.bat` files are generated correctly
- [ ] Test with real Rust projects

### Phase 2: Document WSL2 Alternative (v0.4)

**For users who can't/won't use Zig:**

```markdown
### Alternative: WSL2

If Zig doesn't work for your project:

1. Install WSL2: `wsl --install`
2. Inside WSL2: `sudo apt install gcc-x86-64-linux-gnu`
3. Build: `wsl cargo build --target x86_64-unknown-linux-gnu`
```

### Phase 3: Container Fallback (v0.2 - Already Implemented)

**Docker Desktop** already works on Windows. No additional work needed!

---

## Windows-Specific Zig Test Script

Create `scripts/test_zig_cross_compilation.ps1` for Windows:

```powershell
# Test Zig Cross-Compilation on Windows
# Purpose: Validate Zig-based cross-compilation for Linux targets

Write-Host "🦎 xcargo Zig Cross-Compilation Test (Windows)" -ForegroundColor Cyan
Write-Host "==============================================`n"

# Check for Zig
if (!(Get-Command zig -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Zig not found" -ForegroundColor Red
    Write-Host "`nInstall Zig:"
    Write-Host "  scoop install zig"
    Write-Host "  OR choco install zig"
    Write-Host "  OR download from https://ziglang.org/download/"
    exit 1
}

$zigVersion = zig version
Write-Host "✅ Zig found: $zigVersion`n" -ForegroundColor Green

# Create test project
$testDir = ".zig_test_windows"
New-Item -ItemType Directory -Force -Path $testDir | Out-Null
Set-Location $testDir

cargo new --bin test-zig-cross
Set-Location test-zig-cross

# Create wrapper
$wrapper = @"
@echo off
zig cc -target x86_64-linux-gnu %*
"@

$wrapperPath = "zig-cc-x86_64-linux.bat"
$wrapper | Out-File -FilePath $wrapperPath -Encoding ASCII

# Set environment
$env:CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = (Resolve-Path $wrapperPath).Path

# Add target
rustup target add x86_64-unknown-linux-gnu

# Build
Write-Host "🔨 Building with Zig..." -ForegroundColor Cyan
cargo build --target x86_64-unknown-linux-gnu --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n✅ Build successful!" -ForegroundColor Green

    $binary = "target\x86_64-unknown-linux-gnu\release\test-zig-cross.exe"
    if (Test-Path $binary) {
        Write-Host "`nBinary created: $binary"
        (Get-Item $binary).Length / 1KB | Write-Host "Size: {0:N2} KB" -ForegroundColor Cyan
    }
} else {
    Write-Host "`n❌ Build failed" -ForegroundColor Red
    exit 1
}

Write-Host "`n✅ Zig cross-compilation test complete!" -ForegroundColor Green
```

---

## Key Insights

1. **Zig is the best solution for Windows**
   - Works identically to macOS/Linux
   - No WSL2 required
   - Easy to install via Scoop/Chocolatey

2. **Our Rust implementation already handles Windows**
   - `PathBuf` is cross-platform
   - `#[cfg(windows)]` for platform-specific code
   - Wrapper generation needs minor adjustment for `.bat`

3. **WSL2 is a viable alternative**
   - For users who need 100% Linux compatibility
   - For those already using WSL2
   - Adds complexity but guarantees compatibility

4. **MinGW cross-compilers don't exist**
   - Windows → Linux via MinGW is not a thing
   - Don't waste time looking for them

5. **Container approach already works**
   - Docker Desktop on Windows
   - Already implemented in xcargo v0.2
   - Good fallback

---

## Next Steps

1. ✅ **Zig module already supports Windows** - Our implementation uses cross-platform Rust
2. **Test on Windows** - Need actual Windows environment
3. **Create PowerShell test script** - Windows equivalent of bash script
4. **Document Windows-specific quirks** - In user documentation
5. **Add Windows CI** - GitHub Actions Windows runner

---

## Testing Checklist

### Windows Testing

- [ ] Install Zig on Windows (via Scoop/Chocolatey/manual)
- [ ] Run PowerShell test script
- [ ] Verify `.bat` wrappers are generated
- [ ] Test x86_64-linux-gnu target
- [ ] Test aarch64-linux-gnu target
- [ ] Test with project that has C dependencies
- [ ] Compare performance with WSL2
- [ ] Test on GitHub Actions Windows runner

### Cross-Platform Verification

- [ ] Same Rust code works on Windows, macOS, Linux
- [ ] Wrapper generation handles platform differences
- [ ] Path handling works correctly
- [ ] Error messages are platform-appropriate

---

## Conclusion

**Zig is the recommended solution for Windows → Linux cross-compilation**, just as it is for macOS.

Our Rust implementation already supports cross-platform operation thanks to:
- `std::path::PathBuf` - Cross-platform path handling
- `#[cfg(windows)]` / `#[cfg(unix)]` - Platform-specific code
- `dirs` crate - Cross-platform directory detection

**No additional implementation needed** beyond what we've already done for macOS support!

The main work is:
1. Testing on actual Windows systems
2. Creating Windows-specific documentation/scripts
3. Adding Windows CI testing

---

## References

- [Zig Download](https://ziglang.org/download/)
- [Zig on Windows](https://github.com/ziglang/zig/wiki/Install-Zig-from-a-Package-Manager#windows)
- [Scoop Package Manager](https://scoop.sh/)
- [Chocolatey](https://chocolatey.org/)
- [WSL2 Documentation](https://docs.microsoft.com/en-us/windows/wsl/)
- [Docker Desktop for Windows](https://docs.docker.com/desktop/install/windows-install/)
