use std::sync::Mutex;

use tauri::State;

use crate::{
    dto::{entry_data_to_entry_dto, EntryDto},
    state::{AppError, AppState},
};
#[tauri::command]
pub fn update_entry(
    state: State<'_, Mutex<AppState>>,
    id: String,
    parent: String,
    title: String,
    username: String,
    password: String,
    url: String,
    notes: String,
    totp: String,
) {
}
#[tauri::command]
pub fn delete_entry(state: State<'_, Mutex<AppState>>, id: String) -> bool {
    true
}
#[tauri::command]
pub fn create_entry(
    state: State<'_, Mutex<AppState>>,
    parent: String,
    title: String,
    username: String,
    password: String,
    url: String,
    notes: String,
    totp: String,
) {
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
