//! Domain-level error type. Infrastructure errors are mapped into these
//! categories so the application and presentation layers stay decoupled from
//! concrete failure sources.

use std::fmt;

/// All errors surfaced by the SprayHub core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppError {
    /// A configuration could not be read, parsed, or written.
    Config(String),
    /// Steam could not be located or its metadata could not be parsed.
    Steam(String),
    /// A filesystem operation failed.
    Filesystem(String),
    /// User-supplied data failed validation (e.g. unsafe path).
    Validation(String),
    /// A requested resource does not exist.
    NotFound(String),
}

impl AppError {
    /// Stable machine-readable category, useful for the frontend.
    pub fn category(&self) -> &'static str {
        match self {
            AppError::Config(_) => "Config",
            AppError::Steam(_) => "Steam",
            AppError::Filesystem(_) => "Filesystem",
            AppError::Validation(_) => "Validation",
            AppError::NotFound(_) => "NotFound",
        }
    }

    /// Human-readable message.
    pub fn message(&self) -> &str {
        match self {
            AppError::Config(m)
            | AppError::Steam(m)
            | AppError::Filesystem(m)
            | AppError::Validation(m)
            | AppError::NotFound(m) => m,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.category(), self.message())
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Filesystem(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Config(e.to_string())
    }
}

/// Convenience alias used across the core.
pub type AppResult<T> = Result<T, AppError>;
