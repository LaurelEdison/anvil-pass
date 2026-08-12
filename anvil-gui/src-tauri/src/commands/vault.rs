use std::{path::PathBuf, sync::Mutex};

use tauri::State;

use crate::state::{AppError, AppState};

#[tauri::command]
pub async fn open_a_vault(
    state: State<'_, Mutex<AppState>>,
    path: PathBuf,
    master_password: String,
) -> Result<(), AppError> {
    let mut guard = state
        .lock()
        .map_err(|e| AppError::StateLocked(e.to_string()))?;
    let vault = anvil_core::vault::database::open_vault(&master_password, path.clone())
        .map_err(|e| AppError::VaultOpen(e.to_string()))?;
    guard.set_vault(vault, path.clone(), master_password)?;
    Ok(())
}

#[tauri::command]
pub async fn create_vault(
    state: State<'_, Mutex<AppState>>,
    path: PathBuf,
    master_password: String,
) -> Result<(), AppError> {
    let mut guard = state
        .lock()
        .map_err(|e| AppError::StateLocked(e.to_string()))?;
    let vault = anvil_core::vault::database::create_vault(&master_password, path.clone())
        .map_err(|e| AppError::VaultCreate(e.to_string()))?;
    guard.set_vault(vault, path.clone(), master_password)?;
    Ok(())
}

#[tauri::command]
pub async fn save_vault(state: State<'_, Mutex<AppState>>) -> Result<(), AppError> {
    let mut guard = state
        .lock()
        .map_err(|e| AppError::StateLocked(e.to_string()))?;

    let AppState {
        vault,
        master_password,
        ..
    } = &mut *guard;

    let vault = vault.as_mut().ok_or(AppError::VaultNone)?;

    vault
        .save(master_password)
        .map_err(|e| AppError::VaultSave(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn clear_vault(state: State<'_, Mutex<AppState>>) -> Result<(), AppError> {
    let mut guard = state
        .lock()
        .map_err(|e| AppError::StateLocked(e.to_string()))?;
    guard.reset()?;
    Ok(())
}

#[tauri::command]
pub fn is_dirty(state: State<'_, Mutex<AppState>>) -> Result<bool, AppError> {
    let mut guard = state
        .lock()
        .map_err(|e| AppError::StateLocked(e.to_string()))?;

    let AppState { vault, .. } = &mut *guard;

    let vault = vault.as_mut().ok_or(AppError::VaultNone)?;
    Ok(vault.dirty)
}
