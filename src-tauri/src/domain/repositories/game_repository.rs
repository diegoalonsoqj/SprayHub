//! Contracts for Steam detection and game discovery.

use crate::domain::entities::{GameInfo, SteamDetection};
use crate::domain::error::AppResult;

pub trait SteamRepository: Send + Sync {
    /// Locate Steam and enumerate all its library folders.
    fn detect(&self) -> AppResult<SteamDetection>;

    /// Resolve the supported-game catalog against the detected libraries.
    fn list_games(&self, detection: &SteamDetection) -> AppResult<Vec<GameInfo>>;
}
