use crate::core::resolver::{display_name, resolve};
use crate::core::walker::collect_files;
use anyhow::Result;
use colored::Colorize;
use std::fs;
use walkdir::WalkDir;

/// Runs the `empty` command: reports files with no content
/// and folders that contain no files at all (recursively).
pub fn run(paths: &[String]) -> Result<()> {
    for path in paths {
        let kind = resolve(path)?;

        // Empty files check (works for a single file path too).
        for file in collect_files(&kind) {
            let metadata = fs::metadata(&file)?;
            if metadata.len() == 0 {
                println!("{} {}", "[empty file]".yellow(), display_name(&file));
            }
        }

        // Empty folder check: only meaningful when the path is a directory.
        if let crate::core::resolver::PathKind::Directory(dir) = &kind {
            for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_dir() {
                    let has_files = WalkDir::new(entry.path())
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .any(|e| e.file_type().is_file());

                    if !has_files {
                        println!(
                            "{} {}",
                            "[empty folder]".magenta(),
                            display_name(entry.path())
                        );
                    }
                }
            }
        }
    }

    Ok(())
}
