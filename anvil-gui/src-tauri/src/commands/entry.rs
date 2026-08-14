use anvil_core::vault::entries::{NewEntry, UpdateEntry};
use tauri::State;
use uuid::Uuid;

use crate::{
    dto::{entry_data_to_entry_dto, EntryDto},
    state::{
        AppError::{self, EntryCreate, EntryDelete, EntryUpdate},
        AppState,
    },
};
#[tauri::command]
pub fn update_entry(
    state: State<'_, AppState>,
    id: Uuid,
    parent: Uuid,
    title: Option<String>,
    username: Option<String>,
    password: Option<String>,
    url: Option<String>,
    notes: Option<String>,
    totp: Option<String>,
) -> Result<(), AppError> {
    let mut guard = state.get_vault()?;
    let vault = guard.as_mut().unwrap();
    let master_guard = state.get_master_password()?;
    let master_password = master_guard.as_str();
    let update_entry = UpdateEntry {
        title,
        username,
        password,
        url,
        notes,
        totp,
    };

    vault
        .update_entry(id, update_entry)
        .map_err(|e| EntryUpdate(e))?;
    vault
        .save(master_password)
        .map_err(|e| AppError::VaultSave(e))?;
    Ok(())
}
#[tauri::command]
pub fn delete_entry(state: State<'_, AppState>, entry_id: Uuid) -> Result<bool, AppError> {
    let mut guard = state.get_vault()?;
    let vault = guard.as_mut().unwrap();
    let master_guard = state.get_master_password()?;
    let master_password = master_guard.as_str();
    vault.delete_entry(entry_id).map_err(|e| EntryDelete(e))?;
    vault
        .save(master_password)
        .map_err(|e| AppError::VaultSave(e))?;
    Ok(true)
}
#[tauri::command]
pub fn create_entry(
    state: State<'_, AppState>,
    parent: Option<Uuid>,
    title: Option<String>,
    username: Option<String>,
    password: String,
    url: Option<String>,
    notes: Option<String>,
    totp: Option<String>,
) -> Result<(), AppError> {
    let mut vault_guard = state.get_vault()?;
    let vault = vault_guard.as_mut().unwrap();
    let master_guard = state.get_master_password()?;
    let master_password = master_guard.as_str();
    let new_entry = NewEntry {
        title,
        username,
        password,
        url,
        notes,
        totp,
        group: parent,
    };
    vault.add_entry(new_entry).map_err(|e| EntryCreate(e))?;
    vault
        .save(master_password)
        .map_err(|e| AppError::VaultSave(e))?;
    Ok(())
}
#[tauri::command]
pub fn list_entries(state: State<'_, AppState>) -> Result<Vec<EntryDto>, AppError> {
    let guard = state.get_vault()?;
    let vault = guard.as_ref().unwrap();
    Ok(vault
        .list_entries()
        .into_iter()
        .map(entry_data_to_entry_dto)
        .collect())
}
