//! The `Spray` entity: a `.vtf` texture optionally paired with a `.vmt` material.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spray {
    /// Stable identifier derived from the spray's path.
    pub id: String,
    /// Display name (file stem).
    pub name: String,
    /// Absolute path to the `.vtf` file.
    pub vtf_path: String,
    /// Absolute path to the `.vmt` file, when present.
    pub vmt_path: Option<String>,
    /// Size of the `.vtf` file in bytes.
    pub size_bytes: u64,
    /// Last modification time as Unix seconds.
    pub modified_at: i64,
}

impl Spray {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        vtf_path: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            vtf_path: vtf_path.into(),
            vmt_path: None,
            size_bytes: 0,
            modified_at: 0,
        }
    }
}
