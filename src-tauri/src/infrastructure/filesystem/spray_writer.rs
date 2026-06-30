//! `SprayWriter` implementation: encode an RGBA image to a `.vtf` and write a
//! matching `.vmt` material into the spray library.

use std::path::Path;
use std::time::UNIX_EPOCH;

use crate::domain::entities::{NewSpray, Spray};
use crate::domain::error::{AppError, AppResult};
use crate::domain::path_rules;
use crate::domain::repositories::SprayWriter;

use super::spray_scanner::stable_id;
use super::vtf_encode::{self, VtfFormat};

#[derive(Default)]
pub struct FsSprayWriter;

impl FsSprayWriter {
    pub fn new() -> Self {
        Self
    }
}

impl SprayWriter for FsSprayWriter {
    fn create(&self, input: &NewSpray) -> AppResult<Spray> {
        let name = sanitize_name(&input.name)?;
        let format = VtfFormat::parse(&input.format)?;

        let dir = Path::new(&input.library_dir);
        path_rules::ensure_no_traversal(dir)?;
        if !dir.is_dir() {
            return Err(AppError::NotFound(format!(
                "library directory not found: {}",
                input.library_dir
            )));
        }

        let vtf_bytes = vtf_encode::encode_vtf(input.width, input.height, &input.rgba, format)?;

        let vtf_path = dir.join(format!("{name}.vtf"));
        let vmt_path = dir.join(format!("{name}.vmt"));

        // Atomic-ish writes: temp file then rename for the texture.
        write_atomic(&vtf_path, &vtf_bytes)?;
        std::fs::write(&vmt_path, vmt_contents(&name).as_bytes())?;

        let metadata = std::fs::metadata(&vtf_path)?;
        let modified_at = metadata
            .modified()
            .ok()
            .and_then(|m| m.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        let vtf_path_str = vtf_path.to_string_lossy().into_owned();
        Ok(Spray {
            id: stable_id(&vtf_path_str),
            name,
            vtf_path: vtf_path_str,
            vmt_path: Some(vmt_path.to_string_lossy().into_owned()),
            size_bytes: metadata.len(),
            modified_at,
        })
    }
}

/// Validate a base file name: non-empty, no extension, no path separators or
/// traversal, restricted to a safe character set.
fn sanitize_name(raw: &str) -> AppResult<String> {
    let name = raw.trim();
    if name.is_empty() {
        return Err(AppError::Validation("spray name is empty".into()));
    }
    if name.contains('/') || name.contains('\\') || name.contains("..") || name.contains(':') {
        return Err(AppError::Validation(
            "spray name contains invalid characters".into(),
        ));
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err(AppError::Validation(
            "spray name may only contain letters, digits, '_' and '-'".into(),
        ));
    }
    Ok(name.to_string())
}

/// Material definition for a translucent logo/spray.
fn vmt_contents(name: &str) -> String {
    format!(
        "\"UnlitGeneric\"\n{{\n\t\"$basetexture\" \"vgui/logos/{name}\"\n\t\"$translucent\" \"1\"\n\t\"$ignorez\" \"1\"\n\t\"$vertexcolor\" \"1\"\n\t\"$vertexalpha\" \"1\"\n}}\n"
    )
}

fn write_atomic(target: &Path, bytes: &[u8]) -> AppResult<()> {
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
    std::fs::write(&tmp, bytes)?;
    if let Err(e) = std::fs::rename(&tmp, target) {
        let _ = std::fs::remove_file(&tmp);
        return Err(AppError::Filesystem(e.to_string()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_accepts_clean_names() {
        assert_eq!(sanitize_name(" my-logo_1 ").unwrap(), "my-logo_1");
    }

    #[test]
    fn sanitize_rejects_bad_names() {
        assert!(sanitize_name("").is_err());
        assert!(sanitize_name("../evil").is_err());
        assert!(sanitize_name("a/b").is_err());
        assert!(sanitize_name("name.vtf").is_err()); // '.' not allowed
    }

    #[test]
    fn creates_vtf_and_vmt() {
        let tmp = tempfile::tempdir().unwrap();
        let writer = FsSprayWriter::new();
        let input = NewSpray {
            library_dir: tmp.path().to_string_lossy().into_owned(),
            name: "logo".into(),
            width: 8,
            height: 8,
            rgba: vec![128u8; 8 * 8 * 4],
            format: "bgra8888".into(),
        };
        let spray = writer.create(&input).unwrap();
        assert!(tmp.path().join("logo.vtf").is_file());
        assert!(tmp.path().join("logo.vmt").is_file());
        assert_eq!(spray.name, "logo");
        assert!(spray.vmt_path.is_some());

        let vmt = std::fs::read_to_string(tmp.path().join("logo.vmt")).unwrap();
        assert!(vmt.contains("vgui/logos/logo"));
        assert!(vmt.contains("$translucent"));
    }
}
