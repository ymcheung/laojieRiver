use serde::Deserialize;

use crate::core::errors::AppError;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordGeneratorOptions {
    length: u8,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
    avoid_ambiguous: bool,
}

#[tauri::command]
pub async fn generate_password(options: PasswordGeneratorOptions) -> Result<String, AppError> {
    let _ = (
        options.uppercase,
        options.lowercase,
        options.numbers,
        options.symbols,
        options.avoid_ambiguous,
    );

    if !(8..=128).contains(&options.length) {
        return Err(AppError::Validation(
            "Password length must be between 8 and 128.".into(),
        ));
    }

    Err(AppError::NotImplemented)
}

