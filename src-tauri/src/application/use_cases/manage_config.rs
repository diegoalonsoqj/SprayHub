//! Use case: load and persist the application configuration.

use std::sync::Arc;

use crate::domain::entities::AppConfig;
use crate::domain::error::{AppError, AppResult};
use crate::domain::path_rules;
use crate::domain::repositories::ConfigRepository;
use std::path::Path;

pub struct ManageConfig {
    config: Arc<dyn ConfigRepository>,
}

impl ManageConfig {
    pub fn new(config: Arc<dyn ConfigRepository>) -> Self {
        Self { config }
    }

    pub fn load(&self) -> AppResult<AppConfig> {
        self.config.load()
    }

    pub fn save(&self, config: &AppConfig) -> AppResult<AppConfig> {
        self.validate(config)?;
        self.config.save(config)?;
        Ok(config.clone())
    }

    fn validate(&self, config: &AppConfig) -> AppResult<()> {
        for dir in [&config.library_dir, &config.destination_dir]
            .into_iter()
            .flatten()
        {
            if !dir.is_empty() {
                path_rules::ensure_no_traversal(Path::new(dir))?;
            }
        }
        match config.language.as_str() {
            "es" | "en" => {}
            other => {
                return Err(AppError::Validation(format!(
                    "unsupported language: {other}"
                )))
            }
        }
        Ok(())
    }
}
