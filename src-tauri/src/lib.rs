//! SprayHub core library. The Tauri application is assembled here so the same
//! crate can be reused (e.g. by a future CLI) without the binary entry point.

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use presentation::commands;
use presentation::AppState;

/// Build and run the SprayHub desktop application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if let Some(path) = infrastructure::logging::init() {
        log::info!("SprayHub starting; logging to {}", path.display());
    }

    let state = AppState::bootstrap().unwrap_or_else(|e| {
        eprintln!("Fatal: failed to initialize SprayHub state: {e}");
        std::process::exit(1);
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::save_config,
            commands::detect_steam,
            commands::list_games,
            commands::scan_sprays,
            commands::get_thumbnail,
            commands::apply_spray,
        ])
        .run(tauri::generate_context!())
        .expect("error while running the SprayHub application");
}
