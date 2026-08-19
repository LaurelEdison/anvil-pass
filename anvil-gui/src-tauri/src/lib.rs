use std::sync::{Arc, Mutex};

use crate::{
    commands::{
        entry::{create_entry, delete_entry, list_entries, update_entry},
        group::{create_group, delete_group, list_groups, update_group},
        password::generate_password,
        vault::{clear_vault, create_vault, is_dirty, open_a_vault, save_vault},
    },
    state::AppState,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod dto;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = Arc::new(Mutex::new(AppState::new()));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            create_vault,
            open_a_vault,
            create_entry,
            update_entry,
            delete_entry,
            list_entries,
            create_group,
            update_group,
            delete_group,
            list_groups,
            generate_password,
            save_vault,
            is_dirty,
            clear_vault,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
