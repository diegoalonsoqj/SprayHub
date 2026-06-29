//! `SteamRepository` implementation: locate Steam, enumerate libraries and
//! resolve the supported-game catalog against them.

use std::path::{Path, PathBuf};

use crate::domain::entities::{GameInfo, SteamDetection};
use crate::domain::error::AppResult;
use crate::domain::repositories::SteamRepository;

use super::game_catalog::{self};
use super::vdf;

#[derive(Default)]
pub struct SteamLocator;

impl SteamLocator {
    pub fn new() -> Self {
        Self
    }
}

impl SteamRepository for SteamLocator {
    fn detect(&self) -> AppResult<SteamDetection> {
        let steam_root = locate_steam_root();
        let libraries = match &steam_root {
            Some(root) => enumerate_libraries(root),
            None => Vec::new(),
        };
        Ok(SteamDetection {
            steam_root: steam_root.map(|p| p.to_string_lossy().into_owned()),
            libraries: libraries
                .into_iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect(),
        })
    }

    fn list_games(&self, detection: &SteamDetection) -> AppResult<Vec<GameInfo>> {
        let libraries: Vec<PathBuf> = detection.libraries.iter().map(PathBuf::from).collect();

        let games = game_catalog::GAMES
            .iter()
            .map(
                |def| match resolve_install_dir(&libraries, def.app_id, def.install_dir_name) {
                    Some(install_dir) => {
                        let sprays_dir = install_dir.join(def.sprays_relative);
                        GameInfo {
                            id: def.id.to_string(),
                            name: def.name.to_string(),
                            app_id: def.app_id,
                            installed: true,
                            install_dir: Some(install_dir.to_string_lossy().into_owned()),
                            sprays_dir: Some(sprays_dir.to_string_lossy().into_owned()),
                        }
                    }
                    None => GameInfo::uninstalled(def),
                },
            )
            .collect();

        Ok(games)
    }
}

/// Resolve a game's install directory by checking each library's
/// `steamapps/common/<installdir>` and `appmanifest_<appid>.acf`.
fn resolve_install_dir(
    libraries: &[PathBuf],
    app_id: u32,
    fallback_dir_name: &str,
) -> Option<PathBuf> {
    for lib in libraries {
        let steamapps = lib.join("steamapps");

        // Prefer the install dir declared in the app manifest.
        let manifest = steamapps.join(format!("appmanifest_{app_id}.acf"));
        if let Ok(text) = std::fs::read_to_string(&manifest) {
            let kv = vdf::parse(&text);
            if let Some(install_dir_name) = kv.first("installdir") {
                let candidate = steamapps.join("common").join(install_dir_name);
                if candidate.is_dir() {
                    return Some(candidate);
                }
            }
        }

        // Fall back to the catalog's known folder name.
        let candidate = steamapps.join("common").join(fallback_dir_name);
        if candidate.is_dir() {
            return Some(candidate);
        }
    }
    None
}

/// Parse `steamapps/libraryfolders.vdf` to enumerate every library folder.
fn enumerate_libraries(steam_root: &Path) -> Vec<PathBuf> {
    let mut libs = vec![steam_root.to_path_buf()];

    let vdf_path = steam_root.join("steamapps").join("libraryfolders.vdf");
    if let Ok(text) = std::fs::read_to_string(&vdf_path) {
        let kv = vdf::parse(&text);
        for path in kv.values_for("path") {
            let p = PathBuf::from(path);
            if p.is_dir() && !libs.iter().any(|existing| existing == &p) {
                libs.push(p);
            }
        }
    }

    libs
}

/// Locate the Steam root directory for the current platform.
fn locate_steam_root() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        if let Some(p) = locate_steam_root_windows() {
            return Some(p);
        }
    }

    default_steam_paths()
        .into_iter()
        .find(|candidate| candidate.join("steamapps").is_dir() || candidate.is_dir())
}

#[cfg(windows)]
fn locate_steam_root_windows() -> Option<PathBuf> {
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
    use winreg::RegKey;

    // Try HKCU\Software\Valve\Steam (SteamPath), then HKLM (InstallPath).
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(key) = hkcu.open_subkey(r"Software\Valve\Steam") {
        if let Ok(path) = key.get_value::<String, _>("SteamPath") {
            let p = PathBuf::from(path.replace('/', "\\"));
            if p.is_dir() {
                return Some(p);
            }
        }
    }

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    for sub in [r"SOFTWARE\WOW6432Node\Valve\Steam", r"SOFTWARE\Valve\Steam"] {
        if let Ok(key) = hklm.open_subkey(sub) {
            if let Ok(path) = key.get_value::<String, _>("InstallPath") {
                let p = PathBuf::from(path);
                if p.is_dir() {
                    return Some(p);
                }
            }
        }
    }
    None
}

/// Well-known default Steam locations per platform.
fn default_steam_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    #[cfg(windows)]
    {
        for env in ["ProgramFiles(x86)", "ProgramFiles"] {
            if let Ok(base) = std::env::var(env) {
                paths.push(PathBuf::from(base).join("Steam"));
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(home) = directories::BaseDirs::new().map(|b| b.home_dir().to_path_buf()) {
            paths.push(home.join(".steam").join("steam"));
            paths.push(home.join(".steam").join("root"));
            paths.push(home.join(".local").join("share").join("Steam"));
            paths.push(
                home.join(".var")
                    .join("app")
                    .join("com.valvesoftware.Steam")
                    .join(".local")
                    .join("share")
                    .join("Steam"),
            );
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = directories::BaseDirs::new().map(|b| b.home_dir().to_path_buf()) {
            paths.push(
                home.join("Library")
                    .join("Application Support")
                    .join("Steam"),
            );
        }
    }

    paths
}
