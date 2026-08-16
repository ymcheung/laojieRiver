use serde::ser::{Serialize, Serializer};
use thiserror::Error;

use crate::core::hide_my_email::HideMyEmailError;
use crate::core::storage::StorageError;
use crate::core::vault::VaultError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Vault is unavailable.")]
    VaultUnavailable,
    #[error("A vault already exists.")]
    VaultExists,
    #[error("Vault is locked.")]
    VaultLocked,
    #[error("{0}")]
    Validation(String),
    #[error("This feature is not implemented yet.")]
    NotImplemented,
    #[error("Live Apple connection is unavailable in this build.")]
    ProviderUnavailable,
    #[error("Internal error.")]
    Internal,
}

impl From<StorageError> for AppError {
    fn from(_: StorageError) -> Self {
        Self::Internal
    }
}

impl From<HideMyEmailError> for AppError {
    fn from(error: HideMyEmailError) -> Self {
        match error {
            HideMyEmailError::ServiceUnavailable => Self::ProviderUnavailable,
            _ => Self::Validation(error.to_string()),
        }
    }
}

impl From<VaultError> for AppError {
    fn from(error: VaultError) -> Self {
        match error {
            VaultError::Locked => Self::VaultLocked,
            VaultError::WeakPassword => Self::Validation(error.to_string()),
            VaultError::NotFound => Self::VaultUnavailable,
            VaultError::InvalidData => Self::Internal,
        }
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
