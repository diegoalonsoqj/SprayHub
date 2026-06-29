//! `SprayApplier` implementation: atomic copy of spray files into a game's
//! sprays directory, with optional backup of pre-existing files.

use std::path::{Path, PathBuf};

use crate::domain::entities::{ApplyResult, ApplySprayRequest};
use crate::domain::error::{AppError, AppResult};

#[derive(Default)]
pub struct FsSprayApplier;

impl FsSprayApplier {
    pub fn new() -> Self {
        Self
    }
}

impl crate::domain::repositories::SprayApplier for FsSprayApplier {
    fn apply(&self, request: &ApplySprayRequest) -> AppResult<ApplyResult> {
        let dest_dir = Path::new(&request.destination_dir);
        std::fs::create_dir_all(dest_dir)?;

        // Collect source files to copy (vtf required, vmt optional).
        let mut sources: Vec<PathBuf> = vec![PathBuf::from(&request.vtf_path)];
        if let Some(vmt) = &request.vmt_path {
            sources.push(PathBuf::from(vmt));
        }

        // Determine a backup directory only if needed.
        let mut backup_dir: Option<PathBuf> = None;
        let mut applied = Vec::new();

        for source in &sources {
            let file_name = source
                .file_name()
                .ok_or_else(|| AppError::Validation("source has no file name".into()))?;
            let target = dest_dir.join(file_name);

            if target.exists() {
                if !request.overwrite {
                    return Err(AppError::Validation(format!(
                        "destination already exists: {}",
                        target.display()
                    )));
                }
                if request.create_backup {
                    let dir = match &backup_dir {
                        Some(d) => d.clone(),
                        None => {
                            let d = make_backup_dir(dest_dir)?;
                            backup_dir = Some(d.clone());
                            d
                        }
                    };
                    std::fs::copy(&target, dir.join(file_name))?;
                }
            }

            atomic_copy(source, &target)?;
            applied.push(target.to_string_lossy().into_owned());
        }

        Ok(ApplyResult {
            applied_files: applied,
            backup_dir: backup_dir.map(|d| d.to_string_lossy().into_owned()),
        })
    }
}

/// Copy `source` to `target` atomically: write to a temp file in the same
/// directory, then rename over the target.
fn atomic_copy(source: &Path, target: &Path) -> AppResult<()> {
    let parent = target
        .parent()
        .ok_or_else(|| AppError::Validation("target has no parent directory".into()))?;
    let tmp = parent.join(format!(
        ".{}.tmp",
        target
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "spray".into())
    ));

    std::fs::copy(source, &tmp)?;
    // rename is atomic on the same filesystem; clean up the temp on failure.
    if let Err(e) = std::fs::rename(&tmp, target) {
        let _ = std::fs::remove_file(&tmp);
        return Err(AppError::Filesystem(e.to_string()));
    }
    Ok(())
}

/// Create a timestamped backup directory under the destination.
fn make_backup_dir(dest_dir: &Path) -> AppResult<PathBuf> {
    let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S");
    let dir = dest_dir.join(".sprayhub-backups").join(stamp.to_string());
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::SprayApplier;
    use std::fs;

    #[test]
    fn applies_and_backs_up() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("logo.vtf");
        fs::write(&src, b"new-content").unwrap();

        let dest = tmp.path().join("dest");
        fs::create_dir_all(&dest).unwrap();
        fs::write(dest.join("logo.vtf"), b"old-content").unwrap();

        let applier = FsSprayApplier::new();
        let req = ApplySprayRequest {
            spray_id: "x".into(),
            vtf_path: src.to_string_lossy().into_owned(),
            vmt_path: None,
            destination_dir: dest.to_string_lossy().into_owned(),
            create_backup: true,
            overwrite: true,
        };

        let result = applier.apply(&req).unwrap();
        assert_eq!(result.applied_files.len(), 1);
        assert!(result.backup_dir.is_some());
        assert_eq!(fs::read(dest.join("logo.vtf")).unwrap(), b"new-content");
    }

    #[test]
    fn refuses_overwrite_when_disabled() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("logo.vtf");
        fs::write(&src, b"new").unwrap();
        let dest = tmp.path().join("dest");
        fs::create_dir_all(&dest).unwrap();
        fs::write(dest.join("logo.vtf"), b"old").unwrap();

        let applier = FsSprayApplier::new();
        let req = ApplySprayRequest {
            spray_id: "x".into(),
            vtf_path: src.to_string_lossy().into_owned(),
            vmt_path: None,
            destination_dir: dest.to_string_lossy().into_owned(),
            create_backup: false,
            overwrite: false,
        };
        assert!(applier.apply(&req).is_err());
    }
}
