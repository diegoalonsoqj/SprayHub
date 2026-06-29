//! Application use cases. Each orchestrates the domain to satisfy one intent and
//! depends only on domain contracts — never on concrete infrastructure.

pub mod apply_spray;
pub mod detect_steam;
pub mod manage_config;
pub mod scan_sprays;

pub use apply_spray::ApplySpray;
pub use detect_steam::DetectSteam;
pub use manage_config::ManageConfig;
pub use scan_sprays::ScanSprays;
