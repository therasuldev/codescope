use crate::core::resolver::{display_name, resolve};
use crate::core::walker::collect_files;
use anyhow::Result;
use colored::Colorize;
use std::fs;

/// Runs the `cat` command: prints the contents of every file
/// found under the given paths. Empty files are skipped silently.
pub fn run(paths: &[String]) -> Result<()> {
    for path in paths {
        let kind = resolve(path)?;
        let files = collect_files(&kind);

        for file in files {
            let content = fs::read_to_string(&file).unwrap_or_default();

            // Skip empty files, as required by the spec.
            if content.trim().is_empty() {
                continue;
            }

            println!("{}:", display_name(&file).cyan());
            println!("{{");
            println!("{}", content);
            println!("}}");
        }
    }

    Ok(())
}
