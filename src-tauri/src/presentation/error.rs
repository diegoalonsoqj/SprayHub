//! Serializable error wrapper for the Tauri command boundary.

use serde::Serialize;

use crate::domain::error::AppError;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandError {
    pub category: String,
    pub message: String,
}

impl From<AppError> for CommandError {
    fn from(e: AppError) -> Self {
        CommandError {
            category: e.category().to_string(),
            message: e.message().to_string(),
        }
    }
}

/// Result type returned by every Tauri command.
pub type CommandResult<T> = Result<T, CommandError>;
