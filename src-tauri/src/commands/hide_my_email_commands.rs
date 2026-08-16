use std::sync::Mutex;

use serde::Serialize;
use tauri::State;
use zeroize::Zeroizing;

use crate::{
    core::{
        errors::AppError,
        hide_my_email::{
            reconcile_aliases, AppleHideMyEmailProvider, AuthState, UnavailableAppleTransport,
        },
    },
    platform::keychain::{KeychainSessionStore, ProtectedSessionStore},
    AppState,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HideMyEmailAliasSummary {
    id: String,
    address: String,
    label: Option<String>,
    origin: Option<String>,
    is_active: bool,
    updated_at: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HideMyEmailStatus {
    connection_state: &'static str,
    setup_available: bool,
    masked_apple_id: Option<String>,
    last_refresh_at: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HideMyEmailSetupResult {
    requires_verification: bool,
    methods: Vec<&'static str>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HideMyEmailRefreshResult {
    added: usize,
    updated: usize,
    unchanged: usize,
    marked_inactive: usize,
}

#[tauri::command]
pub async fn get_hide_my_email_status(
    state: State<'_, Mutex<AppState>>,
) -> Result<HideMyEmailStatus, AppError> {
    let state = state.lock().map_err(|_| AppError::Internal)?;
    if state.vault.is_none() {
        return Err(AppError::VaultUnavailable);
    }
    let vault_id = state.vault.as_ref().unwrap().vault_id();
    let has_stored_session = KeychainSessionStore.load(vault_id).ok().flatten().is_some();
    let (connection_state, masked_apple_id) = match state
        .hide_my_email
        .as_ref()
        .map(|provider| provider.state())
    {
        None => ("syncing", None),
        Some(AuthState::Connected { masked_account }) => {
            ("connected", Some(masked_account.clone()))
        }
        Some(AuthState::Disconnected) if has_stored_session => ("expired", None),
        Some(AuthState::Disconnected) => ("disconnected", None),
        Some(AuthState::VerificationRequired { .. } | AuthState::SmsCodeRequired { .. }) => {
            ("disconnected", None)
        }
    };
    Ok(HideMyEmailStatus {
        connection_state,
        setup_available: false,
        masked_apple_id,
        last_refresh_at: None,
    })
}

#[tauri::command]
pub async fn list_hide_my_email_aliases(
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<HideMyEmailAliasSummary>, AppError> {
    let state = state.lock().map_err(|_| AppError::Internal)?;
    let mut aliases: Vec<_> = state
        .vault
        .as_ref()
        .ok_or(AppError::VaultUnavailable)?
        .list_hide_my_email_alias_records()?
        .into_iter()
        .map(|(id, alias)| HideMyEmailAliasSummary {
            id,
            address: alias.address,
            label: alias.label,
            origin: alias.origin,
            is_active: alias.is_active,
            updated_at: None,
        })
        .collect();
    aliases.sort_by(|left, right| {
        right
            .is_active
            .cmp(&left.is_active)
            .then_with(|| left.label.cmp(&right.label))
            .then_with(|| left.address.cmp(&right.address))
    });
    Ok(aliases)
}

#[tauri::command]
pub async fn start_hide_my_email_setup(
    apple_id: String,
    password: String,
    acknowledged: bool,
    state: State<'_, Mutex<AppState>>,
) -> Result<HideMyEmailSetupResult, AppError> {
    let apple_id = Zeroizing::new(apple_id);
    let password = Zeroizing::new(password);
    if !acknowledged || apple_id.trim().is_empty() || password.is_empty() {
        return Err(AppError::Validation(
            "Apple Account, password, and acknowledgement are required.".into(),
        ));
    }
    let mut provider = {
        let mut state = state.lock().map_err(|_| AppError::Internal)?;
        let vault = state.vault.as_ref().ok_or(AppError::VaultUnavailable)?;
        if !vault.is_unlocked() {
            return Err(AppError::VaultLocked);
        }
        state.hide_my_email.take().ok_or_else(|| {
            AppError::Validation("A Hide My Email operation is already running.".into())
        })?
    };
    let mut result = provider.start_connect(&apple_id, &password).map(|state| {
        let methods = match state {
            AuthState::VerificationRequired {
                trusted_device_available,
                sms_destinations,
            } => {
                let mut methods = Vec::new();
                if *trusted_device_available {
                    methods.push("trustedDevice");
                }
                if !sms_destinations.is_empty() {
                    methods.push("sms");
                }
                methods
            }
            _ => Vec::new(),
        };
        HideMyEmailSetupResult {
            requires_verification: !methods.is_empty(),
            methods,
        }
    });
    let vault_is_unlocked = state
        .lock()
        .map_err(|_| AppError::Internal)?
        .vault
        .as_ref()
        .is_some_and(|vault| vault.is_unlocked());
    if !vault_is_unlocked {
        provider.disconnect();
    }
    if result.is_ok() && matches!(provider.state(), AuthState::Connected { .. }) {
        if save_session(&state, &provider).is_err() {
            provider.disconnect();
            result = Err(crate::core::hide_my_email::HideMyEmailError::ServiceUnavailable);
        }
    }
    state.lock().map_err(|_| AppError::Internal)?.hide_my_email = Some(provider);
    if !vault_is_unlocked {
        return Err(AppError::VaultLocked);
    }
    result.map_err(Into::into)
}

#[tauri::command]
pub async fn verify_hide_my_email_setup(
    code: String,
    method: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<HideMyEmailRefreshResult, AppError> {
    let code = Zeroizing::new(code);
    if code.is_empty() || !matches!(method.as_str(), "trustedDevice" | "sms") {
        return Err(AppError::Validation(
            "A valid verification method and code are required.".into(),
        ));
    }
    let mut provider = {
        let mut state = state.lock().map_err(|_| AppError::Internal)?;
        state.hide_my_email.take().ok_or_else(|| {
            AppError::Validation("A Hide My Email operation is already running.".into())
        })?
    };
    let verified = (|| match method.as_str() {
        "trustedDevice" => provider.submit_trusted_device_code(&code).map(|_| ()),
        "sms" => {
            let destination = match provider.state() {
                AuthState::VerificationRequired {
                    sms_destinations, ..
                } => sms_destinations.first().map(|item| item.id.clone()),
                _ => None,
            }
            .ok_or(crate::core::hide_my_email::HideMyEmailError::VerificationNotPending)?;
            provider.request_sms_code(&destination)?;
            provider.submit_sms_code(&code).map(|_| ())
        }
        _ => unreachable!(),
    })();
    let vault_is_unlocked = state
        .lock()
        .map_err(|_| AppError::Internal)?
        .vault
        .as_ref()
        .is_some_and(|vault| vault.is_unlocked());
    if !vault_is_unlocked {
        provider.disconnect();
    }
    if verified.is_ok() && save_session(&state, &provider).is_err() {
        provider.disconnect();
        state.lock().map_err(|_| AppError::Internal)?.hide_my_email = Some(provider);
        return Err(AppError::Internal);
    }
    state.lock().map_err(|_| AppError::Internal)?.hide_my_email = Some(provider);
    if !vault_is_unlocked {
        return Err(AppError::VaultLocked);
    }
    verified.map_err(AppError::from)?;
    refresh_hide_my_email_aliases(state).await
}

#[tauri::command]
pub async fn refresh_hide_my_email_aliases(
    state: State<'_, Mutex<AppState>>,
) -> Result<HideMyEmailRefreshResult, AppError> {
    let mut provider = {
        let mut state = state.lock().map_err(|_| AppError::Internal)?;
        state.hide_my_email.take().ok_or_else(|| {
            AppError::Validation("A Hide My Email operation is already running.".into())
        })?
    };
    let incoming = provider.list();
    let mut state = state.lock().map_err(|_| AppError::Internal)?;
    let result = (|| {
        let existing = state
            .vault
            .as_ref()
            .ok_or(AppError::VaultUnavailable)?
            .list_hide_my_email_aliases()?;
        let reconciliation = reconcile_aliases(&existing, incoming?)?;
        let counts = reconciliation.counts;
        let original = state.vault.clone();
        state
            .vault
            .as_mut()
            .unwrap()
            .apply_hide_my_email_aliases(reconciliation.aliases_to_store)?;
        if let (Some(storage), Some(vault)) = (&state.storage, &state.vault) {
            if let Err(error) = storage.save(vault) {
                state.vault = original;
                return Err(error.into());
            }
        }
        Ok(HideMyEmailRefreshResult {
            added: counts.added,
            updated: counts.updated,
            unchanged: counts.unchanged,
            marked_inactive: counts.marked_inactive,
        })
    })();
    state.hide_my_email = Some(provider);
    result
}

#[tauri::command]
pub async fn disconnect_hide_my_email(state: State<'_, Mutex<AppState>>) -> Result<(), AppError> {
    let mut state = state.lock().map_err(|_| AppError::Internal)?;
    let vault_id = state
        .vault
        .as_ref()
        .ok_or(AppError::VaultUnavailable)?
        .vault_id();
    KeychainSessionStore
        .delete(vault_id)
        .map_err(|_| AppError::Internal)?;
    if let Some(provider) = state.hide_my_email.as_mut() {
        provider.disconnect();
    }
    Ok(())
}

#[tauri::command]
pub async fn remove_hide_my_email_aliases(
    state: State<'_, Mutex<AppState>>,
) -> Result<(), AppError> {
    let mut state = state.lock().map_err(|_| AppError::Internal)?;
    let original = state.vault.clone();
    state
        .vault
        .as_mut()
        .ok_or(AppError::VaultUnavailable)?
        .remove_all_hide_my_email_aliases()?;
    if let (Some(storage), Some(vault)) = (&state.storage, &state.vault) {
        if let Err(error) = storage.save(vault) {
            state.vault = original;
            return Err(error.into());
        }
    }
    Ok(())
}

fn save_session(
    state: &State<'_, Mutex<AppState>>,
    provider: &AppleHideMyEmailProvider<UnavailableAppleTransport>,
) -> Result<(), AppError> {
    let Some(session) = provider.session_for_keychain() else {
        return Ok(());
    };
    let payload = Zeroizing::new(
        serde_json::json!({
            "accountIdentifier": &*session.account_identifier,
            "session": &*session.session,
        })
        .to_string(),
    );
    let vault_id = state
        .lock()
        .map_err(|_| AppError::Internal)?
        .vault
        .as_ref()
        .ok_or(AppError::VaultUnavailable)?
        .vault_id()
        .to_owned();
    KeychainSessionStore
        .store(&vault_id, &payload)
        .map_err(|_| AppError::Internal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{hide_my_email::HideMyEmailAlias, vault::Vault};

    #[test]
    fn serialized_alias_summary_excludes_provider_secrets() {
        let mut vault = Vault::create("vault-1", "correct horse battery staple").unwrap();
        vault
            .store_hide_my_email_alias(HideMyEmailAlias {
                provider_id: "private-provider-id".into(),
                address: "alias@example.com".into(),
                forwarding_address: Some("owner@example.com".into()),
                label: Some("Shopping".into()),
                note: Some("private note".into()),
                origin: Some("shop.example".into()),
                is_active: true,
            })
            .unwrap();
        let (id, alias) = vault.list_hide_my_email_alias_records().unwrap().remove(0);
        let json = serde_json::to_string(&HideMyEmailAliasSummary {
            id,
            address: alias.address,
            label: alias.label,
            origin: alias.origin,
            is_active: alias.is_active,
            updated_at: None,
        })
        .unwrap();

        assert!(json.contains("alias@example.com"));
        for secret in ["private-provider-id", "owner@example.com", "private note"] {
            assert!(!json.contains(secret));
        }
    }
}
