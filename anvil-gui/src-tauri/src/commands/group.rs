use anvil_core::vault::groups::NewGroup;
use tauri::State;
use uuid::Uuid;

use crate::{
    dto::{group_data_to_group_dto, GroupDto},
    state::{AppError, AppState},
};

#[tauri::command]
pub fn update_group(state: State<'_, AppState>, group_id: Uuid) -> Result<(), AppError> {
    //TODO
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
pub fn delete_group(state: State<'_, AppState>, group_id: Uuid) -> Result<(), AppError> {
    let mut guard = state.get_vault()?;
    let vault = guard.as_mut().ok_or(AppError::VaultNone)?;
    let master_guard = state.get_master_password()?;
    let master_password = master_guard.as_str();
    vault
        .delete_group(group_id)
        .map_err(|e| AppError::GroupDelete(e.to_string()))?;
    vault
        .save(master_password)
        .map_err(|e| AppError::VaultSave(e.to_string()))?;
    Ok(())
}
#[tauri::command]
pub fn create_group(state: State<'_, AppState>, name: String) -> Result<(), AppError> {
    let mut guard = state.get_vault()?;
    let vault = guard.as_mut().ok_or(AppError::VaultNone)?;
    let master_guard = state.get_master_password()?;
    let master_password = master_guard.as_str();
    let new_group = NewGroup::new(name);
    vault
        .add_group(new_group)
        .map_err(|e| AppError::GroupCreate(e.to_string()))?;
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
