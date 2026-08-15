use std::{path::PathBuf, sync::Mutex};

use tauri::State;

use crate::state::{AppError, AppState};

#[tauri::command]
pub fn open_a_vault(
    state: State<'_, Mutex<AppState>>,
    path: PathBuf,
    master_password: String,
) -> Result<(), AppError> {
    let mut guard = state
        .lock()
        .map_err(|e| AppError::StateLocked(e.to_string()))?;
    let vault = anvil_core::vault::database::open_vault(&master_password, path.clone())
        .map_err(|e| AppError::VaultOpen(e.to_string()))?;
    guard.set_vault(vault, path.clone())?;
    Ok(())
}
#[tauri::command]
pub fn create_vault(
    state: State<'_, Mutex<AppState>>,
    path: PathBuf,
    master_password: String,
) -> Result<(), AppError> {
    let mut guard = state
        .lock()
        .map_err(|e| AppError::StateLocked(e.to_string()))?;
    let vault = anvil_core::vault::database::create_vault(&master_password, path.clone())
        .map_err(|e| AppError::VaultCreate(e.to_string()))?;
    guard.set_vault(vault, path.clone())?;
    Ok(())
}
