use tauri::State;

use crate::{
    dto::{group_data_to_group_dto, GroupDto},
    state::{AppError, AppState},
};

#[tauri::command]
pub fn update_group(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    let mut guard = state.get_vault()?;
    let vault = guard.as_mut().ok_or(AppError::VaultNone)?;
    let master_guard = state.get_master_password()?;
    let master_password = master_guard.as_str();
    vault
        .save(master_password)
        .map_err(|e| AppError::VaultSave(e.to_string()))?;
    Ok(())
}
#[tauri::command]
pub fn delete_group(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    let mut guard = state.get_vault()?;
    let vault = guard.as_mut().ok_or(AppError::VaultNone)?;
    let master_guard = state.get_master_password()?;
    let master_password = master_guard.as_str();
    vault
        .save(master_password)
        .map_err(|e| AppError::VaultSave(e.to_string()))?;
    Ok(())
}
#[tauri::command]
pub fn create_group(state: State<'_, AppState>) -> Result<(), AppError> {
    let mut guard = state.get_vault()?;
    let vault = guard.as_mut().ok_or(AppError::VaultNone)?;
    let master_guard = state.get_master_password()?;
    let master_password = master_guard.as_str();
    vault
        .save(master_password)
        .map_err(|e| AppError::VaultSave(e.to_string()))?;
    Ok(())
}
#[tauri::command]
pub fn list_groups(state: State<'_, AppState>) -> Result<Vec<GroupDto>, AppError> {
    let guard = state.get_vault()?;
    let vault = guard.as_ref().ok_or(AppError::VaultNone)?;
    Ok(vault
        .list_groups()
        .into_iter()
        .map(group_data_to_group_dto)
        .collect())
}
