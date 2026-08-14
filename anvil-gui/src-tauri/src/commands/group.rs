use std::sync::Mutex;

use tauri::State;

use crate::{
    dto::{group_data_to_group_dto, GroupDto},
    state::{AppError, AppState},
};

#[tauri::command]
pub fn update_group(state: State<'_, AppState>, id: String) {
    let guard = state.get_vault()?;
    let vault = guard.as_mut().unwrap();
}
#[tauri::command]
pub fn delete_group(state: State<'_, AppState>, id: String) {
    let guard = state.get_vault()?;
    let vault = guard.as_mut().unwrap();
}
#[tauri::command]
pub fn create_group(state: State<'_, AppState>) {
    let guard = state.get_vault()?;
    let vault = guard.as_mut().unwrap();
}
#[tauri::command]
pub fn list_groups(state: State<'_, AppState>) -> Result<Vec<GroupDto>, AppError> {
    let guard = state.get_vault()?;
    let vault = guard.as_ref().unwrap();
    Ok(vault
        .list_groups()
        .into_iter()
        .map(group_data_to_group_dto)
        .collect())
}
