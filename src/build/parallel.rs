//! Parallel build execution

use crate::error::{Error, Result};
use crate::output::helpers;
use std::sync::{Arc, Mutex};
use tokio::task;

use super::executor::Builder;
use super::options::BuildOptions;

impl Builder {
    /// Build multiple targets in parallel using tokio tasks
    pub async fn build_all_parallel(
        &self,
        targets: &[String],
        options: &BuildOptions,
    ) -> Result<()> {
        helpers::section(format!("xcargo {} (parallel)", options.operation.as_str()));
        helpers::info(format!(
            "{} for {} targets in parallel",
            options.operation.description(),
            targets.len()
        ));

        let successes = Arc::new(Mutex::new(Vec::new()));
        let failures = Arc::new(Mutex::new(Vec::new()));

        let mut handles = Vec::new();

        for (idx, target) in targets.iter().enumerate() {
            let target = target.clone();
            let mut target_options = options.clone();
            target_options.target = Some(target.clone());

            let successes = Arc::clone(&successes);
            let failures = Arc::clone(&failures);

            let handle = task::spawn_blocking(move || {
                println!("\n[{}] Starting build for: {}", idx + 1, target);
                println!("{}", "─".repeat(50));

                // Create a new builder for this task
                let builder = match Builder::new() {
                    Ok(b) => b,
                    Err(e) => {
                        let mut failures = failures.lock().unwrap();
                        failures.push(target.clone());
                        eprintln!("Failed to create builder for {target}: {e}");
                        return;
                    }
                };

                match builder.build(&target_options) {
                    Ok(()) => {
                        let mut successes = successes.lock().unwrap();
                        successes.push(target.clone());
                    }
                    Err(e) => {
                        let mut failures = failures.lock().unwrap();
                        failures.push(target.clone());
                        eprintln!("Failed to build {target}: {e}");
                    }
                }
            });

            handles.push(handle);
        }

        // Wait for all builds to complete
        for handle in handles {
            handle
                .await
                .map_err(|e| Error::Build(format!("Task join error: {e}")))?;
        }

        let successes = successes.lock().unwrap();
        let failures = failures.lock().unwrap();

        println!("\n");
        helpers::section("Build Summary");
        helpers::success(format!("{} target(s) built successfully", successes.len()));

        if !failures.is_empty() {
            helpers::error(format!("{} target(s) failed", failures.len()));
            for target in failures.iter() {
                helpers::error(format!("  - {target}"));
            }
            return Err(Error::Build("Some targets failed to build".to_string()));
        }

        Ok(())
    }
}
