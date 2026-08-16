use std::{sync::Mutex, thread, time::Duration};

use tauri::{AppHandle, State};
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::{core::errors::AppError, AppState};

#[tauri::command]
pub async fn copy_hide_my_email_alias(
    id: String,
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), AppError> {
    if id.is_empty() {
        return Err(AppError::Validation("Item id is required.".into()));
    }
    let address = state
        .lock()
        .map_err(|_| AppError::Internal)?
        .vault
        .as_ref()
        .ok_or(AppError::VaultUnavailable)?
        .list_hide_my_email_alias_records()?
        .into_iter()
        .find_map(|(record_id, alias)| (record_id == id).then_some(alias.address))
        .ok_or_else(|| AppError::Validation("Alias not found.".into()))?;
    app.clipboard()
        .write_text(address.clone())
        .map_err(|_| AppError::Internal)?;
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(30));
        if app.clipboard().read_text().ok().as_deref() == Some(address.as_str()) {
            let _ = app.clipboard().write_text(String::new());
        }
    });
    Ok(())
}

#[tauri::command]
pub async fn copy_password(id: String) -> Result<(), AppError> {
    if id.is_empty() {
        return Err(AppError::Validation("Item id is required.".into()));
    }
    Err(AppError::NotImplemented)
}
