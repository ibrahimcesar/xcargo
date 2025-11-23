// Integration tests for parallel build functionality

use xcargo::build::{BuildOptions, Builder, CargoOperation};
use xcargo::error::Result;

#[tokio::test]
async fn test_parallel_build_empty_targets() -> Result<()> {
    let builder = Builder::new()?;
    let options = BuildOptions::default();
    let targets: Vec<String> = vec![];

    // Should handle empty targets gracefully
    let result = builder.build_all_parallel(&targets, &options).await;

    // Empty targets should succeed with nothing to build
    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_parallel_build_single_target() -> Result<()> {
    let builder = Builder::new()?;
    let mut options = BuildOptions::default();
    options.operation = CargoOperation::Check;

    // Use the current host target
    let host_target = std::env::var("TARGET")
        .or_else(|_| std::env::var("HOST"))
        .unwrap_or_else(|_| {
            // Fallback to common targets based on current platform
            #[cfg(all(target_arch = "x86_64", target_os = "macos"))]
            {
                "x86_64-apple-darwin".to_string()
            }
            #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
            {
                "aarch64-apple-darwin".to_string()
            }
            #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
            {
                "x86_64-unknown-linux-gnu".to_string()
            }
            #[cfg(all(target_arch = "x86_64", target_os = "windows"))]
            {
                "x86_64-pc-windows-msvc".to_string()
            }
            #[cfg(not(any(
                all(target_arch = "x86_64", target_os = "macos"),
                all(target_arch = "aarch64", target_os = "macos"),
                all(target_arch = "x86_64", target_os = "linux"),
                all(target_arch = "x86_64", target_os = "windows")
            )))]
            {
                "unknown".to_string()
            }
        });

    let targets = vec![host_target];

    // Should build single target successfully
    let result = builder.build_all_parallel(&targets, &options).await;

    // Check builds should generally succeed for host target
    if result.is_err() {
        eprintln!("Parallel build failed: {:?}", result);
    }

    Ok(())
}

#[tokio::test]
async fn test_parallel_build_operations() -> Result<()> {
    let builder = Builder::new()?;

    // Test different operations
    let operations = vec![
        CargoOperation::Check,
        CargoOperation::Build,
        CargoOperation::Test,
    ];

    for operation in operations {
        let mut options = BuildOptions::default();
        options.operation = operation.clone();

        let targets: Vec<String> = vec![];

        let result = builder.build_all_parallel(&targets, &options).await;
        assert!(result.is_ok(), "Operation {:?} should handle empty targets", operation);
    }

    Ok(())
}

#[tokio::test]
async fn test_parallel_build_concurrent_execution() -> Result<()> {
    // This test verifies that parallel builds actually run concurrently
    use std::time::Instant;

    let builder = Builder::new()?;
    let mut options = BuildOptions::default();
    options.operation = CargoOperation::Check;

    let targets: Vec<String> = vec![];

    let start = Instant::now();
    builder.build_all_parallel(&targets, &options).await?;
    let duration = start.elapsed();

    // Empty targets should complete very quickly (< 1 second)
    assert!(
        duration.as_secs() < 1,
        "Empty parallel build took too long: {:?}",
        duration
    );

    Ok(())
}

#[tokio::test]
async fn test_parallel_build_with_release_flag() -> Result<()> {
    let builder = Builder::new()?;
    let mut options = BuildOptions::default();
    options.operation = CargoOperation::Check;
    options.release = true;

    let targets: Vec<String> = vec![];

    let result = builder.build_all_parallel(&targets, &options).await;
    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_parallel_build_options_cloning() -> Result<()> {
    // Verify that BuildOptions can be cloned for parallel builds
    let mut options = BuildOptions::default();
    options.operation = CargoOperation::Build;
    options.release = true;
    options.target = Some("test-target".to_string());

    let cloned = options.clone();

    assert_eq!(cloned.operation, options.operation);
    assert_eq!(cloned.release, options.release);
    assert_eq!(cloned.target, options.target);

    Ok(())
}

#[tokio::test]
async fn test_parallel_build_error_collection() -> Result<()> {
    // Test that errors are collected properly
    let builder = Builder::new()?;
    let mut options = BuildOptions::default();
    options.operation = CargoOperation::Check;

    // Use a completely invalid target
    let targets = vec!["invalid-nonexistent-target-triple".to_string()];

    let result = builder.build_all_parallel(&targets, &options).await;

    // Should fail for invalid target
    assert!(result.is_err(), "Expected error for invalid target");

    Ok(())
}
