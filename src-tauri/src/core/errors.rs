use serde::ser::{Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Vault is unavailable.")]
    VaultUnavailable,
    #[error("{0}")]
    Validation(String),
    #[error("This feature is not implemented yet.")]
    NotImplemented,
    #[error("Internal error.")]
    Internal,
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

