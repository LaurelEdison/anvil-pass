use std::sync::Mutex;

use anvil_core::vault::groups::NewGroup;
use tauri::State;
use uuid::Uuid;

use crate::{
    dto::{group_data_to_group_dto, GroupDto},
    state::{AppError, AppState},
};

#[tauri::command]
pub fn update_group(state: State<'_, Mutex<AppState>>, group_id: Uuid) -> Result<(), AppError> {
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
pub fn delete_group(state: State<'_, Mutex<AppState>>, group_id: Uuid) -> Result<(), AppError> {
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
        .delete_group(group_id)
        .map_err(|e| AppError::GroupDelete(e.to_string()))?;
    vault
        .save(master_password)
        .map_err(|e| AppError::VaultSave(e.to_string()))?;
    Ok(())
}
#[tauri::command]
pub fn create_group(state: State<'_, Mutex<AppState>>, name: String) -> Result<(), AppError> {
    let mut guard = state
        .lock()
        .map_err(|e| AppError::StateLocked(e.to_string()))?;

    let AppState {
        vault,
        master_password,
        ..
    } = &mut *guard;

    let vault = vault.as_mut().ok_or(AppError::VaultNone)?;
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
pub fn list_groups(state: State<'_, Mutex<AppState>>) -> Result<Vec<GroupDto>, AppError> {
    let guard = state
        .lock()
        .map_err(|e| AppError::StateLocked(e.to_string()))?;
    let vault = guard.vault.as_ref().ok_or(AppError::VaultNone)?;
    Ok(vault
        .list_groups()
        .into_iter()
        .map(group_data_to_group_dto)
        .collect())
}
