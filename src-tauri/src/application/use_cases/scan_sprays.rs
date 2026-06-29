//! Use case: scan a spray library and produce thumbnails.

use std::sync::Arc;

use crate::domain::entities::Spray;
use crate::domain::error::AppResult;
use crate::domain::repositories::SprayRepository;

pub struct ScanSprays {
    sprays: Arc<dyn SprayRepository>,
}

impl ScanSprays {
    pub fn new(sprays: Arc<dyn SprayRepository>) -> Self {
        Self { sprays }
    }

    pub fn scan(&self, library_dir: &str) -> AppResult<Vec<Spray>> {
        let mut sprays = self.sprays.scan(library_dir)?;
        // Deterministic, case-insensitive ordering for a stable grid.
        sprays.sort_by_key(|s| s.name.to_lowercase());
        Ok(sprays)
    }

    pub fn thumbnail(&self, vtf_path: &str) -> AppResult<String> {
        self.sprays.thumbnail(vtf_path)
    }
}
