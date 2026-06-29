//! Rotating file logger built on `fern`. Logs go to the platform data dir and
//! are size-rotated to keep a small history.

use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;

/// Initialize logging. Returns the log file path on success. Failures are
/// non-fatal — the app still runs without file logging.
pub fn init() -> Option<PathBuf> {
    let dirs = ProjectDirs::from("com", "diegoalonsoqj", "SprayHub")?;
    let log_dir = dirs.data_dir().join("logs");
    if fs::create_dir_all(&log_dir).is_err() {
        return None;
    }

    let log_path = log_dir.join("sprayhub.log");
    rotate_if_needed(&log_path);

    let level = if cfg!(debug_assertions) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    let dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(level);

    let file = match fern::log_file(&log_path) {
        Ok(f) => f,
        Err(_) => return None,
    };

    if dispatch.chain(file).apply().is_err() {
        return None;
    }

    Some(log_path)
}

/// Naive size-based rotation: when the log exceeds ~1 MiB, move it to `.1`.
fn rotate_if_needed(log_path: &PathBuf) {
    const MAX_BYTES: u64 = 1024 * 1024;
    if let Ok(meta) = fs::metadata(log_path) {
        if meta.len() > MAX_BYTES {
            let rotated = log_path.with_extension("log.1");
            let _ = fs::rename(log_path, rotated);
        }
    }
}
