use std::path::Path;

use rusqlite::{params, Connection, OptionalExtension};
use thiserror::Error;

use crate::core::vault::{Vault, VaultError};

const SCHEMA_VERSION: i64 = 1;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Vault data is unavailable.")]
    NotFound,
    #[error("Vault data is invalid.")]
    InvalidData,
    #[error("The vault database schema is newer than this app supports.")]
    UnsupportedSchema,
    #[error("Vault storage is unavailable.")]
    Unavailable,
}

pub struct VaultStorage {
    connection: Connection,
}

impl VaultStorage {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        Self::from_connection(Connection::open(path).map_err(|_| StorageError::Unavailable)?)
    }

    fn from_connection(connection: Connection) -> Result<Self, StorageError> {
        connection
            .execute_batch("PRAGMA secure_delete = ON; PRAGMA foreign_keys = ON;")
            .map_err(|_| StorageError::Unavailable)?;
        let version: i64 = connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .map_err(|_| StorageError::Unavailable)?;
        if version > SCHEMA_VERSION {
            return Err(StorageError::UnsupportedSchema);
        }
        if version < 1 {
            connection
                .execute_batch(
                    "BEGIN IMMEDIATE;
                     CREATE TABLE IF NOT EXISTS vaults (
                         vault_id TEXT PRIMARY KEY NOT NULL,
                         encrypted_bundle BLOB NOT NULL
                     ) STRICT;
                     PRAGMA user_version = 1;
                     COMMIT;",
                )
                .map_err(|_| StorageError::Unavailable)?;
        }
        Ok(Self { connection })
    }

    pub fn save(&self, vault: &Vault) -> Result<(), StorageError> {
        let bytes = vault
            .to_persisted_bytes()
            .map_err(|_| StorageError::InvalidData)?;
        self.connection
            .execute(
                "INSERT INTO vaults (vault_id, encrypted_bundle) VALUES (?1, ?2)
                 ON CONFLICT(vault_id) DO UPDATE SET encrypted_bundle = excluded.encrypted_bundle",
                params![vault.vault_id(), bytes],
            )
            .map_err(|_| StorageError::Unavailable)?;
        Ok(())
    }

    pub fn load(&self, vault_id: &str) -> Result<Vault, StorageError> {
        if vault_id.is_empty() {
            return Err(StorageError::InvalidData);
        }
        let bytes: Vec<u8> = self
            .connection
            .query_row(
                "SELECT encrypted_bundle FROM vaults WHERE vault_id = ?1",
                [vault_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|_| StorageError::Unavailable)?
            .ok_or(StorageError::NotFound)?;
        Vault::from_persisted_bytes(vault_id, &bytes).map_err(map_vault_error)
    }

    pub fn load_first(&self) -> Result<Option<Vault>, StorageError> {
        let row: Option<(String, Vec<u8>)> = self
            .connection
            .query_row(
                "SELECT vault_id, encrypted_bundle FROM vaults ORDER BY vault_id LIMIT 1",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()
            .map_err(|_| StorageError::Unavailable)?;
        row.map(|(vault_id, bytes)| {
            Vault::from_persisted_bytes(&vault_id, &bytes).map_err(map_vault_error)
        })
        .transpose()
    }

    pub fn delete(&self, vault_id: &str) -> Result<(), StorageError> {
        self.connection
            .execute("DELETE FROM vaults WHERE vault_id = ?1", [vault_id])
            .map_err(|_| StorageError::Unavailable)?;
        Ok(())
    }
}

fn map_vault_error(error: VaultError) -> StorageError {
    match error {
        VaultError::NotFound => StorageError::NotFound,
        VaultError::Locked | VaultError::InvalidData | VaultError::WeakPassword => {
            StorageError::InvalidData
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use uuid::Uuid;

    use crate::core::hide_my_email::HideMyEmailAlias;

    use super::*;

    #[test]
    fn encrypted_vault_survives_restart_and_schema_upgrade_is_idempotent() {
        let path = std::env::temp_dir().join(format!("laojie-vault-{}.sqlite", Uuid::new_v4()));
        let alias = HideMyEmailAlias {
            provider_id: "private-provider-id".into(),
            address: "quiet-river@privaterelay.appleid.com".into(),
            forwarding_address: Some("owner@example.com".into()),
            label: Some("購物".into()),
            note: Some("private note".into()),
            origin: Some("shop.example".into()),
            is_active: true,
        };
        let mut vault = Vault::create("vault-1", "correct horse battery staple").unwrap();
        vault.store_hide_my_email_alias(alias.clone()).unwrap();

        VaultStorage::open(&path).unwrap().save(&vault).unwrap();
        let stored = fs::read(&path).unwrap();
        for secret in [
            "private-provider-id",
            "quiet-river@privaterelay.appleid.com",
            "owner@example.com",
            "購物",
            "private note",
            "shop.example",
        ] {
            assert!(!stored
                .windows(secret.len())
                .any(|part| part == secret.as_bytes()));
        }

        let mut restored = VaultStorage::open(&path).unwrap().load("vault-1").unwrap();
        assert!(!restored.is_unlocked());
        restored.unlock("correct horse battery staple").unwrap();
        assert_eq!(restored.list_hide_my_email_aliases().unwrap(), vec![alias]);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn newer_database_schema_fails_closed() {
        let connection = Connection::open_in_memory().unwrap();
        connection.pragma_update(None, "user_version", 2).unwrap();

        assert!(matches!(
            VaultStorage::from_connection(connection),
            Err(StorageError::UnsupportedSchema)
        ));
    }
}
