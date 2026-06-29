//! Steam installation entities.

use serde::{Deserialize, Serialize};

/// The result of detecting Steam on the system.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SteamDetection {
    /// Absolute path to the Steam root, if found.
    pub steam_root: Option<String>,
    /// All Steam library folders found in `libraryfolders.vdf`.
    pub libraries: Vec<String>,
}

impl SteamDetection {
    pub fn is_detected(&self) -> bool {
        self.steam_root.is_some()
    }
}
