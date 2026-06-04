mod commands;
mod core;
mod platform;

use std::sync::Mutex;

use commands::{
    clipboard_commands::copy_password,
    item_commands::{get_item, list_items},
    settings_commands::generate_password,
    vault_commands::{create_vault, get_vault_state, lock_vault, unlock_vault},
};

#[derive(Default)]
pub struct AppState {
    pub vault_exists: bool,
    pub unlocked: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![
            get_vault_state,
            create_vault,
            unlock_vault,
            lock_vault,
            list_items,
            get_item,
            copy_password,
            generate_password
        ])
        .run(tauri::generate_context!())
        .expect("error while running LaoJie River");
}

