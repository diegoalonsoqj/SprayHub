//! Repository/service contracts (ports). Infrastructure provides the adapters.

pub mod applier_repository;
pub mod config_repository;
pub mod game_repository;
pub mod spray_repository;

pub use applier_repository::SprayApplier;
pub use config_repository::ConfigRepository;
pub use game_repository::SteamRepository;
pub use spray_repository::SprayRepository;
