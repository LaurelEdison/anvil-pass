use std::path::PathBuf;

use tauri::State;

use crate::state::{AppError, AppState};

#[tauri::command]
pub fn open_a_vault(
    state: State<'_, AppState>,
    path: PathBuf,
    master_password: String,
) -> Result<(), AppError> {
    let vault = anvil_core::vault::database::open_vault(&master_password, path.clone())
        .map_err(|e| AppError::OpenVault(e))?;
    state.set_vault(vault, path.clone())?;
    Ok(())
}
#[tauri::command]
pub fn create_vault(
    state: State<'_, AppState>,
    path: PathBuf,
    master_password: String,
) -> Result<(), AppError> {
    let vault = anvil_core::vault::database::create_vault(&master_password, path.clone())
        .map_err(|e| AppError::CreateVault(e))?;
    state.set_vault(vault, path.clone())?;
    Ok(())
}
