//! Pure path-safety rules. These belong to the domain because they encode a
//! core security invariant: operations must never escape an expected root.

use crate::domain::error::{AppError, AppResult};
use std::path::{Component, Path, PathBuf};

/// Reject paths containing parent (`..`) traversal components.
pub fn ensure_no_traversal(path: &Path) -> AppResult<()> {
    if path
        .components()
        .any(|c| matches!(c, Component::ParentDir))
    {
        return Err(AppError::Validation(format!(
            "path traversal is not allowed: {}",
            path.display()
        )));
    }
    Ok(())
}

/// Ensure `candidate` is located inside `root` after normalization. Both paths
/// should already be absolute/canonical for a reliable check.
pub fn ensure_within(root: &Path, candidate: &Path) -> AppResult<()> {
    let root = normalize(root);
    let candidate = normalize(candidate);
    if candidate.starts_with(&root) {
        Ok(())
    } else {
        Err(AppError::Validation(format!(
            "path {} escapes the allowed root {}",
            candidate.display(),
            root.display()
        )))
    }
}

/// Lexically normalize a path (resolve `.` and `..`) without touching the disk.
pub fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for comp in path.components() {
        match comp {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_parent_traversal() {
        let p = Path::new("a/../b");
        assert!(ensure_no_traversal(p).is_err());
    }

    #[test]
    fn accepts_clean_path() {
        let p = Path::new("a/b/c");
        assert!(ensure_no_traversal(p).is_ok());
    }

    #[test]
    fn within_root_ok() {
        let root = Path::new("/games/l4d2");
        let candidate = Path::new("/games/l4d2/materials/vgui/logos/x.vtf");
        assert!(ensure_within(root, candidate).is_ok());
    }

    #[test]
    fn outside_root_err() {
        let root = Path::new("/games/l4d2");
        let candidate = Path::new("/games/csgo/x.vtf");
        assert!(ensure_within(root, candidate).is_err());
    }
}
