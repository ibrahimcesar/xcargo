# Security Policy

## Supported Versions

We actively support the following versions of xcargo with security updates:

| Version | Supported          | Notes |
| ------- | ------------------ | ----- |
| 1.0.x   | :white_check_mark: | Latest stable release |
| 0.3.x   | :x:                | Upgrade to 1.0.x |
| < 0.3   | :x:                | Upgrade to 1.0.x |

**Recommendation:** Always use the latest stable version for best security and features.

---

## Reporting a Vulnerability

**We take security seriously.** If you discover a security vulnerability in xcargo, please report it responsibly.

### Reporting Process

**DO NOT** create a public GitHub issue for security vulnerabilities.

Instead, please use one of these methods:

#### Option 1: GitHub Security Advisories (Preferred)

1. Go to https://github.com/ibrahimcesar/xcargo/security/advisories
2. Click "New draft security advisory"
3. Provide detailed information about the vulnerability

#### Option 2: Email

Send an email to: **security@xcargo.dev** (or ibrahim.cesar@gmail.com)

**Subject line:** `[SECURITY] Brief description`

#### Option 3: Private Vulnerability Disclosure

Use GitHub's private vulnerability reporting feature.

---

## What to Include

Please include as much information as possible:

- **Type of vulnerability** (command injection, path traversal, etc.)
- **Affected version(s)**
- **Steps to reproduce**
- **Proof of concept** (if available)
- **Impact assessment** (what can an attacker do?)
- **Suggested fix** (if you have one)

### Example Report

```markdown
**Vulnerability Type:** Command injection
**Affected Versions:** 1.0.0
**Severity:** High

**Description:**
User-controlled input in the --target flag is not properly sanitized,
allowing arbitrary command execution.

**Steps to Reproduce:**
1. Run: xcargo build --target "x86_64-unknown-linux-gnu; malicious-command"
2. Observe malicious-command executing

**Impact:**
An attacker who can control build commands could execute arbitrary code.

**Suggested Fix:**
Properly validate target triple format before passing to cargo.
```

---

## Response Timeline

We are committed to addressing security issues promptly:

| Timeline | Action |
|----------|--------|
| **24-48 hours** | Acknowledge receipt of report |
| **7 days** | Provide initial assessment and expected timeline |
| **30 days** | Provide fix or mitigation (target) |
| **45 days** | Public disclosure (after patch is released) |

**Note:** Complex vulnerabilities may require more time. We will keep you informed.

---

## Security Update Process

When a security vulnerability is confirmed:

1. **Patch Development** - We develop and test a fix
2. **Security Advisory** - We create a GitHub Security Advisory
3. **Patch Release** - We release a new version with the fix
4. **Public Disclosure** - We publicly disclose the vulnerability (coordinated)
5. **CVE Assignment** - We request a CVE if applicable

### Notification Channels

Security updates are announced via:

- GitHub Security Advisories
- GitHub Releases (with security tag)
- README notice (for critical issues)
- Discussions board

---

## Security Best Practices for Users

### General Recommendations

1. **Keep xcargo updated** - Always use the latest stable version
2. **Review configurations** - Audit your xcargo.toml files
3. **Verify downloads** - Check SHA256 checksums of binaries
4. **Use official sources** - Download only from GitHub releases or official installers

### Container Security

If using container-based builds:

1. **Verify images** - Use official Rust images
2. **Pin image hashes** - Don't rely on :latest tags
3. **Keep Docker/Podman updated** - Ensure runtime is current
4. **Use rootless containers** - When possible

```toml
# Example: Pin to specific image hash
[container]
image = "rust:1.70@sha256:abc123..."
```

### Dependency Security

1. **Audit dependencies** - Run `cargo audit` regularly
2. **Lock dependencies** - Commit Cargo.lock
3. **Review updates** - Check changelogs before updating

### Environment Security

1. **Trust your PATH** - Ensure no malicious tools in PATH
2. **Verify toolchains** - Use official rustup installations
3. **Protect credentials** - Keep cargo tokens secure

---

## Scope

### In Scope

Security issues in:

- ✅ xcargo binary and Rust code
- ✅ Build process and command execution
- ✅ Configuration parsing
- ✅ Integration with cargo/rustup/zig
- ✅ Container runtime integration
- ✅ Installation scripts

### Out of Scope

The following are **not** considered xcargo vulnerabilities:

- ❌ Vulnerabilities in cargo itself (report to Rust)
- ❌ Vulnerabilities in rustc/rustup (report to Rust)
- ❌ Vulnerabilities in Docker/Podman (report to those projects)
- ❌ User-introduced vulnerabilities in their code
- ❌ Vulnerabilities in user-specified container images
- ❌ System configuration issues

**Why?** xcargo is a build orchestration tool. We can't fix vulnerabilities in upstream tools, but we can help you use them securely.

---

## Security Features

xcargo includes these security features:

- ✅ **Memory safety** - Written in Rust (no buffer overflows)
- ✅ **Input validation** - Target triples and configs are validated
- ✅ **Safe command execution** - No shell interpretation
- ✅ **Least privilege** - Runs as regular user (no sudo)
- ✅ **Dependency pinning** - Cargo.lock ensures reproducibility
- ✅ **Error handling** - No information leakage in errors

---

## Known Limitations

### Trust Model

xcargo trusts:

- User's xcargo.toml configuration
- User's Cargo.toml and dependencies
- System-installed toolchains (rustc, cargo, rustup)
- Container images specified by user

**Justification:** As a build tool, xcargo must trust the user's build environment. This is the same trust model as cargo, make, npm, etc.

### Container Security

When using container builds, security depends on:

- Docker/Podman runtime security
- Container image integrity
- Host system security

**User responsibility:** Verify container images and keep runtimes updated.

---

## Security Audits

xcargo undergoes regular security reviews:

- **Self-audits:** Before each major release
- **Community reviews:** Via public GitHub repository
- **Static analysis:** Clippy with security lints
- **Dependency audits:** cargo-audit integration

**Last audit:** 2025-11-23 (v1.0.0 launch)
**Next planned audit:** v1.1.0 or 6 months (whichever comes first)

---

## Hall of Fame

We recognize security researchers who responsibly disclose vulnerabilities:

*Currently empty - be the first!*

**Eligibility:** Researchers who report valid security issues will be listed here (with permission).

---

## Legal

### Safe Harbor

We support security research conducted in good faith. We will not pursue legal action against researchers who:

- Make a good faith effort to avoid privacy violations and service disruption
- Report vulnerabilities responsibly and privately
- Give us reasonable time to fix issues before public disclosure

### Responsible Disclosure

We request that you:

- Do not publicly disclose the vulnerability until we've had time to fix it
- Do not exploit the vulnerability beyond what's necessary to demonstrate it
- Do not access data that doesn't belong to you

---

## Contact

- **Security issues:** security@xcargo.dev or GitHub Security Advisories
- **General questions:** https://github.com/ibrahimcesar/xcargo/discussions
- **Bugs (non-security):** https://github.com/ibrahimcesar/xcargo/issues

---

## Additional Resources

- [Security Evaluation (v1.0.0)](.github/SECURITY_EVALUATION.md)
- [Release Process](.github/RELEASE_PROCESS.md)
- [Contributing Guidelines](CONTRIBUTING.md)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)

---

**Thank you for helping keep xcargo and the Rust ecosystem secure!** 🛡️

*Last updated: 2025-11-23*
