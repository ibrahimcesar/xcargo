# CI/CD Templates for xcargo

Ready-to-use CI/CD configurations for building multi-target Rust projects with xcargo.

## Available Templates

### GitHub Actions

1. **[multi-target-build.yml](../../.github/workflows/templates/multi-target-build.yml)**
   - Matrix builds across multiple OS and targets
   - Automatic artifact uploads
   - Optional release creation
   - **Use when:** You want separate jobs per target

2. **[parallel-builds.yml](../../.github/workflows/templates/parallel-builds.yml)**
   - Single job with parallel target builds
   - Uses xcargo's parallel build feature
   - Faster total time, less resource usage
   - **Use when:** You want fast builds with xcargo's built-in parallelism

### GitLab CI

1. **[gitlab-ci.yml](gitlab-ci.yml)**
   - Per-target build jobs
   - Parallel build option
   - Cargo caching
   - Artifact management
   - **Use when:** Using GitLab CI/CD

## Quick Start

### GitHub Actions

1. Copy template to your repository:
   ```bash
   mkdir -p .github/workflows
   cp .github/workflows/templates/multi-target-build.yml .github/workflows/build.yml
   ```

2. Customize for your project:
   ```yaml
   # Edit matrix.include to add/remove targets
   matrix:
     include:
       - os: ubuntu-latest
         target: x86_64-unknown-linux-gnu
       # Add more targets...
   ```

3. Commit and push:
   ```bash
   git add .github/workflows/build.yml
   git commit -m "ci: add multi-target build workflow"
   git push
   ```

### GitLab CI

1. Copy template to repository root:
   ```bash
   cp docs/examples/ci/gitlab-ci.yml .gitlab-ci.yml
   ```

2. Customize jobs:
   ```yaml
   # Edit build jobs or use parallel build
   build:parallel:
     script:
       - xcargo build --target linux,windows,macos
   ```

3. Commit and push:
   ```bash
   git add .gitlab-ci.yml
   git commit -m "ci: add GitLab CI configuration"
   git push
   ```

## Target Selection Guide

### Common Target Combinations

**Server/CLI applications:**
```yaml
targets:
  - x86_64-unknown-linux-gnu      # Linux glibc
  - x86_64-unknown-linux-musl     # Linux static
  - x86_64-apple-darwin           # macOS Intel
  - aarch64-apple-darwin          # macOS ARM
  - x86_64-pc-windows-msvc        # Windows
```

**Embedded/IoT:**
```yaml
targets:
  - armv7-unknown-linux-gnueabihf # Raspberry Pi
  - aarch64-unknown-linux-gnu     # ARM64 Linux
  - thumbv7em-none-eabihf         # Cortex-M4/M7
```

**Mobile:**
```yaml
targets:
  - aarch64-apple-ios             # iPhone/iPad
  - aarch64-linux-android         # Android ARM64
  - armv7-linux-androideabi       # Android ARMv7
```

**WebAssembly:**
```yaml
targets:
  - wasm32-unknown-unknown        # Browser/WASI
  - wasm32-wasi                   # WASI runtime
```

## Performance Comparison

### Matrix Builds (Separate Jobs)

**Pros:**
- Failures isolated (one target fails, others continue)
- Clear logs per target
- Can run on different OS runners

**Cons:**
- Higher total wall time
- More CI minutes used
- Sequential by default

**Example timing (5 targets):**
```
Total wall time: ~15 minutes (3 min per target, parallel)
CI minutes used: 15 minutes (5 targets × 3 min each)
```

### Parallel Builds (Single Job)

**Pros:**
- Faster total wall time
- Lower CI minutes usage
- xcargo optimizes parallelism

**Cons:**
- One target failure fails entire job
- Mixed logs
- Must run on Linux (for cross-compilation)

**Example timing (5 targets):**
```
Total wall time: ~8 minutes (xcargo parallel build)
CI minutes used: 8 minutes (single job)
```

**Recommendation:** Use parallel builds for PRs (fast feedback), matrix builds for releases (better isolation).

## Cross-Compilation Setup

### Zig (Recommended for Linux targets)

```yaml
- name: Install Zig
  run: |
    wget https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz
    tar -xf zig-linux-x86_64-0.11.0.tar.xz
    sudo mv zig-linux-x86_64-0.11.0/zig /usr/local/bin/
    zig version

- name: Build with Zig
  run: xcargo build --target x86_64-unknown-linux-musl
  # xcargo auto-detects Zig and uses it
```

**Advantages:**
- No additional packages needed
- Works on macOS, Linux, Windows hosts
- Handles musl static linking automatically

### Native Toolchains

```yaml
- name: Install cross-compilation tools
  run: |
    sudo apt-get update
    sudo apt-get install -y \
      gcc-aarch64-linux-gnu \
      gcc-arm-linux-gnueabihf \
      mingw-w64

- name: Add Rust targets
  run: |
    rustup target add aarch64-unknown-linux-gnu
    rustup target add armv7-unknown-linux-gnueabihf
    rustup target add x86_64-pc-windows-gnu
```

### Container-Based (Fallback)

If Zig and native toolchains aren't available:

```yaml
- name: Setup Docker
  uses: docker/setup-buildx-action@v3

- name: Build with containers
  run: xcargo build --target aarch64-unknown-linux-gnu
  # xcargo auto-uses containers when needed
```

## Caching Strategies

### GitHub Actions

```yaml
- name: Cache cargo registry
  uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

- name: Cache build artifacts
  uses: actions/cache@v3
  with:
    path: target
    key: ${{ runner.os }}-target-${{ hashFiles('**/Cargo.lock') }}
```

### GitLab CI

```yaml
.cargo_cache:
  cache:
    key: ${CI_COMMIT_REF_SLUG}
    paths:
      - .cargo/
      - target/
    policy: pull-push
```

## Testing Cross-Compiled Binaries

### Native Targets (Can Run Tests)

```yaml
- name: Run tests
  if: matrix.target == 'x86_64-unknown-linux-gnu'
  run: xcargo test --target ${{ matrix.target }}
```

### Cross-Compiled Targets (Skip Tests)

```yaml
- name: Build only (cross-compiled)
  if: matrix.target != host_triple
  run: xcargo build --release --target ${{ matrix.target }}
```

### Using QEMU for ARM Testing

```yaml
- name: Setup QEMU
  uses: docker/setup-qemu-action@v3

- name: Test ARM binary
  run: |
    sudo apt-get install -y qemu-user-static
    qemu-aarch64-static target/aarch64-unknown-linux-gnu/release/myapp --version
```

## Release Automation

### Trigger on Tags

```yaml
on:
  push:
    tags:
      - 'v*'
```

### Create GitHub Release

```yaml
release:
  needs: build
  if: startsWith(github.ref, 'refs/tags/v')
  steps:
    - uses: softprops/action-gh-release@v1
      with:
        files: artifacts/**/*
```

### Upload to Package Registries

**crates.io:**
```yaml
- name: Publish to crates.io
  run: cargo publish
  env:
    CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_TOKEN }}
```

**Homebrew:**
```yaml
- name: Update Homebrew tap
  run: |
    # See homebrew-tap repository workflow
    # Usually automated via cargo-dist
```

## Troubleshooting

### Job Fails: "xcargo: command not found"

**Cause:** xcargo not added to PATH

**Fix:**
```yaml
- name: Install xcargo
  run: |
    curl --proto '=https' --tlsv1.2 -LsSf \
      https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-installer.sh | sh
    echo "$HOME/.cargo/bin" >> $GITHUB_PATH  # Important!
```

### Build Fails: "linker not found"

**Cause:** Missing cross-compilation toolchain

**Fix:** Install appropriate toolchain or use Zig:
```yaml
- name: Install Zig
  run: |
    brew install zig  # macOS
    # or
    apt-get install -y zig  # Linux (if available)
```

### Windows Build Fails on Linux

**Cause:** mingw-w64 not installed

**Fix:**
```yaml
- name: Install MinGW
  run: sudo apt-get install -y mingw-w64
```

### Parallel Build Runs Out of Memory

**Cause:** Too many targets for available RAM

**Fix:** Reduce parallelism or use matrix builds:
```yaml
# Option 1: Reduce targets per job
- run: xcargo build --target linux,windows  # Fewer targets

# Option 2: Use matrix instead
strategy:
  matrix:
    target: [linux-gnu, linux-musl, windows, macos]
```

## Best Practices

1. **Cache dependencies** - Speeds up builds dramatically
2. **Use Zig for Linux targets** - Simpler than installing toolchains
3. **Matrix for releases** - Better error isolation
4. **Parallel for PRs** - Faster feedback
5. **Skip tests for cross-compiled** - They can't run
6. **Verify with `xcargo doctor`** - Check setup before building
7. **Upload artifacts** - Debug build issues
8. **Use latest xcargo** - Get bug fixes and improvements

## Example Projects

See real-world examples:

- **xcargo itself:** [.github/workflows/release.yml](../../.github/workflows/release.yml)
- **Self-hosting test:** [.github/workflows/self-host-test.yml](../../.github/workflows/self-host-test.yml)

## Getting Help

- **Documentation:** https://ibrahimcesar.github.io/xcargo
- **Issues:** https://github.com/ibrahimcesar/xcargo/issues
- **Discussions:** https://github.com/ibrahimcesar/xcargo/discussions

---

**Templates maintained by:** xcargo maintainers
**Last updated:** 2025-11-23
