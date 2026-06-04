use serde::Serialize;

use crate::core::errors::AppError;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultItemSummary {
    id: String,
    kind: String,
    title: String,
    username: Option<String>,
    url: Option<String>,
    favorite: bool,
    updated_at: String,
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
pub async fn list_items() -> Result<Vec<VaultItemSummary>, AppError> {
    Ok(Vec::new())
}

#[tauri::command]
pub async fn get_item(id: String) -> Result<VaultItemDetail, AppError> {
    if id.is_empty() {
        return Err(AppError::Validation("Item id is required.".into()));
    }

    Err(AppError::NotImplemented)
}

