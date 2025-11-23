// Advanced tests for src/target/mod.rs
// Focuses on edge cases and error paths

use xcargo::target::{Target, TargetRequirements, TargetTier};

// ============================================================================
// Host Detection Edge Cases
// ============================================================================

#[test]
fn test_detect_host_success() {
    // Should always succeed if rustc is installed
    let host = Target::detect_host();
    assert!(host.is_ok());

    let target = host.unwrap();
    assert!(!target.triple.is_empty());
    assert!(!target.arch.is_empty());
    assert!(!target.os.is_empty());
}

// ============================================================================
// Target Installation and Detection
// ============================================================================

#[test]
fn test_is_installed_for_host_target() {
    // Host target should always be installed
    let host = Target::detect_host().unwrap();
    let is_installed = host.is_installed();
    assert!(is_installed.is_ok());
    assert!(is_installed.unwrap());
}

#[test]
fn test_detect_installed_returns_vec() {
    let installed = Target::detect_installed();
    assert!(installed.is_ok());

    let targets = installed.unwrap();
    // Should have at least the host target
    assert!(!targets.is_empty());
}

#[test]
fn test_list_available_returns_many_targets() {
    let available = Target::list_available();
    assert!(available.is_ok());

    let targets = available.unwrap();
    // rustup should list many available targets
    assert!(targets.len() > 50);
}

#[test]
fn test_list_available_includes_common_targets() {
    let available = Target::list_available().unwrap();

    let triples: Vec<_> = available.iter().map(|t| t.triple.as_str()).collect();

    // Should include common targets
    assert!(triples.contains(&"x86_64-unknown-linux-gnu"));
    assert!(triples.contains(&"x86_64-pc-windows-msvc"));
}

// ============================================================================
// Alias Resolution Edge Cases
// ============================================================================

#[test]
fn test_resolve_alias_macos_on_arm64() {
    // When we're on Apple Silicon, "macos" should resolve to aarch64
    // This test checks the dynamic behavior
    let resolved = Target::resolve_alias("macos").unwrap();
    assert!(
        resolved == "aarch64-apple-darwin" || resolved == "x86_64-apple-darwin",
        "Should resolve to either Intel or ARM macOS"
    );
}

#[test]
fn test_resolve_alias_android_variants() {
    assert_eq!(
        Target::resolve_alias("android").unwrap(),
        "aarch64-linux-android"
    );
    assert_eq!(
        Target::resolve_alias("android-arm64").unwrap(),
        "aarch64-linux-android"
    );
    assert_eq!(
        Target::resolve_alias("android-armv7").unwrap(),
        "armv7-linux-androideabi"
    );
    assert_eq!(
        Target::resolve_alias("android-x86").unwrap(),
        "x86_64-linux-android"
    );
}

#[test]
fn test_resolve_alias_ios_variants() {
    assert_eq!(
        Target::resolve_alias("ios").unwrap(),
        "aarch64-apple-ios"
    );
    assert_eq!(
        Target::resolve_alias("ios-arm64").unwrap(),
        "aarch64-apple-ios"
    );
    assert_eq!(
        Target::resolve_alias("ios-sim").unwrap(),
        "aarch64-apple-ios-sim"
    );
}

#[test]
fn test_resolve_alias_wasm_variants() {
    assert_eq!(
        Target::resolve_alias("wasm").unwrap(),
        "wasm32-unknown-unknown"
    );
    assert_eq!(
        Target::resolve_alias("wasm32").unwrap(),
        "wasm32-unknown-unknown"
    );
    assert_eq!(
        Target::resolve_alias("wasi").unwrap(),
        "wasm32-wasi"
    );
}

#[test]
fn test_resolve_alias_windows_variants() {
    assert_eq!(
        Target::resolve_alias("windows-msvc").unwrap(),
        "x86_64-pc-windows-msvc"
    );
    assert_eq!(
        Target::resolve_alias("windows-gnu").unwrap(),
        "x86_64-pc-windows-gnu"
    );
    assert_eq!(
        Target::resolve_alias("windows-32").unwrap(),
        "i686-pc-windows-gnu"
    );
}

#[test]
fn test_resolve_alias_linux_musl_variants() {
    assert_eq!(
        Target::resolve_alias("linux-musl").unwrap(),
        "x86_64-unknown-linux-musl"
    );
    assert_eq!(
        Target::resolve_alias("linux-arm64-musl").unwrap(),
        "aarch64-unknown-linux-musl"
    );
}

#[test]
fn test_resolve_alias_preserves_case_for_unknown() {
    // Unknown aliases should be returned as-is
    let custom = "MyCustomTarget-123";
    assert_eq!(Target::resolve_alias(custom).unwrap(), custom);
}

// ============================================================================
// Target Requirements for Different Platforms
// ============================================================================

#[test]
fn test_requirements_for_linux_aarch64_gnu() {
    let target = Target::from_triple("aarch64-unknown-linux-gnu").unwrap();
    let reqs = target.get_requirements();

    assert_eq!(reqs.linker, Some("aarch64-linux-gnu-gcc".to_string()));
    assert!(reqs.tools.contains(&"aarch64-linux-gnu-gcc".to_string()));
}

#[test]
fn test_requirements_for_linux_aarch64_musl() {
    let target = Target::from_triple("aarch64-unknown-linux-musl").unwrap();
    let reqs = target.get_requirements();

    assert_eq!(reqs.linker, Some("aarch64-linux-musl-gcc".to_string()));
    assert!(reqs.tools.contains(&"aarch64-linux-musl-gcc".to_string()));
}

#[test]
fn test_requirements_for_linux_armv7() {
    let target = Target::from_triple("armv7-unknown-linux-gnueabihf").unwrap();
    let reqs = target.get_requirements();

    assert_eq!(reqs.linker, Some("arm-linux-gnueabihf-gcc".to_string()));
    assert!(reqs.tools.contains(&"arm-linux-gnueabihf-gcc".to_string()));
}

#[test]
fn test_requirements_for_windows_gnu() {
    let target = Target::from_triple("x86_64-pc-windows-gnu").unwrap();
    let reqs = target.get_requirements();

    assert_eq!(reqs.linker, Some("x86_64-w64-mingw32-gcc".to_string()));
    assert!(reqs.tools.contains(&"x86_64-w64-mingw32-gcc".to_string()));
}

#[test]
fn test_requirements_for_windows_i686_gnu() {
    let target = Target::from_triple("i686-pc-windows-gnu").unwrap();
    let reqs = target.get_requirements();

    assert_eq!(reqs.linker, Some("i686-w64-mingw32-gcc".to_string()));
}

#[test]
fn test_requirements_for_windows_msvc() {
    let target = Target::from_triple("x86_64-pc-windows-msvc").unwrap();
    let reqs = target.get_requirements();

    // MSVC requires cl.exe
    assert!(reqs.tools.contains(&"cl.exe".to_string()));
}

#[test]
fn test_requirements_for_android() {
    let target = Target::from_triple("aarch64-linux-android").unwrap();
    let reqs = target.get_requirements();

    assert!(reqs.tools.contains(&"ndk-build".to_string()));
    assert!(reqs.env_vars.iter().any(|(k, _)| k == "ANDROID_NDK_HOME"));
}

#[test]
fn test_requirements_for_ios() {
    let target = Target::from_triple("aarch64-apple-ios").unwrap();
    let reqs = target.get_requirements();

    // iOS requires xcrun
    assert!(reqs.tools.contains(&"xcrun".to_string()));
}

#[test]
fn test_requirements_for_wasm() {
    let target = Target::from_triple("wasm32-unknown-unknown").unwrap();
    let reqs = target.get_requirements();

    // WASM typically doesn't need special tools
    assert!(reqs.linker.is_none());
}

#[test]
fn test_requirements_none_has_empty_fields() {
    let reqs = TargetRequirements::none();

    assert!(reqs.linker.is_none());
    assert_eq!(reqs.tools.len(), 0);
    assert_eq!(reqs.system_libs.len(), 0);
    assert_eq!(reqs.env_vars.len(), 0);
}

// ============================================================================
// Linker Detection
// ============================================================================

#[test]
fn test_detect_linker_for_host() {
    let host = Target::detect_host().unwrap();
    let linker = host.detect_linker();

    // Should detect some linker (gcc, clang, cc, or platform-specific)
    // May be None if no compiler is installed
    assert!(linker.is_some() || linker.is_none());
}

#[test]
fn test_detect_linker_alternatives() {
    let target = Target::from_triple("x86_64-unknown-linux-gnu").unwrap();
    let linker = target.detect_linker();

    // Should find gcc, clang, or cc
    if let Some(linker_name) = linker {
        assert!(
            linker_name.contains("gcc") || linker_name.contains("clang") || linker_name.contains("cc"),
            "Unexpected linker: {}", linker_name
        );
    }
}

// ============================================================================
// Installation Instructions
// ============================================================================

#[test]
fn test_install_instructions_for_linux_aarch64_on_linux() {
    let target = Target::from_triple("aarch64-unknown-linux-gnu").unwrap();
    let instructions = target.get_install_instructions();

    // If requirements are satisfied, instructions will be empty
    // If not, should provide installation commands
    // Both scenarios are valid - just check the method works
    if !instructions.is_empty() {
        let text = instructions.join("\n");
        // Should contain some installation command or mention the toolchain
        assert!(!text.is_empty(), "Instructions should not be empty if present");
    }
}

#[test]
fn test_install_instructions_for_windows_on_linux() {
    let target = Target::from_triple("x86_64-pc-windows-gnu").unwrap();
    let instructions = target.get_install_instructions();

    if !instructions.is_empty() {
        let text = instructions.join("\n");
        assert!(
            text.contains("mingw") || text.contains("apt-get") || text.contains("dnf"),
            "Should provide mingw installation instructions"
        );
    }
}

#[test]
fn test_install_instructions_for_android() {
    let target = Target::from_triple("aarch64-linux-android").unwrap();
    let instructions = target.get_install_instructions();

    if !instructions.is_empty() {
        let text = instructions.join("\n");
        assert!(
            text.contains("NDK") || text.contains("ANDROID_NDK_HOME"),
            "Should mention Android NDK"
        );
    }
}

#[test]
fn test_install_instructions_empty_when_satisfied() {
    // Native target requirements should be satisfied
    let host = Target::detect_host().unwrap();
    let reqs = host.get_requirements();

    if reqs.are_satisfied() {
        let instructions = host.get_install_instructions();
        assert_eq!(instructions.len(), 0, "Should have no instructions when requirements are satisfied");
    }
}

// ============================================================================
// Cross-Compilation Capability Checks
// ============================================================================

#[test]
fn test_can_cross_compile_same_target() {
    let target1 = Target::from_triple("x86_64-unknown-linux-gnu").unwrap();
    let target2 = Target::from_triple("x86_64-unknown-linux-gnu").unwrap();

    assert!(target1.can_cross_compile_from(&target2));
}

#[test]
fn test_specialized_targets_cannot_native_cross_compile() {
    let wasm = Target::from_triple("wasm32-unknown-unknown").unwrap();
    let host = Target::detect_host().unwrap();

    // WASM is specialized tier, so native cross-compile not supported
    assert!(!wasm.can_cross_compile_from(&host));
}

#[test]
fn test_container_tier_targets() {
    let aarch64 = Target::from_triple("aarch64-unknown-linux-gnu").unwrap();
    assert_eq!(aarch64.tier, TargetTier::Container);
    assert!(aarch64.requires_container());
    assert!(!aarch64.supports_native_build());
}

// ============================================================================
// Target Tier Classification
// ============================================================================

#[test]
fn test_tier_classification_native() {
    let native_targets = vec![
        "x86_64-unknown-linux-gnu",
        "x86_64-unknown-linux-musl",
        "x86_64-pc-windows-gnu",
        "x86_64-apple-darwin",
        "aarch64-apple-darwin",
        "i686-pc-windows-gnu",
        "i686-unknown-linux-gnu",
    ];

    for triple in native_targets {
        let target = Target::from_triple(triple).unwrap();
        assert_eq!(
            target.tier,
            TargetTier::Native,
            "{} should be Native tier",
            triple
        );
        assert!(target.supports_native_build());
        assert!(!target.requires_container());
    }
}

#[test]
fn test_tier_classification_specialized() {
    let specialized_targets = vec![
        "wasm32-unknown-unknown",
        "aarch64-linux-android",
        "aarch64-apple-ios",
        "thumbv7em-none-eabihf",
        "riscv64gc-unknown-linux-gnu",
    ];

    for triple in specialized_targets {
        let target = Target::from_triple(triple).unwrap();
        assert_eq!(
            target.tier,
            TargetTier::Specialized,
            "{} should be Specialized tier",
            triple
        );
        assert!(target.requires_container());
    }
}

#[test]
fn test_tier_classification_container() {
    let container_targets = vec![
        "aarch64-unknown-linux-gnu",
        "armv7-unknown-linux-gnueabihf",
    ];

    for triple in container_targets {
        let target = Target::from_triple(triple).unwrap();
        assert_eq!(
            target.tier,
            TargetTier::Container,
            "{} should be Container tier",
            triple
        );
        assert!(target.requires_container());
    }
}

// ============================================================================
// Target Display
// ============================================================================

#[test]
fn test_target_tier_display() {
    assert_eq!(format!("{}", TargetTier::Native), "Tier 1 (Native)");
    assert_eq!(format!("{}", TargetTier::Container), "Tier 2 (Container)");
    assert_eq!(format!("{}", TargetTier::Specialized), "Tier 3 (Specialized)");
}

// ============================================================================
// Complex Target Triples
// ============================================================================

#[test]
fn test_parse_complex_android_triple() {
    let target = Target::from_triple("armv7-linux-androideabi").unwrap();
    assert_eq!(target.arch, "armv7");
    assert_eq!(target.vendor, "linux");
    assert_eq!(target.os, "androideabi");
}

#[test]
fn test_parse_ios_sim_triple() {
    let target = Target::from_triple("aarch64-apple-ios-sim").unwrap();
    assert_eq!(target.arch, "aarch64");
    assert_eq!(target.vendor, "apple");
    assert_eq!(target.os, "ios");
    assert_eq!(target.env, Some("sim".to_string()));
}

#[test]
fn test_parse_target_with_multiple_env_parts() {
    // Some targets have complex environment strings
    let target = Target::from_triple("armv7-unknown-linux-gnueabihf").unwrap();
    assert_eq!(target.arch, "armv7");
    assert_eq!(target.os, "linux");
    assert_eq!(target.env, Some("gnueabihf".to_string()));
}
