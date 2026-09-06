use crate::core::resolver::{display_name, resolve};
use crate::core::walker::collect_files;
use anyhow::Result;
use colored::Colorize;
use std::fs;

/// Runs the `loc` command: counts lines per file and prints
/// a per-file breakdown plus a total across all given paths.
pub fn run(paths: &[String]) -> Result<()> {
    let mut total = 0usize;

    for path in paths {
        let kind = resolve(path)?;
        let files = collect_files(&kind);

        for file in files {
            let content = fs::read_to_string(&file).unwrap_or_default();
            if content.is_empty() {
                continue;
            }

            let lines = content.lines().count();
            total += lines;

            println!("{}: {} lines", display_name(&file).cyan(), lines);
        }
    }

    println!("{}", format!("Total: {} lines", total).bold());

    Ok(())
}
