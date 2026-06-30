//! Domain entities: the core data model, free of any framework dependency.

pub mod apply;
pub mod config;
pub mod game;
pub mod new_spray;
pub mod spray;
pub mod steam;

pub use apply::{ApplyResult, ApplySprayRequest};
pub use config::AppConfig;
pub use game::{GameDefinition, GameInfo};
pub use new_spray::NewSpray;
pub use spray::Spray;
pub use steam::SteamDetection;
