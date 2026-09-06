use crate::core::resolver::PathKind;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

/// Returns true if the entry should be skipped during traversal:
/// hidden files/folders, the "target" build directory and ".git".
fn should_skip(entry: &DirEntry) -> bool {
    let name = entry.file_name().to_str().unwrap_or("");
    (name.starts_with('.') && name != ".") || name == "target"
}

/// Collects all files under the given PathKind.
/// A single file returns itself; a directory returns every file
/// found recursively inside it.
pub fn collect_files(kind: &PathKind) -> Vec<PathBuf> {
    match kind {
        PathKind::File(path) => vec![path.clone()],
        PathKind::Directory(dir) => WalkDir::new(dir)
            .into_iter()
            .filter_entry(|e| !should_skip(e))
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| entry.into_path())
            .collect(),
    }
}

/// Searches the project (starting from `root`) for any file or folder
/// whose name matches `name` exactly. Used as a fallback when the user
/// provides a bare name instead of a full path.
pub fn find_by_name(root: &Path, name: &str) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !should_skip(e))
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().to_str() == Some(name))
        .map(|entry| entry.into_path())
        .collect()
}
