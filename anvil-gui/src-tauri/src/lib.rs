use std::sync::Mutex;

use crate::state::AppState;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod dto;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            vault: Mutex::new(None),
            vault_path: Mutex::new(None),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
