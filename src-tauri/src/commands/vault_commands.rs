use std::sync::Mutex;

use serde::Serialize;
use tauri::State;

use crate::{core::errors::AppError, AppState};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultState {
    has_vault: bool,
    unlocked: bool,
}

#[tauri::command]
pub async fn get_vault_state(state: State<'_, Mutex<AppState>>) -> Result<VaultState, AppError> {
    let state = state.lock().map_err(|_| AppError::Internal)?;

    Ok(VaultState {
        has_vault: state.vault_exists,
        unlocked: state.unlocked,
    })
}

#[tauri::command]
pub async fn create_vault(
    master_password: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), AppError> {
    if master_password.len() < 12 {
        return Err(AppError::Validation(
            "Use a longer master password or passphrase.".into(),
        ));
    }

    let mut state = state.lock().map_err(|_| AppError::Internal)?;
    state.vault_exists = true;
    state.unlocked = true;

    Ok(())
}

#[tauri::command]
pub async fn unlock_vault(
    master_password: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), AppError> {
    if master_password.is_empty() {
        return Err(AppError::Validation("Master password is required.".into()));
    }

    let mut state = state.lock().map_err(|_| AppError::Internal)?;
    if !state.vault_exists {
        return Err(AppError::VaultUnavailable);
    }

    state.unlocked = true;
    Ok(())
}

#[tauri::command]
pub async fn lock_vault(state: State<'_, Mutex<AppState>>) -> Result<(), AppError> {
    let mut state = state.lock().map_err(|_| AppError::Internal)?;
    state.unlocked = false;
    Ok(())
}

