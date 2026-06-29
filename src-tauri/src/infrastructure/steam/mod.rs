//! Steam-related infrastructure: catalog, VDF parsing and detection.

pub mod game_catalog;
pub mod steam_locator;
pub mod vdf;

pub use steam_locator::SteamLocator;
