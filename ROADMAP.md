# xcargo Roadmap

**Vision:** Make Rust cross-compilation zero-friction. Just works. ✨

**Current Version:** v0.3.0
**Next Release:** v1.0.0 (Production Ready)

---

## 🎯 v1.0.0 - Production Release

Our goal is to deliver a stable, well-documented, battle-tested cross-compilation tool.

### Status: 85% Complete

| Feature Area | Status | Notes |
|--------------|--------|-------|
| Core Build System | ✅ Complete | Native, Zig, container builds |
| Parallel Builds | ✅ Complete | Async multi-target support |
| Configuration | ✅ Complete | Interactive setup, per-target config |
| CLI Commands | ✅ Complete | build, check, test, init, list, doctor |
| Error Handling | ✅ Complete | Structured errors, helpful suggestions |
| Documentation | ✅ Complete | Guides, API docs, troubleshooting |
| CI/CD Testing | ✅ Complete | Cross-platform matrix, Zig & containers |
| Test Coverage | 🚧 In Progress | 68.20% (target: 80%) |
| Progress Bars | ✅ Complete | Build status with indicatif |
| Binary Distribution | ✅ Complete | cargo-dist with 5 platforms, installers |

### Remaining Work

**Test Coverage** (P0 - Critical)
- Current: 68.20% coverage (1,287/1,887 lines)
- Target: 80% coverage (~220 more lines, 25-30 tests)
- Focus: src/main.rs (47%), src/target/mod.rs (58%), src/doctor/report.rs (77%)

**Recent Progress (2025-11-23):**
- ✅ Added cargo-dist integration (Phase 1)
- ✅ Configured 5-platform binary distribution
- ✅ Set up automated installers (shell, PowerShell, Homebrew)
- ✅ Added 30 integration and CLI tests (+2.55% coverage)
- ✅ Added 56 execution path tests for linker/Zig/container/config (+3.66% coverage)

---

## 🚀 Post-1.0 Features

### Build Performance
- **Build Caching** - Hash-based incremental builds
- **Artifact Tracking** - Skip unchanged targets
- **Clean Command** - Target-specific cleanup

### Container Enhancements
- **Custom Dockerfiles** - Project-specific images
- **Volume Caching** - Faster cargo registry access
- **Smart Image Selection** - Auto-select optimal images
- **Podman Machine Support** - Better macOS integration

### CI/CD Integration
- **GitHub Action** - `uses: xcargo/action@v1`
- **GitLab CI Template** - `.gitlab-ci.yml` examples
- **Matrix Builds** - Multi-target automation
- **Release Command** - `xcargo release` with changelog

### Distribution Automation (cargo-dist) ✅ Phase 1 Complete

**Status:** Basic setup complete (2025-11-23)

**Implemented:**
- ✅ cargo-dist v0.30.2 integrated
- ✅ 5-platform builds: Linux (GNU/musl), macOS (x86_64/ARM64), Windows (MSVC)
- ✅ Automated installers: Shell script, PowerShell, Homebrew formula
- ✅ GitHub Releases integration with automatic binary uploads
- ✅ Professional release workflow (.github/workflows/release.yml)

**Installation Methods Available:**
```bash
# Shell installer (Linux/macOS)
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-installer.sh | sh

# PowerShell (Windows)
powershell -c "irm https://github.com/ibrahimcesar/xcargo/releases/latest/download/xcargo-installer.ps1 | iex"

# Homebrew (macOS)
brew install xcargo
```

**Remaining Phases (Post-v1.0):**

2. **Self-Hosting** (4 hours) - Deferred to v0.5.0
   - Configure workflow to use xcargo for cross-compilation
   - xcargo builds xcargo (dogfooding demonstration)

3. **Enhanced Distribution** (1 day) - v1.1.0
   - Homebrew tap publishing automation
   - Custom installers with system checks
   - VM testing across all platforms

4. **Production Polish** (2 days) - v1.2.0
   - Package managers: APT, Scoop, AUR
   - Offline installation support
   - Comprehensive installation documentation

**Total Effort:** Phase 1: 30 min (completed) | Remaining: 3-4 days
**Impact:** ⭐⭐⭐⭐⭐ High - Professional distribution lowers barrier to entry

### Developer Experience
- **Build Profiles** - Predefined target groups (mobile, server, minimal)
- **TUI Interface** - Interactive target selection with ratatui
- **Bundle Toolchains** - On-demand toolchain downloads
- **Telemetry** - Opt-in usage analytics for improvements

---

## 🎨 Long-Term Vision

### Ecosystem
- Plugin marketplace/registry
- Community target configurations repository
- Custom builder framework
- cargo-dist for professional distribution (see post-1.0 section above)

### Platform Support
- Improved Windows native support (beyond WSL2)
- Emulator-based cross-testing
- Mobile platform optimizations (iOS, Android)
- Embedded target helpers

---

## 📈 Success Metrics

### v1.0.0 Launch
- ✅ 80%+ test coverage
- ✅ Zero panics in production code
- ✅ Comprehensive documentation
- ✅ CI/CD examples for GitHub Actions
- ⏳ Published to crates.io
- ⏳ Homebrew formula available

### Community Growth (6 months post-1.0)
- 500+ GitHub stars
- 10+ production users
- Active Discord/discussions
- 5+ external contributors

---

## 🔄 Release Cadence

**Stable Releases (v1.x):**
- Major releases: Quarterly
- Minor releases: Monthly
- Patch releases: As needed

**Pre-1.0:**
- v0.4.0 - Test coverage complete
- v1.0.0 - Production ready

---

## 🤝 How to Contribute

We welcome contributions! Areas where help is needed:

1. **Testing** - Help us reach 80% coverage
2. **Documentation** - Real-world usage examples
3. **Target Support** - Test exotic targets
4. **Container Images** - Optimize cross images
5. **CI Templates** - GitLab, CircleCI, Azure Pipelines

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## 📚 More Information

- [README.md](README.md) - Getting started guide
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [CHANGELOG.md](CHANGELOG.md) - Release history
- [Documentation](https://ibrahimcesar.github.io/xcargo) - Full docs site

---

*Last Updated: 2025-11-23*
