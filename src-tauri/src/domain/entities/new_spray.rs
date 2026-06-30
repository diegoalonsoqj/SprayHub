//! Input for creating a spray from a raw RGBA image.

/// A request to materialize a new spray (`.vtf` + `.vmt`) in a library folder.
pub struct NewSpray {
    /// Library directory the spray files are written into.
    pub library_dir: String,
    /// Base file name (without extension), already sanitized by the caller.
    pub name: String,
    pub width: u32,
    pub height: u32,
    /// Row-major RGBA8888 pixels (`width * height * 4` bytes).
    pub rgba: Vec<u8>,
    /// Texture format key (e.g. `"bgra8888"` or `"dxt5"`).
    pub format: String,
}
