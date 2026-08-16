mod commands;
mod core;
mod platform;

use std::{fs, sync::Mutex};

use tauri::Manager;

use commands::{
    clipboard_commands::{copy_hide_my_email_alias, copy_password},
    hide_my_email_commands::{
        disconnect_hide_my_email, get_hide_my_email_status, list_hide_my_email_aliases,
        refresh_hide_my_email_aliases, remove_hide_my_email_aliases, start_hide_my_email_setup,
        verify_hide_my_email_setup,
    },
    item_commands::{get_item, list_items},
    settings_commands::generate_password,
    vault_commands::{create_vault, discard_vault, get_vault_state, lock_vault, unlock_vault},
};
use core::{
    hide_my_email::{AppleHideMyEmailProvider, UnavailableAppleTransport},
    storage::VaultStorage,
    vault::Vault,
};

pub struct AppState {
    pub vault: Option<Vault>,
    pub storage: Option<VaultStorage>,
    pub hide_my_email: Option<AppleHideMyEmailProvider<UnavailableAppleTransport>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            vault: None,
            storage: None,
            hide_my_email: Some(AppleHideMyEmailProvider::new(UnavailableAppleTransport)),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            fs::create_dir_all(&data_dir)?;
            let storage = VaultStorage::open(data_dir.join("vault.sqlite"))?;
            let vault = storage.load_first()?;
            app.manage(Mutex::new(AppState {
                vault,
                storage: Some(storage),
                hide_my_email: Some(AppleHideMyEmailProvider::new(UnavailableAppleTransport)),
            }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_vault_state,
            create_vault,
            unlock_vault,
            lock_vault,
            discard_vault,
            get_hide_my_email_status,
            list_hide_my_email_aliases,
            start_hide_my_email_setup,
            verify_hide_my_email_setup,
            refresh_hide_my_email_aliases,
            disconnect_hide_my_email,
            remove_hide_my_email_aliases,
            copy_hide_my_email_alias,
            list_items,
            get_item,
            copy_password,
            generate_password
        ])
        .run(tauri::generate_context!())
        .expect("error while running LaoJie River");
}
