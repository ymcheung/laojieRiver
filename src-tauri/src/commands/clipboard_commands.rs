use crate::core::errors::AppError;

#[tauri::command]
pub async fn copy_password(id: String) -> Result<(), AppError> {
    if id.is_empty() {
        return Err(AppError::Validation("Item id is required.".into()));
    }

    Err(AppError::NotImplemented)
}

