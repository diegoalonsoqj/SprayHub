//! Source-engine game entities and the supported-game catalog contract.

use serde::{Deserialize, Serialize};

/// A statically-known supported Source game (catalog entry).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameDefinition {
    /// Stable slug, e.g. `"left4dead2"`.
    pub id: &'static str,
    /// Display name, e.g. `"Left 4 Dead 2"`.
    pub name: &'static str,
    /// Steam App ID.
    pub app_id: u32,
    /// Steam `installdir` folder name under `steamapps/common`.
    pub install_dir_name: &'static str,
    /// Relative path (from the install dir) to the sprays/logos folder.
    pub sprays_relative: &'static str,
}

/// A game annotated with runtime installation state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInfo {
    pub id: String,
    pub name: String,
    pub app_id: u32,
    /// Whether the game was found in any Steam library.
    pub installed: bool,
    /// Absolute path to the game install directory, if installed.
    pub install_dir: Option<String>,
    /// Absolute path to the sprays (`materials/vgui/logos`) directory, if installed.
    pub sprays_dir: Option<String>,
}

impl GameInfo {
    /// Build an uninstalled `GameInfo` from a catalog definition.
    pub fn uninstalled(def: &GameDefinition) -> Self {
        Self {
            id: def.id.to_string(),
            name: def.name.to_string(),
            app_id: def.app_id,
            installed: false,
            install_dir: None,
            sprays_dir: None,
        }
    }
}
