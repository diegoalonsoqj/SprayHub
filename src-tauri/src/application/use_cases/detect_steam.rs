//! Use case: detect Steam and list supported games.

use std::sync::Arc;

use crate::domain::entities::{GameInfo, SteamDetection};
use crate::domain::error::AppResult;
use crate::domain::repositories::SteamRepository;

pub struct DetectSteam {
    steam: Arc<dyn SteamRepository>,
}

impl DetectSteam {
    pub fn new(steam: Arc<dyn SteamRepository>) -> Self {
        Self { steam }
    }

    pub fn detect(&self) -> AppResult<SteamDetection> {
        self.steam.detect()
    }

    pub fn list_games(&self) -> AppResult<Vec<GameInfo>> {
        let detection = self.steam.detect()?;
        self.steam.list_games(&detection)
    }
}
