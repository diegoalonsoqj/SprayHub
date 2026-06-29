//! Persisted application configuration.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    /// Folder holding the user's spray library (`.vtf`/`.vmt`).
    pub library_dir: Option<String>,
    /// Currently selected game id (from the catalog).
    pub selected_game_id: Option<String>,
    /// Destination directory; auto-filled from the selected game but editable.
    pub destination_dir: Option<String>,
    /// Create a backup before overwriting an existing spray.
    pub create_backup: bool,
    /// Ask for confirmation before applying.
    pub confirm_before_apply: bool,
    /// Apply a spray on double-click.
    pub apply_on_double_click: bool,
    /// UI language (`"es"` or `"en"`).
    pub language: String,
    /// UI theme (`"dark"` or `"light"`).
    pub theme: String,
    /// Favorite spray ids (future feature; persisted now for forward-compat).
    #[serde(default)]
    pub favorites: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            library_dir: None,
            selected_game_id: None,
            destination_dir: None,
            create_backup: true,
            confirm_before_apply: true,
            apply_on_double_click: false,
            language: "es".to_string(),
            theme: "dark".to_string(),
            favorites: Vec::new(),
        }
    }
}
