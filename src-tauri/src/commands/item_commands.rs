use serde::Serialize;

use std::sync::Mutex;

use tauri::State;

use crate::{core::errors::AppError, AppState};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultItemSummary {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub username: Option<String>,
    pub url: Option<String>,
    pub favorite: bool,
    pub updated_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultItemDetail {
    id: String,
    kind: String,
    title: String,
    username: Option<String>,
    url: Option<String>,
    notes: Option<String>,
    tags: Vec<String>,
    favorite: bool,
    created_at: String,
    updated_at: String,
}

#[tauri::command]
pub async fn list_items(
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<VaultItemSummary>, AppError> {
    let state = state.lock().map_err(|_| AppError::Internal)?;
    let vault = state.vault.as_ref().ok_or(AppError::VaultUnavailable)?;
    Ok(vault
        .list_hide_my_email_alias_records()?
        .into_iter()
        .map(|(id, alias)| VaultItemSummary {
            id,
            kind: "email_alias".into(),
            title: alias.label.unwrap_or_else(|| alias.address.clone()),
            username: Some(alias.address),
            url: alias.origin,
            favorite: false,
            updated_at: String::new(),
        })
        .collect())
}

#[tauri::command]
pub async fn get_item(id: String) -> Result<VaultItemDetail, AppError> {
    if id.is_empty() {
        return Err(AppError::Validation("Item id is required.".into()));
    }

    Err(AppError::NotImplemented)
}
