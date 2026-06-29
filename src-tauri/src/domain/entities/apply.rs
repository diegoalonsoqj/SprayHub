//! Request/result entities for the "apply spray" operation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplySprayRequest {
    pub spray_id: String,
    pub vtf_path: String,
    pub vmt_path: Option<String>,
    /// Destination directory (the game's `materials/vgui/logos`).
    pub destination_dir: String,
    /// Back up any existing file before overwriting.
    pub create_backup: bool,
    /// Allow overwriting an existing file.
    pub overwrite: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyResult {
    /// Absolute paths of the files written to the destination.
    pub applied_files: Vec<String>,
    /// Directory where backups were stored, if any.
    pub backup_dir: Option<String>,
}
