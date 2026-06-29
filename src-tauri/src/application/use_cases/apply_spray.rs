//! Use case: apply a spray to a game directory, enforcing safety rules.

use std::path::Path;
use std::sync::Arc;

use crate::domain::entities::{ApplyResult, ApplySprayRequest};
use crate::domain::error::{AppError, AppResult};
use crate::domain::path_rules;
use crate::domain::repositories::SprayApplier;

pub struct ApplySpray {
    applier: Arc<dyn SprayApplier>,
}

impl ApplySpray {
    pub fn new(applier: Arc<dyn SprayApplier>) -> Self {
        Self { applier }
    }

    pub fn execute(&self, request: &ApplySprayRequest) -> AppResult<ApplyResult> {
        self.validate(request)?;
        self.applier.apply(request)
    }

    fn validate(&self, request: &ApplySprayRequest) -> AppResult<()> {
        if request.vtf_path.trim().is_empty() {
            return Err(AppError::Validation("source .vtf path is empty".into()));
        }
        if request.destination_dir.trim().is_empty() {
            return Err(AppError::Validation(
                "destination directory is empty".into(),
            ));
        }

        let vtf = Path::new(&request.vtf_path);
        path_rules::ensure_no_traversal(vtf)?;
        if !vtf.exists() {
            return Err(AppError::NotFound(format!(
                "source spray not found: {}",
                request.vtf_path
            )));
        }
        if let Some(vmt) = &request.vmt_path {
            path_rules::ensure_no_traversal(Path::new(vmt))?;
        }
        path_rules::ensure_no_traversal(Path::new(&request.destination_dir))?;
        Ok(())
    }
}
