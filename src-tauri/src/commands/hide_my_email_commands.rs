use std::sync::Mutex;

use tauri::State;

use crate::{
    core::{errors::AppError, hide_my_email::HideMyEmailAlias},
    AppState,
};

#[tauri::command]
pub async fn list_hide_my_email_aliases(
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<HideMyEmailAlias>, AppError> {
    let state = state.lock().map_err(|_| AppError::Internal)?;
    state
        .vault
        .as_ref()
        .ok_or(AppError::VaultUnavailable)?
        .list_hide_my_email_aliases()
        .map_err(Into::into)
}
