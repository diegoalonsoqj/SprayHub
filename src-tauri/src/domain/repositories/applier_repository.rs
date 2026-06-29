//! Contract for applying a spray to a game directory.

use crate::domain::entities::{ApplyResult, ApplySprayRequest};
use crate::domain::error::AppResult;

pub trait SprayApplier: Send + Sync {
    /// Copy the spray's files into the destination, optionally backing up and
    /// honoring the overwrite flag. Implementations must be atomic per file.
    fn apply(&self, request: &ApplySprayRequest) -> AppResult<ApplyResult>;
}
