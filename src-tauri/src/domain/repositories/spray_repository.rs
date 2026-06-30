//! Contract for reading sprays and thumbnails from a library directory.

use crate::domain::entities::Spray;
use crate::domain::error::AppResult;

pub trait SprayRepository: Send + Sync {
    /// Scan a directory and return the sprays it contains.
    fn scan(&self, library_dir: &str) -> AppResult<Vec<Spray>>;

    /// Decode a thumbnail for the given `.vtf` file, returned as a data URL.
    fn thumbnail(&self, vtf_path: &str) -> AppResult<String>;

    /// Delete a spray's `.vtf` (and its `.vmt`, if present) from disk.
    fn delete(&self, vtf_path: &str, vmt_path: Option<&str>) -> AppResult<()>;
}
