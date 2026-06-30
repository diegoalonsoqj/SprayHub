//! `SprayRepository` implementation backed by the local filesystem.

use std::path::Path;
use std::time::UNIX_EPOCH;

use crate::domain::entities::Spray;
use crate::domain::error::{AppError, AppResult};
use crate::domain::path_rules;
use crate::domain::repositories::SprayRepository;

use super::vtf;

#[derive(Default)]
pub struct FsSprayRepository;

impl FsSprayRepository {
    pub fn new() -> Self {
        Self
    }
}

impl SprayRepository for FsSprayRepository {
    fn scan(&self, library_dir: &str) -> AppResult<Vec<Spray>> {
        let dir = Path::new(library_dir);
        if !dir.is_dir() {
            return Err(AppError::NotFound(format!(
                "library directory not found: {library_dir}"
            )));
        }

        let mut sprays = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let is_vtf = path
                .extension()
                .map(|e| e.eq_ignore_ascii_case("vtf"))
                .unwrap_or(false);
            if !is_vtf {
                continue;
            }

            let stem = path
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default();

            let vmt_path = path.with_extension("vmt");
            let vmt = if vmt_path.is_file() {
                Some(vmt_path.to_string_lossy().into_owned())
            } else {
                None
            };

            let metadata = entry.metadata()?;
            let modified_at = metadata
                .modified()
                .ok()
                .and_then(|m| m.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);

            let vtf_path = path.to_string_lossy().into_owned();
            sprays.push(Spray {
                id: stable_id(&vtf_path),
                name: stem,
                vtf_path,
                vmt_path: vmt,
                size_bytes: metadata.len(),
                modified_at,
            });
        }

        Ok(sprays)
    }

    fn thumbnail(&self, vtf_path: &str) -> AppResult<String> {
        let path = Path::new(vtf_path);
        if !path.is_file() {
            return Err(AppError::NotFound(format!("vtf not found: {vtf_path}")));
        }
        let bytes = std::fs::read(path)?;
        vtf::thumbnail_data_url(&bytes)
    }

    fn delete(&self, vtf_path: &str, vmt_path: Option<&str>) -> AppResult<()> {
        let vtf = Path::new(vtf_path);
        path_rules::ensure_no_traversal(vtf)?;
        if vtf.is_file() {
            std::fs::remove_file(vtf)?;
        }
        if let Some(vmt) = vmt_path {
            let vmt = Path::new(vmt);
            path_rules::ensure_no_traversal(vmt)?;
            if vmt.is_file() {
                let _ = std::fs::remove_file(vmt);
            }
        }
        Ok(())
    }

    fn applied_names(&self, dir: &str) -> AppResult<Vec<String>> {
        let path = Path::new(dir);
        if !path.is_dir() {
            return Ok(Vec::new());
        }
        let mut names = Vec::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();
            let is_vtf = p
                .extension()
                .map(|e| e.eq_ignore_ascii_case("vtf"))
                .unwrap_or(false);
            if is_vtf {
                if let Some(stem) = p.file_stem() {
                    names.push(stem.to_string_lossy().into_owned());
                }
            }
        }
        Ok(names)
    }
}

/// Short, stable identifier derived from the absolute path (FNV-1a, hex).
pub(crate) fn stable_id(path: &str) -> String {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = FNV_OFFSET;
    for byte in path.to_lowercase().bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_id_is_deterministic() {
        assert_eq!(stable_id("/a/b.vtf"), stable_id("/a/b.vtf"));
        assert_ne!(stable_id("/a/b.vtf"), stable_id("/a/c.vtf"));
    }
}
