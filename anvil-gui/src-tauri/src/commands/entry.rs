use std::sync::Mutex;

use anvil_core::vault::entries::{NewEntry, UpdateEntry};
use tauri::State;
use uuid::Uuid;

use crate::{
    dto::{entry_data_to_entry_dto, EntryDto},
    state::{
        AppError::{self, EntryCreate, EntryDelete, EntryMove, EntryUpdate},
        AppState,
    },
};
#[tauri::command]
pub fn update_entry(
    state: State<'_, Mutex<AppState>>,
    id: Uuid,
    title: Option<String>,
    username: Option<String>,
    password: Option<String>,
    url: Option<String>,
    notes: Option<String>,
    totp: Option<String>,
) -> Result<(), AppError> {
    let mut guard = state
        .lock()
        .map_err(|e| AppError::StateLocked(e.to_string()))?;

    let AppState { vault, .. } = &mut *guard;

    let vault = vault.as_mut().ok_or(AppError::VaultNone)?;

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
        .map_err(|e| EntryUpdate(e.to_string()))?;

    Ok(())
}
#[tauri::command]
pub fn delete_entry(state: State<'_, Mutex<AppState>>, entry_id: Uuid) -> Result<(), AppError> {
    let mut guard = state
        .lock()
        .map_err(|e| AppError::StateLocked(e.to_string()))?;

    let AppState { vault, .. } = &mut *guard;

    let vault = vault.as_mut().ok_or(AppError::VaultNone)?;
    vault
        .delete_entry(entry_id)
        .map_err(|e| EntryDelete(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn move_entry(
    state: State<'_, Mutex<AppState>>,
    entry_id: Uuid,
    destination_id: Uuid,
) -> Result<(), AppError> {
    let mut guard = state
        .lock()
        .map_err(|e| AppError::StateLocked(e.to_string()))?;

    let AppState { vault, .. } = &mut *guard;

    let vault = vault.as_mut().ok_or(AppError::VaultNone)?;
    vault
        .move_entry(entry_id, destination_id)
        .map_err(|e| EntryMove(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn create_entry(
    state: State<'_, Mutex<AppState>>,
    parent: Option<Uuid>,
    title: Option<String>,
    username: Option<String>,
    password: String,
    url: Option<String>,
    notes: Option<String>,
    totp: Option<String>,
) -> Result<(), AppError> {
    let mut guard = state
        .lock()
        .map_err(|e| AppError::StateLocked(e.to_string()))?;

    let AppState { vault, .. } = &mut *guard;

    let vault = vault.as_mut().ok_or(AppError::VaultNone)?;
    let new_entry = NewEntry {
        title,
        username,
        password,
        url,
        notes,
        totp,
        group: parent,
    };
    vault
        .add_entry(new_entry)
        .map_err(|e| EntryCreate(e.to_string()))?;
    Ok(())
}
#[tauri::command]
pub fn list_entries(state: State<'_, Mutex<AppState>>) -> Result<Vec<EntryDto>, AppError> {
    let guard = state
        .lock()
        .map_err(|e| AppError::StateLocked(e.to_string()))?;
    let vault = guard.vault.as_ref().ok_or(AppError::VaultNone)?;
    Ok(vault
        .list_entries()
        .into_iter()
        .map(entry_data_to_entry_dto)
        .collect())
}
