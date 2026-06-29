//! Contract for persisting the application configuration.

use crate::domain::entities::AppConfig;
use crate::domain::error::AppResult;

pub trait ConfigRepository: Send + Sync {
    /// Load the configuration, returning defaults if none exists yet.
    fn load(&self) -> AppResult<AppConfig>;

    /// Persist the configuration.
    fn save(&self, config: &AppConfig) -> AppResult<()>;
}
