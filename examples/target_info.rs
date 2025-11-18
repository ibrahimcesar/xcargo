//! Example: Display target information and requirements
//!
//! Run with: cargo run --example target_info

use xcargo::prelude::*;
use colored::Colorize;

fn main() -> Result<()> {
    println!("{}", "🎯 xcargo - Target Information".bold().cyan());
    println!();

    // Detect host target
    println!("{}", "📍 Host Platform:".bold());
    let host = Target::detect_host()?;
    println!("  Triple: {}", host.triple.green());
    println!("  Arch:   {}", host.arch);
    println!("  OS:     {}", host.os);
    println!("  Tier:   {}", host.tier);
    println!();

    // Show some common cross-compilation targets
    let targets = vec![
        "x86_64-unknown-linux-gnu",
        "aarch64-unknown-linux-gnu",
        "x86_64-pc-windows-gnu",
        "aarch64-apple-darwin",
        "wasm32-unknown-unknown",
        "aarch64-linux-android",
    ];

    println!("{}", "🔍 Common Target Information:".bold());
    println!();

    for triple in targets {
        let target = Target::from_triple(triple)?;
        println!("{}:", triple.bold());
        println!("  Tier: {}", target.tier);

        let reqs = target.get_requirements();

        if let Some(linker) = &reqs.linker {
            let available = if which::which(linker).is_ok() {
                "✅".green()
            } else {
                "❌".red()
            };
            println!("  Linker: {} {}", linker, available);
        } else {
            println!("  Linker: {} (default)", "None".dimmed());
        }

        if !reqs.tools.is_empty() {
            println!("  Tools:");
            for tool in &reqs.tools {
                let available = if which::which(tool).is_ok() {
                    "✅".green()
                } else {
                    "❌".red()
                };
                println!("    - {} {}", tool, available);
            }
        }

        if target.can_cross_compile_from(&host) {
            println!("  Status: {} Native compilation possible", "✅".green());
        } else if target.triple == host.triple {
            println!("  Status: {} Host target", "✅".green());
        } else if reqs.are_satisfied() {
            println!("  Status: {} Tools available", "✅".green());
        } else {
            println!("  Status: {} Container build or install tools", "⚠️".yellow());
        }

        println!();
    }

    // Show installation instructions for a cross-compilation target
    println!("{}", "📦 Installation Instructions Example:".bold());
    let arm_target = Target::from_triple("aarch64-unknown-linux-gnu")?;

    if !arm_target.get_requirements().are_satisfied() {
        println!("\nTo build for {}:", arm_target.triple.yellow());
        for instruction in arm_target.get_install_instructions() {
            println!("  {}", instruction);
        }
    } else {
        println!("\n{} {} is ready to use!",
            "✅".green(),
            arm_target.triple.green()
        );
    }

    Ok(())
}
