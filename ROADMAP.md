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
| Test Coverage | 🚧 In Progress | 62% (target: 80%) |
| Progress Bars | ✅ Complete | Build status with indicatif |

### Remaining Work

**Test Coverage** (P0 - Critical)
- Current: 62% coverage (1,170/1,887 lines)
- Target: 80% coverage (~340 more lines)
- Focus: CLI command paths, edge cases

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

### Distribution Automation (cargo-dist)
- **Self-Hosting Builds** - xcargo builds xcargo for all platforms (dogfooding)
- **Automated Installers** - Shell script, PowerShell, Homebrew tap
- **GitHub Releases Integration** - Automatic binary uploads and release notes
- **Package Manager Support** - Homebrew, Scoop, APT repositories
- **One-Command Install** - `curl https://xcargo.sh | sh`

**Implementation Plan:**
1. Basic Setup (2 hours)
   - Add `[workspace.metadata.dist]` configuration
   - Run `cargo dist init` to generate workflows
   - Test release process on feature branch

2. Self-Hosting (4 hours)
   - Configure workflow to use xcargo for cross-compilation
   - Replace default cross/zig with `xcargo build --all --parallel`
   - Validate builds on all target platforms

3. Enhanced Distribution (1 day)
   - Set up Homebrew tap publishing
   - Create custom installers with system checks
   - Test installers on clean VMs (Linux, macOS, Windows)

4. Production Polish (2 days)
   - Integrate with package managers (apt, scoop)
   - Add offline installation support
   - Documentation for all installation methods

**Effort:** 3-4 days | **Impact:** High - Professional distribution channel

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
