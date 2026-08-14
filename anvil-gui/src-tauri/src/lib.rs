use crate::{
    commands::{
        entry::{create_entry, delete_entry, update_entry},
        group::{create_group, delete_group, update_group},
        vault::{create_vault, open_a_vault},
    },
    state::AppState,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod dto;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            create_vault,
            open_a_vault,
            create_entry,
            update_entry,
            delete_entry,
            create_group,
            update_group,
            delete_group
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
