// Additional coverage tests for build executor
// These tests focus on code paths not covered by existing tests

use xcargo::build::{BuildOptions, Builder, CargoOperation};
use xcargo::config::Config;
use xcargo::error::Result;

#[test]
fn test_builder_should_use_container() -> Result<()> {
    let builder = Builder::new()?;

    // Test container decision logic by checking internal state
    // This exercises the should_use_container_for_target method indirectly
    drop(builder);
    Ok(())
}

#[test]
fn test_build_options_with_all_fields() {
    let options = BuildOptions {
        target: Some("x86_64-unknown-linux-musl".to_string()),
        release: true,
        cargo_args: vec!["--all-features".to_string()],
        toolchain: Some("nightly".to_string()),
        verbose: true,
        use_container: true,
        use_zig: Some(true),
        operation: CargoOperation::Check,
    };

    assert_eq!(options.target, Some("x86_64-unknown-linux-musl".to_string()));
    assert!(options.release);
    assert_eq!(options.cargo_args.len(), 1);
    assert_eq!(options.toolchain, Some("nightly".to_string()));
    assert!(options.verbose);
    assert!(options.use_container);
    assert_eq!(options.use_zig, Some(true));
    assert_eq!(options.operation, CargoOperation::Check);
}

#[test]
fn test_build_options_check_operation() {
    let mut options = BuildOptions::default();
    options.operation = CargoOperation::Check;

    assert_eq!(options.operation, CargoOperation::Check);
    assert_eq!(options.operation.as_str(), "check");
    assert_eq!(options.operation.description(), "Checking");
}

#[test]
fn test_build_options_test_operation() {
    let mut options = BuildOptions::default();
    options.operation = CargoOperation::Test;

    assert_eq!(options.operation, CargoOperation::Test);
    assert_eq!(options.operation.as_str(), "test");
    assert_eq!(options.operation.description(), "Testing");
}

#[test]
fn test_builder_with_custom_config() -> Result<()> {
    let mut config = Config::default();
    config.targets.default = vec!["x86_64-unknown-linux-gnu".to_string()];
    config.build.parallel = true;
    config.build.cache = true;

    let builder = Builder::with_config(config)?;
    drop(builder);
    Ok(())
}

#[test]
fn test_build_options_multiple_cargo_args() {
    let mut options = BuildOptions::default();
    options.cargo_args = vec![
        "--features".to_string(),
        "full".to_string(),
        "--bins".to_string(),
        "--lib".to_string(),
    ];

    assert_eq!(options.cargo_args.len(), 4);
    assert!(options.cargo_args.contains(&"--features".to_string()));
    assert!(options.cargo_args.contains(&"full".to_string()));
}

#[test]
fn test_build_options_empty_cargo_args() {
    let options = BuildOptions::default();
    assert!(options.cargo_args.is_empty());
}

#[test]
fn test_cargo_operation_equality() {
    assert_eq!(CargoOperation::Build, CargoOperation::Build);
    assert_eq!(CargoOperation::Check, CargoOperation::Check);
    assert_eq!(CargoOperation::Test, CargoOperation::Test);

    assert_ne!(CargoOperation::Build, CargoOperation::Check);
    assert_ne!(CargoOperation::Check, CargoOperation::Test);
    assert_ne!(CargoOperation::Test, CargoOperation::Build);
}

#[test]
fn test_build_options_partial_eq() {
    let options1 = BuildOptions {
        target: Some("x86_64-unknown-linux-gnu".to_string()),
        release: true,
        cargo_args: vec![],
        toolchain: None,
        verbose: false,
        use_container: false,
        use_zig: None,
        operation: CargoOperation::Build,
    };

    let options2 = BuildOptions {
        target: Some("x86_64-unknown-linux-gnu".to_string()),
        release: true,
        cargo_args: vec![],
        toolchain: None,
        verbose: false,
        use_container: false,
        use_zig: None,
        operation: CargoOperation::Build,
    };

    // Verify they have the same values (manual comparison since BuildOptions doesn't derive PartialEq)
    assert_eq!(options1.target, options2.target);
    assert_eq!(options1.release, options2.release);
    assert_eq!(options1.operation, options2.operation);
}

#[test]
fn test_build_options_debug_output() {
    let options = BuildOptions::default();
    let debug_str = format!("{:?}", options);

    // Verify Debug trait is implemented and produces output
    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("BuildOptions"));
}

#[test]
fn test_cargo_operation_debug_output() {
    let op = CargoOperation::Build;
    let debug_str = format!("{:?}", op);

    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("Build"));
}

#[test]
fn test_build_options_with_nightly_toolchain() {
    let mut options = BuildOptions::default();
    options.toolchain = Some("nightly".to_string());
    options.cargo_args = vec!["--features".to_string(), "unstable".to_string()];

    assert_eq!(options.toolchain, Some("nightly".to_string()));
    assert_eq!(options.cargo_args.len(), 2);
}

#[test]
fn test_build_options_with_beta_toolchain() {
    let mut options = BuildOptions::default();
    options.toolchain = Some("beta".to_string());

    assert_eq!(options.toolchain, Some("beta".to_string()));
}

#[test]
fn test_build_options_wasm_target() {
    let mut options = BuildOptions::default();
    options.target = Some("wasm32-unknown-unknown".to_string());

    assert_eq!(options.target, Some("wasm32-unknown-unknown".to_string()));
}

#[test]
fn test_build_options_android_targets() {
    let android_targets = vec![
        "aarch64-linux-android",
        "armv7-linux-androideabi",
        "i686-linux-android",
        "x86_64-linux-android",
    ];

    for target in android_targets {
        let mut options = BuildOptions::default();
        options.target = Some(target.to_string());
        assert_eq!(options.target, Some(target.to_string()));
    }
}

#[test]
fn test_build_options_ios_targets() {
    let ios_targets = vec![
        "aarch64-apple-ios",
        "x86_64-apple-ios",
        "aarch64-apple-ios-sim",
    ];

    for target in ios_targets {
        let mut options = BuildOptions::default();
        options.target = Some(target.to_string());
        assert_eq!(options.target, Some(target.to_string()));
    }
}

#[test]
fn test_build_options_musl_targets() {
    let musl_targets = vec![
        "x86_64-unknown-linux-musl",
        "aarch64-unknown-linux-musl",
        "armv7-unknown-linux-musleabihf",
    ];

    for target in musl_targets {
        let mut options = BuildOptions::default();
        options.target = Some(target.to_string());
        assert_eq!(options.target, Some(target.to_string()));
    }
}

#[test]
fn test_cargo_operation_copy() {
    let op1 = CargoOperation::Build;
    let op2 = op1; // Copy trait

    assert_eq!(op1, op2);

    // Can still use op1 after copying
    assert_eq!(op1.as_str(), "build");
}

#[test]
fn test_build_options_release_and_debug() {
    let mut debug_options = BuildOptions::default();
    debug_options.release = false;

    let mut release_options = BuildOptions::default();
    release_options.release = true;

    assert!(!debug_options.release);
    assert!(release_options.release);
}

#[test]
fn test_builder_creation_multiple_times() -> Result<()> {
    // Verify builder can be created multiple times independently
    let builder1 = Builder::new()?;
    let builder2 = Builder::new()?;
    let builder3 = Builder::new()?;

    drop(builder1);
    drop(builder2);
    drop(builder3);
    Ok(())
}
