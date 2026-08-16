use std::sync::Mutex;

use serde::Serialize;
use tauri::State;

use uuid::Uuid;
use zeroize::Zeroizing;

use crate::{
    core::{errors::AppError, vault::Vault},
    AppState,
};

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
        has_vault: state.vault.is_some(),
        unlocked: state.vault.as_ref().is_some_and(Vault::is_unlocked),
    })
}

#[tauri::command]
pub async fn create_vault(
    master_password: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), AppError> {
    let master_password = Zeroizing::new(master_password);
    let mut state = state.lock().map_err(|_| AppError::Internal)?;
    create_vault_in_state(&mut state, &master_password)
}

fn create_vault_in_state(state: &mut AppState, master_password: &str) -> Result<(), AppError> {
    if state.vault.is_some() {
        return Err(AppError::VaultExists);
    }
    state.vault = Some(Vault::create(&Uuid::new_v4().to_string(), master_password)?);
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

    let master_password = Zeroizing::new(master_password);
    state
        .lock()
        .map_err(|_| AppError::Internal)?
        .vault
        .as_mut()
        .ok_or(AppError::VaultUnavailable)?
        .unlock(&master_password)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn lock_vault(state: State<'_, Mutex<AppState>>) -> Result<(), AppError> {
    let mut state = state.lock().map_err(|_| AppError::Internal)?;
    state
        .vault
        .as_mut()
        .ok_or(AppError::VaultUnavailable)?
        .lock();
    Ok(())
}

#[tauri::command]
pub async fn discard_vault(state: State<'_, Mutex<AppState>>) -> Result<(), AppError> {
    let mut state = state.lock().map_err(|_| AppError::Internal)?;
    discard_vault_in_state(&mut state);
    Ok(())
}

fn discard_vault_in_state(state: &mut AppState) {
    state.vault = None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_second_vault_is_rejected() {
        let mut state = AppState::default();
        create_vault_in_state(&mut state, "correct horse battery staple").unwrap();

        assert!(matches!(
            create_vault_in_state(&mut state, "another correct horse battery staple"),
            Err(AppError::VaultExists)
        ));
    }

    #[test]
    fn discarding_a_vault_removes_it_from_memory() {
        let mut state = AppState::default();
        create_vault_in_state(&mut state, "correct horse battery staple").unwrap();

        discard_vault_in_state(&mut state);

        assert!(state.vault.is_none());
    }
}
