mod commands;
mod core;
mod platform;

use std::sync::Mutex;

use commands::{
    clipboard_commands::copy_password,
    hide_my_email_commands::list_hide_my_email_aliases,
    item_commands::{get_item, list_items},
    settings_commands::generate_password,
    vault_commands::{create_vault, discard_vault, get_vault_state, lock_vault, unlock_vault},
};
use core::vault::Vault;

#[derive(Default)]
pub struct AppState {
    pub vault: Option<Vault>,
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
            discard_vault,
            list_hide_my_email_aliases,
            list_items,
            get_item,
            copy_password,
            generate_password
        ])
        .run(tauri::generate_context!())
        .expect("error while running LaoJie River");
}
