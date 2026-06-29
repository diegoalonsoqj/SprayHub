//! Dependency-injection container. Wires concrete infrastructure adapters to the
//! application use cases and exposes them as shared, thread-safe handles.

use std::sync::Arc;

use crate::application::use_cases::{ApplySpray, DetectSteam, ManageConfig, ScanSprays};
use crate::domain::error::AppResult;
use crate::infrastructure::config::JsonConfigRepository;
use crate::infrastructure::filesystem::{FsSprayApplier, FsSprayRepository};
use crate::infrastructure::steam::SteamLocator;

/// Application state shared with every Tauri command via `tauri::State`.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<ManageConfig>,
    pub steam: Arc<DetectSteam>,
    pub sprays: Arc<ScanSprays>,
    pub apply: Arc<ApplySpray>,
}

impl AppState {
    /// Build the production wiring with filesystem/Steam adapters.
    pub fn bootstrap() -> AppResult<Self> {
        let config_repo = Arc::new(JsonConfigRepository::with_default_path()?);
        let steam_repo = Arc::new(SteamLocator::new());
        let spray_repo = Arc::new(FsSprayRepository::new());
        let applier = Arc::new(FsSprayApplier::new());

        Ok(Self {
            config: Arc::new(ManageConfig::new(config_repo)),
            steam: Arc::new(DetectSteam::new(steam_repo)),
            sprays: Arc::new(ScanSprays::new(spray_repo)),
            apply: Arc::new(ApplySpray::new(applier)),
        })
    }
}
