//! Contract for creating spray files from raw image data.

use crate::domain::entities::{NewSpray, Spray};
use crate::domain::error::AppResult;

pub trait SprayWriter: Send + Sync {
    /// Encode and write the `.vtf` (and a matching `.vmt`) for `input`,
    /// returning the created spray's metadata.
    fn create(&self, input: &NewSpray) -> AppResult<Spray>;
}
