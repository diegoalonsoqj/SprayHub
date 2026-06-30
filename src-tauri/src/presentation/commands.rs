//! Tauri command handlers. These are thin: deserialize input, call a use case
//! on a blocking worker (filesystem/registry work is blocking), and map the
//! result into a serializable DTO. No business logic lives here.

use tauri::State;

use crate::application::dto::{ApplyResult, ApplySprayRequest};
use crate::domain::entities::{AppConfig, GameInfo, NewSpray, Spray, SteamDetection};
use crate::infrastructure::filesystem::vtf_encode;

use super::error::{CommandError, CommandResult};
use super::state::AppState;

/// Run blocking work off the UI thread, flattening join + domain errors.
async fn blocking<T, F>(f: F) -> CommandResult<T>
where
    T: Send + 'static,
    F: FnOnce() -> crate::domain::error::AppResult<T> + Send + 'static,
{
    tauri::async_runtime::spawn_blocking(f)
        .await
        .map_err(|e| CommandError {
            category: "Internal".into(),
            message: format!("task join error: {e}"),
        })?
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> CommandResult<AppConfig> {
    let uc = state.config.clone();
    blocking(move || uc.load()).await
}

#[tauri::command]
pub async fn save_config(
    state: State<'_, AppState>,
    config: AppConfig,
) -> CommandResult<AppConfig> {
    let uc = state.config.clone();
    blocking(move || uc.save(&config)).await
}

#[tauri::command]
pub async fn detect_steam(state: State<'_, AppState>) -> CommandResult<SteamDetection> {
    let uc = state.steam.clone();
    blocking(move || uc.detect()).await
}

#[tauri::command]
pub async fn list_games(state: State<'_, AppState>) -> CommandResult<Vec<GameInfo>> {
    let uc = state.steam.clone();
    blocking(move || uc.list_games()).await
}

#[tauri::command]
pub async fn scan_sprays(
    state: State<'_, AppState>,
    library_dir: String,
) -> CommandResult<Vec<Spray>> {
    let uc = state.sprays.clone();
    blocking(move || uc.scan(&library_dir)).await
}

#[tauri::command]
pub async fn get_thumbnail(state: State<'_, AppState>, vtf_path: String) -> CommandResult<String> {
    let uc = state.sprays.clone();
    blocking(move || uc.thumbnail(&vtf_path)).await
}

#[tauri::command]
pub async fn delete_spray(
    state: State<'_, AppState>,
    vtf_path: String,
    vmt_path: Option<String>,
) -> CommandResult<()> {
    let uc = state.sprays.clone();
    blocking(move || uc.delete(&vtf_path, vmt_path.as_deref())).await
}

#[tauri::command]
pub async fn applied_spray_names(
    state: State<'_, AppState>,
    destination_dir: String,
) -> CommandResult<Vec<String>> {
    let uc = state.sprays.clone();
    blocking(move || uc.applied_names(&destination_dir)).await
}

#[tauri::command]
pub async fn apply_spray(
    state: State<'_, AppState>,
    request: ApplySprayRequest,
) -> CommandResult<ApplyResult> {
    let uc = state.apply.clone();
    blocking(move || uc.execute(&request)).await
}

#[tauri::command]
pub async fn create_spray(
    state: State<'_, AppState>,
    name: String,
    width: u32,
    height: u32,
    rgba_base64: String,
    format: String,
    library_dir: String,
) -> CommandResult<Spray> {
    let uc = state.create.clone();
    blocking(move || {
        let rgba = vtf_encode::base64_decode(&rgba_base64)?;
        uc.execute(NewSpray {
            library_dir,
            name,
            width,
            height,
            rgba,
            format,
        })
    })
    .await
}
