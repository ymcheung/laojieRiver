use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use zeroize::Zeroizing;

use crate::core::{
    crypto::{decrypt, derive_key, encrypt, random_bytes, EncryptedEnvelope, KdfParams, KEY_LEN},
    hide_my_email::HideMyEmailAlias,
};

const SALT_LEN: usize = 16;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct VaultMetadata {
    kdf: KdfParams,
    salt: Vec<u8>,
    wrapped_key: EncryptedEnvelope,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct EncryptedRecord {
    id: String,
    revision: u64,
    updated_at: u64,
    envelope: EncryptedEnvelope,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SyncBundle {
    schema_version: u16,
    vault_id: String,
    metadata: VaultMetadata,
    records: Vec<EncryptedRecord>,
}

#[cfg(test)]
pub(crate) trait SyncStore {
    fn get(&self, vault_id: &str) -> Option<SyncBundle>;
    fn put(&mut self, bundle: SyncBundle);
}

#[cfg(test)]
#[derive(Default)]
pub struct MemorySyncStore(BTreeMap<String, SyncBundle>);

#[cfg(test)]
impl MemorySyncStore {
    pub fn wire_bytes(&self, vault_id: &str) -> Result<Vec<u8>, VaultError> {
        let bundle = self.0.get(vault_id).ok_or(VaultError::NotFound)?;
        let mut bytes = serde_json::to_vec(bundle).map_err(|_| VaultError::InvalidData)?;
        bytes.extend_from_slice(&bundle.metadata.wrapped_key.ciphertext);
        for record in &bundle.records {
            bytes.extend_from_slice(&record.envelope.ciphertext);
        }
        Ok(bytes)
    }

    pub fn set_kdf_memory(&mut self, vault_id: &str, memory_kib: u32) -> Result<(), VaultError> {
        self.0
            .get_mut(vault_id)
            .ok_or(VaultError::NotFound)?
            .metadata
            .kdf
            .memory_kib = memory_kib;
        Ok(())
    }

    #[cfg(test)]
    pub fn tamper_first_ciphertext(&mut self, vault_id: &str) -> Result<(), VaultError> {
        let byte = self
            .0
            .get_mut(vault_id)
            .and_then(|bundle| bundle.records.first_mut())
            .and_then(|record| record.envelope.ciphertext.first_mut())
            .ok_or(VaultError::NotFound)?;
        *byte ^= 1;
        Ok(())
    }
}

#[cfg(test)]
impl SyncStore for MemorySyncStore {
    fn get(&self, vault_id: &str) -> Option<SyncBundle> {
        self.0.get(vault_id).cloned()
    }

    fn put(&mut self, bundle: SyncBundle) {
        self.0.insert(bundle.vault_id.clone(), bundle);
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum VaultError {
    #[error("Vault is locked.")]
    Locked,
    #[error("Vault data is unavailable.")]
    NotFound,
    #[error("Vault data is invalid.")]
    InvalidData,
    #[error("Use a longer master password or passphrase.")]
    WeakPassword,
}

#[derive(Clone)]
pub struct Vault {
    vault_id: String,
    metadata: VaultMetadata,
    records: BTreeMap<String, EncryptedRecord>,
    key: Option<Zeroizing<[u8; KEY_LEN]>>,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
enum VaultItem {
    EmailAlias(HideMyEmailAlias),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ItemPayload {
    schema_version: u16,
    item: VaultItem,
}

impl Vault {
    pub fn create(vault_id: &str, master_password: &str) -> Result<Self, VaultError> {
        if master_password.len() < 12 {
            return Err(VaultError::WeakPassword);
        }

        let salt = random_bytes::<SALT_LEN>();
        let key = Zeroizing::new(random_bytes::<KEY_LEN>());
        let kdf = KdfParams::default();
        let wrapping_key = Zeroizing::new(
            derive_key(master_password, &salt, &kdf).map_err(|_| VaultError::InvalidData)?,
        );
        let wrapped_key = encrypt(
            &wrapping_key,
            key.as_slice(),
            vault_key_aad(vault_id).as_bytes(),
        )
        .map_err(|_| VaultError::InvalidData)?;

        Ok(Self {
            vault_id: vault_id.into(),
            metadata: VaultMetadata {
                kdf,
                salt: salt.to_vec(),
                wrapped_key,
            },
            records: BTreeMap::new(),
            key: Some(key),
        })
    }

    #[cfg(test)]
    pub(crate) fn download(vault_id: &str, store: &impl SyncStore) -> Result<Self, VaultError> {
        let bundle = store.get(vault_id).ok_or(VaultError::NotFound)?;
        Self::from_bundle(vault_id, bundle)
    }

    pub fn vault_id(&self) -> &str {
        &self.vault_id
    }

    pub fn to_persisted_bytes(&self) -> Result<Vec<u8>, VaultError> {
        serde_json::to_vec(&self.bundle()).map_err(|_| VaultError::InvalidData)
    }

    pub fn from_persisted_bytes(vault_id: &str, bytes: &[u8]) -> Result<Self, VaultError> {
        let bundle = serde_json::from_slice(bytes).map_err(|_| VaultError::InvalidData)?;
        Self::from_bundle(vault_id, bundle)
    }

    fn from_bundle(vault_id: &str, bundle: SyncBundle) -> Result<Self, VaultError> {
        if bundle.vault_id != vault_id
            || bundle.schema_version != 1
            || bundle.metadata.salt.len() != SALT_LEN
            || !bundle.metadata.kdf.is_supported()
            || bundle.records.iter().any(|record| {
                record.id.is_empty() || record.revision == 0 || record.envelope.nonce.is_empty()
            })
        {
            return Err(VaultError::InvalidData);
        }

        let record_count = bundle.records.len();
        let records: BTreeMap<_, _> = bundle
            .records
            .into_iter()
            .map(|record| (record.id.clone(), record))
            .collect();
        if records.len() != record_count {
            return Err(VaultError::InvalidData);
        }

        Ok(Self {
            vault_id: bundle.vault_id,
            metadata: bundle.metadata,
            records,
            key: None,
        })
    }

    pub fn unlock(&mut self, master_password: &str) -> Result<(), VaultError> {
        let wrapping_key = Zeroizing::new(
            derive_key(master_password, &self.metadata.salt, &self.metadata.kdf)
                .map_err(|_| VaultError::InvalidData)?,
        );
        let plaintext = Zeroizing::new(
            decrypt(
                &wrapping_key,
                &self.metadata.wrapped_key,
                vault_key_aad(&self.vault_id).as_bytes(),
            )
            .map_err(|_| VaultError::InvalidData)?,
        );
        self.key = Some(Zeroizing::new(
            plaintext
                .as_slice()
                .try_into()
                .map_err(|_| VaultError::InvalidData)?,
        ));
        Ok(())
    }

    pub fn lock(&mut self) {
        self.key = None;
    }

    pub fn is_unlocked(&self) -> bool {
        self.key.is_some()
    }

    pub fn store_hide_my_email_alias(&mut self, alias: HideMyEmailAlias) -> Result<(), VaultError> {
        let existing = self
            .decrypted_items()?
            .into_iter()
            .find_map(|(id, item)| match item {
                VaultItem::EmailAlias(current) if current.provider_id == alias.provider_id => {
                    Some((id, current))
                }
                _ => None,
            });
        if existing
            .as_ref()
            .is_some_and(|(_, current)| current == &alias)
        {
            return Ok(());
        }

        let key = self.key.as_ref().ok_or(VaultError::Locked)?;
        let (id, revision) = existing
            .map(|(id, _)| {
                self.records[&id]
                    .revision
                    .checked_add(1)
                    .map(|revision| (id, revision))
                    .ok_or(VaultError::InvalidData)
            })
            .transpose()?
            .unwrap_or_else(|| (Uuid::new_v4().to_string(), 1));
        let plaintext = Zeroizing::new(
            serde_json::to_vec(&ItemPayload {
                schema_version: 1,
                item: VaultItem::EmailAlias(alias),
            })
            .map_err(|_| VaultError::InvalidData)?,
        );
        let envelope = encrypt(key, &plaintext, item_aad(&self.vault_id, &id).as_bytes())
            .map_err(|_| VaultError::InvalidData)?;
        self.records.insert(
            id.clone(),
            EncryptedRecord {
                id,
                revision,
                updated_at: now(),
                envelope,
            },
        );
        Ok(())
    }

    pub fn list_hide_my_email_aliases(&self) -> Result<Vec<HideMyEmailAlias>, VaultError> {
        Ok(self
            .list_hide_my_email_alias_records()?
            .into_iter()
            .map(|(_, alias)| alias)
            .collect())
    }

    pub fn list_hide_my_email_alias_records(
        &self,
    ) -> Result<Vec<(String, HideMyEmailAlias)>, VaultError> {
        Ok(self
            .decrypted_items()?
            .into_iter()
            .filter_map(|(id, item)| match item {
                VaultItem::EmailAlias(alias) => Some((id, alias)),
            })
            .collect())
    }

    pub fn apply_hide_my_email_aliases(
        &mut self,
        aliases: Vec<HideMyEmailAlias>,
    ) -> Result<(), VaultError> {
        let mut provider_ids = BTreeMap::new();
        if aliases.iter().any(|alias| {
            alias.provider_id.trim().is_empty()
                || alias.address.trim().is_empty()
                || provider_ids.insert(&alias.provider_id, ()).is_some()
        }) {
            return Err(VaultError::InvalidData);
        }

        let original = self.records.clone();
        for alias in aliases {
            if let Err(error) = self.store_hide_my_email_alias(alias) {
                self.records = original;
                return Err(error);
            }
        }
        Ok(())
    }

    pub fn remove_all_hide_my_email_aliases(&mut self) -> Result<usize, VaultError> {
        let ids: Vec<_> = self
            .decrypted_items()?
            .into_iter()
            .filter_map(|(id, item)| matches!(item, VaultItem::EmailAlias(_)).then_some(id))
            .collect();
        for id in &ids {
            self.records.remove(id);
        }
        Ok(ids.len())
    }

    #[cfg(test)]
    pub(crate) fn upload(&self, store: &mut impl SyncStore) -> Result<(), VaultError> {
        store.put(self.bundle());
        Ok(())
    }

    fn bundle(&self) -> SyncBundle {
        SyncBundle {
            schema_version: 1,
            vault_id: self.vault_id.clone(),
            metadata: self.metadata.clone(),
            records: self.records.values().cloned().collect(),
        }
    }

    #[cfg(test)]
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    #[cfg(test)]
    pub fn set_first_revision(&mut self, revision: u64) -> Result<(), VaultError> {
        self.records
            .values_mut()
            .next()
            .ok_or(VaultError::NotFound)?
            .revision = revision;
        Ok(())
    }

    fn decrypted_items(&self) -> Result<Vec<(String, VaultItem)>, VaultError> {
        let key = self.key.as_ref().ok_or(VaultError::Locked)?;
        self.records
            .values()
            .map(|record| {
                let plaintext = Zeroizing::new(
                    decrypt(
                        key,
                        &record.envelope,
                        item_aad(&self.vault_id, &record.id).as_bytes(),
                    )
                    .map_err(|_| VaultError::InvalidData)?,
                );
                let payload: ItemPayload =
                    serde_json::from_slice(&plaintext).map_err(|_| VaultError::InvalidData)?;
                if payload.schema_version != 1 {
                    return Err(VaultError::InvalidData);
                }
                Ok((record.id.clone(), payload.item))
            })
            .collect()
    }
}

fn vault_key_aad(vault_id: &str) -> String {
    format!("laojie-river:vault-key:v1:{vault_id}")
}

fn item_aad(vault_id: &str, item_id: &str) -> String {
    format!("laojie-river:item:v1:{vault_id}:{item_id}")
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod persistence_tests {
    use super::*;

    fn alias(provider_id: &str, label: &str) -> HideMyEmailAlias {
        HideMyEmailAlias {
            provider_id: provider_id.into(),
            address: format!("{provider_id}@privaterelay.appleid.com"),
            forwarding_address: None,
            label: Some(label.into()),
            note: None,
            origin: None,
            is_active: true,
        }
    }

    #[test]
    fn alias_batch_rolls_back_and_records_use_random_ids() {
        let original = alias("existing", "Original");
        let mut vault = Vault::create("vault-1", "correct horse battery staple").unwrap();
        vault.store_hide_my_email_alias(original.clone()).unwrap();
        let (item_id, _) = vault.list_hide_my_email_alias_records().unwrap().remove(0);
        assert_ne!(item_id, original.provider_id);

        vault.set_first_revision(u64::MAX).unwrap();
        assert_eq!(
            vault.apply_hide_my_email_aliases(vec![
                alias("new", "New"),
                alias("existing", "Updated"),
            ]),
            Err(VaultError::InvalidData)
        );
        assert_eq!(vault.list_hide_my_email_aliases().unwrap(), vec![original]);
        assert_eq!(vault.remove_all_hide_my_email_aliases().unwrap(), 1);
        assert!(vault.list_hide_my_email_aliases().unwrap().is_empty());
    }
}
