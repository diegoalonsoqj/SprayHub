//! Data transfer objects for the application boundary. Most domain entities are
//! already serializable; this module re-exports the request/result types that
//! cross the Tauri boundary so callers have a single import site.

pub use crate::domain::entities::{ApplyResult, ApplySprayRequest};
