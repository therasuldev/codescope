use crate::core::walker::find_by_name;
use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

/// Represents the kind of a resolved path.
pub enum PathKind {
    File(PathBuf),
    Directory(PathBuf),
}

/// Resolves a user-provided string into a PathKind.
/// First tries the path as-is (relative to the current directory).
/// If that fails, searches the whole project for a matching file or
/// folder name, so the user only needs to type a name (e.g. "loc.rs")
/// instead of a full path.
pub fn resolve(input: &str) -> Result<PathKind> {
    let direct = PathBuf::from(input);
    if direct.exists() {
        return Ok(to_kind(direct));
    }

    let name = Path::new(input)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(input);

    let matches = find_by_name(Path::new("."), name);

    match matches.len() {
        0 => bail!("Path not found in project: {}", input),
        1 => Ok(to_kind(matches[0].clone())),
        _ => {
            let list = matches
                .iter()
                .map(|p| display_name(p))
                .collect::<Vec<_>>()
                .join("\n  ");
            bail!(
                "Multiple matches found for '{}', please be more specific:\n  {}",
                input,
                list
            )
        }
    }
}

fn to_kind(path: PathBuf) -> PathKind {
    if path.is_dir() {
        PathKind::Directory(path)
    } else {
        PathKind::File(path)
    }
}

/// Returns a clean, relative path string for display
/// (strips the leading "./" produced when walking from the current directory).
pub fn display_name(path: &Path) -> String {
    let text = path.to_string_lossy();
    text.strip_prefix("./").unwrap_or(&text).to_string()
}
