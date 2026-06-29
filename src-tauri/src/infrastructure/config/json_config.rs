//! `ConfigRepository` implementation persisting JSON in the platform config dir.
//!
//! - Windows: `%APPDATA%\SprayHub\config.json`
//! - Linux:   `~/.config/sprayhub/config.json`
//! - macOS:   `~/Library/Application Support/com.diegoalonsoqj.SprayHub/config.json`

use std::path::{Path, PathBuf};

use directories::ProjectDirs;

use crate::domain::entities::AppConfig;
use crate::domain::error::{AppError, AppResult};
use crate::domain::repositories::ConfigRepository;

pub struct JsonConfigRepository {
    path: PathBuf,
}

impl JsonConfigRepository {
    /// Build a repository writing to the default per-user config path.
    pub fn with_default_path() -> AppResult<Self> {
        let dirs = ProjectDirs::from("com", "diegoalonsoqj", "SprayHub").ok_or_else(|| {
            AppError::Config("could not resolve a config directory for this platform".into())
        })?;
        let path = dirs.config_dir().join("config.json");
        Ok(Self { path })
    }

    /// Build a repository writing to an explicit path (used in tests).
    pub fn at(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl ConfigRepository for JsonConfigRepository {
    fn load(&self) -> AppResult<AppConfig> {
        if !self.path.exists() {
            return Ok(AppConfig::default());
        }
        let text = std::fs::read_to_string(&self.path)?;
        let config = serde_json::from_str(&text)?;
        Ok(config)
    }

    fn save(&self, config: &AppConfig) -> AppResult<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let text = serde_json::to_string_pretty(config)?;
        // Write atomically: temp file + rename.
        let tmp = self.path.with_extension("json.tmp");
        std::fs::write(&tmp, text.as_bytes())?;
        std::fs::rename(&tmp, &self.path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_returns_default_when_absent() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = JsonConfigRepository::at(tmp.path().join("config.json"));
        assert_eq!(repo.load().unwrap(), AppConfig::default());
    }

    #[test]
    fn save_then_load_roundtrips() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = JsonConfigRepository::at(tmp.path().join("config.json"));
        let mut cfg = AppConfig::default();
        cfg.selected_game_id = Some("tf2".into());
        cfg.language = "en".into();
        repo.save(&cfg).unwrap();
        assert_eq!(repo.load().unwrap(), cfg);
    }
}
