// Configuration and setup tests
// Tests config creation, loading, saving, and interactive setup logic
// - Config file parsing
// - Config defaults
// - Target selection logic
// - Config save/load roundtrip
// - Setup wizard logic components

use std::fs;
use tempfile::TempDir;
use xcargo::config::Config;
use xcargo::target::Target;
use xcargo::Result;

#[test]
fn test_config_default_creation() {
    let config = Config::default();

    // Default config should have sensible values
    assert!(config.build.parallel, "Parallel builds should be default");
    assert!(config.build.cache, "Build cache should be default");
    assert!(!config.container.runtime.is_empty(), "Container runtime should be set");
}

#[test]
fn test_config_save_and_load() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("xcargo.toml");

    // Create and save config
    let mut config = Config::default();
    let host = Target::detect_host()?;
    config.targets.default = vec![host.triple.clone()];

    config.save(config_path.to_str().unwrap())?;

    // Verify file was created
    assert!(config_path.exists(), "Config file should be created");

    // Load and verify
    let loaded = Config::from_file(config_path.to_str().unwrap())?;
    assert_eq!(loaded.targets.default, config.targets.default);
    assert_eq!(loaded.build.parallel, config.build.parallel);
    assert_eq!(loaded.build.cache, config.build.cache);

    Ok(())
}

#[test]
fn test_config_with_multiple_targets() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("xcargo.toml");

    let mut config = Config::default();
    config.targets.default = vec![
        "x86_64-unknown-linux-gnu".to_string(),
        "aarch64-unknown-linux-gnu".to_string(),
        "x86_64-pc-windows-gnu".to_string(),
    ];

    config.save(config_path.to_str().unwrap())?;

    let loaded = Config::from_file(config_path.to_str().unwrap())?;
    assert_eq!(loaded.targets.default.len(), 3);
    assert!(loaded.targets.default.contains(&"x86_64-unknown-linux-gnu".to_string()));

    Ok(())
}

#[test]
fn test_config_parallel_builds_flag() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("xcargo.toml");

    let mut config = Config::default();
    config.build.parallel = false; // Disable parallel builds

    config.save(config_path.to_str().unwrap())?;

    let loaded = Config::from_file(config_path.to_str().unwrap())?;
    assert!(!loaded.build.parallel, "Parallel builds should be disabled");

    Ok(())
}

#[test]
fn test_config_cache_flag() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("xcargo.toml");

    let mut config = Config::default();
    config.build.cache = false; // Disable cache

    config.save(config_path.to_str().unwrap())?;

    let loaded = Config::from_file(config_path.to_str().unwrap())?;
    assert!(!loaded.build.cache, "Cache should be disabled");

    Ok(())
}

#[test]
fn test_config_container_strategy_auto() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("xcargo.toml");

    let mut config = Config::default();
    config.container.use_when = "target.os != host.os".to_string();

    config.save(config_path.to_str().unwrap())?;

    let loaded = Config::from_file(config_path.to_str().unwrap())?;
    assert_eq!(loaded.container.use_when, "target.os != host.os");

    Ok(())
}

#[test]
fn test_config_container_strategy_always() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("xcargo.toml");

    let mut config = Config::default();
    config.container.use_when = "always".to_string();

    config.save(config_path.to_str().unwrap())?;

    let loaded = Config::from_file(config_path.to_str().unwrap())?;
    assert_eq!(loaded.container.use_when, "always");

    Ok(())
}

#[test]
fn test_config_container_strategy_never() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("xcargo.toml");

    let mut config = Config::default();
    config.container.use_when = "never".to_string();

    config.save(config_path.to_str().unwrap())?;

    let loaded = Config::from_file(config_path.to_str().unwrap())?;
    assert_eq!(loaded.container.use_when, "never");

    Ok(())
}

#[test]
fn test_config_target_specific_settings() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("xcargo.toml");

    let mut config = Config::default();

    // Add target-specific linker configuration
    let target_triple = "x86_64-pc-windows-gnu".to_string();
    let target_config = xcargo::config::TargetCustomConfig {
        linker: Some("x86_64-w64-mingw32-gcc".to_string()),
        force_container: None,
        env: std::collections::HashMap::new(),
        rustflags: None,
    };

    config.targets.custom.insert(target_triple.clone(), target_config);

    config.save(config_path.to_str().unwrap())?;

    let loaded = Config::from_file(config_path.to_str().unwrap())?;
    let target_cfg = loaded.get_target_config(&target_triple);
    assert!(target_cfg.is_some(), "Target config should exist");
    assert_eq!(
        target_cfg.unwrap().linker.as_ref().unwrap(),
        "x86_64-w64-mingw32-gcc"
    );

    Ok(())
}

#[test]
fn test_config_get_target_config_missing() {
    let config = Config::default();
    let result = config.get_target_config("non-existent-target");
    assert!(result.is_none(), "Should return None for missing target");
}

#[test]
fn test_config_host_detection() -> Result<()> {
    let host = Target::detect_host()?;

    // Host should be a valid target triple
    assert!(!host.triple.is_empty());
    assert!(!host.os.is_empty());
    assert!(!host.arch.is_empty());

    Ok(())
}

#[test]
fn test_config_file_format_toml() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("xcargo.toml");

    let mut config = Config::default();
    config.targets.default = vec!["x86_64-unknown-linux-gnu".to_string()];
    config.save(config_path.to_str().unwrap())?;

    // Read raw file and verify TOML format
    let contents = fs::read_to_string(&config_path)?;
    assert!(contents.contains("[build]"), "Should have [build] section");
    assert!(contents.contains("[targets]"), "Should have [targets] section");
    assert!(contents.contains("[container]"), "Should have [container] section");

    Ok(())
}

#[test]
fn test_config_empty_targets_list() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("xcargo.toml");

    let mut config = Config::default();
    config.targets.default = vec![]; // Empty targets

    config.save(config_path.to_str().unwrap())?;

    let loaded = Config::from_file(config_path.to_str().unwrap())?;
    assert_eq!(loaded.targets.default.len(), 0);

    Ok(())
}

#[test]
fn test_config_overwrite_protection() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("xcargo.toml");

    // Create initial config
    let mut config1 = Config::default();
    config1.build.parallel = true;
    config1.save(config_path.to_str().unwrap())?;

    // Overwrite with different config
    let mut config2 = Config::default();
    config2.build.parallel = false;
    config2.save(config_path.to_str().unwrap())?;

    // Verify overwrite worked
    let loaded = Config::from_file(config_path.to_str().unwrap())?;
    assert!(!loaded.build.parallel);

    Ok(())
}

#[test]
fn test_config_with_wasm_target() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("xcargo.toml");

    let mut config = Config::default();
    config.targets.default = vec!["wasm32-unknown-unknown".to_string()];

    config.save(config_path.to_str().unwrap())?;

    let loaded = Config::from_file(config_path.to_str().unwrap())?;
    assert!(loaded.targets.default.contains(&"wasm32-unknown-unknown".to_string()));

    Ok(())
}

#[test]
fn test_config_container_runtime_default() {
    let config = Config::default();

    // Should have a default container runtime
    assert!(!config.container.runtime.is_empty());
    assert!(
        config.container.runtime == "auto" ||
        config.container.runtime == "docker" ||
        config.container.runtime == "podman",
        "Container runtime should be auto/docker/podman"
    );
}

#[test]
fn test_config_target_add_remove() -> Result<()> {
    let mut config = Config::default();
    let initial_count = config.targets.default.len();

    // Add a target
    config.targets.default.push("aarch64-unknown-linux-gnu".to_string());
    assert_eq!(config.targets.default.len(), initial_count + 1);

    // Remove it
    config.targets.default.retain(|t| t != "aarch64-unknown-linux-gnu");
    assert_eq!(config.targets.default.len(), initial_count);

    Ok(())
}

#[test]
fn test_config_duplicate_targets() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("xcargo.toml");

    let mut config = Config::default();
    // Add duplicates
    config.targets.default = vec![
        "x86_64-unknown-linux-gnu".to_string(),
        "x86_64-unknown-linux-gnu".to_string(), // Duplicate
    ];

    config.save(config_path.to_str().unwrap())?;

    let loaded = Config::from_file(config_path.to_str().unwrap())?;
    // Config should preserve duplicates (user's responsibility to clean)
    assert_eq!(loaded.targets.default.len(), 2);

    Ok(())
}

#[test]
fn test_config_missing_file_error() {
    let result = Config::from_file("/non/existent/path/xcargo.toml");
    assert!(result.is_err(), "Should error on missing config file");
}
