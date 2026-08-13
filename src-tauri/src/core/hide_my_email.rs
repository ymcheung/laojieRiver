use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::core::vault::{Vault, VaultError};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HideMyEmailAlias {
    pub provider_id: String,
    pub address: String,
    pub forwarding_address: Option<String>,
    pub label: Option<String>,
    pub note: Option<String>,
    pub origin: Option<String>,
    pub is_active: bool,
}

#[cfg(test)]
pub trait HideMyEmailProvider {
    fn list_aliases(&self) -> Result<Vec<HideMyEmailAlias>, VaultError>;
}

#[cfg(test)]
pub fn sync_hide_my_email(
    vault: &mut Vault,
    provider: &impl HideMyEmailProvider,
) -> Result<usize, VaultError> {
    let aliases = provider.list_aliases()?;
    if aliases
        .iter()
        .any(|alias| alias.provider_id.trim().is_empty() || alias.address.trim().is_empty())
    {
        return Err(VaultError::InvalidData);
    }
    let count = aliases.len();
    for alias in aliases {
        vault.store_hide_my_email_alias(alias)?;
    }
    Ok(count)
}

#[cfg(test)]
pub struct StaticHideMyEmailProvider(Vec<HideMyEmailAlias>);

#[cfg(test)]
impl StaticHideMyEmailProvider {
    fn new(aliases: Vec<HideMyEmailAlias>) -> Self {
        Self(aliases)
    }
}

#[cfg(test)]
impl HideMyEmailProvider for StaticHideMyEmailProvider {
    fn list_aliases(&self) -> Result<Vec<HideMyEmailAlias>, VaultError> {
        Ok(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{
        hide_my_email::{sync_hide_my_email, HideMyEmailAlias, StaticHideMyEmailProvider},
        vault::{MemorySyncStore, Vault, VaultError},
    };

    #[test]
    fn fake_alias_round_trips_between_devices_as_ciphertext_only() {
        let alias = HideMyEmailAlias {
            provider_id: "apple-private-id".into(),
            address: "quiet-river@privaterelay.appleid.com".into(),
            forwarding_address: Some("owner@example.com".into()),
            label: Some("購物".into()),
            note: Some("private note".into()),
            origin: Some("shop.example".into()),
            is_active: true,
        };
        let provider = StaticHideMyEmailProvider::new(vec![alias.clone()]);
        let mut store = MemorySyncStore::default();
        let mut device_a = Vault::create("user-1", "correct horse battery staple").unwrap();

        sync_hide_my_email(&mut device_a, &provider).unwrap();
        device_a.upload(&mut store).unwrap();

        let payload = store.wire_bytes("user-1").unwrap();
        for secret in [
            "apple-private-id",
            "quiet-river@privaterelay.appleid.com",
            "owner@example.com",
            "購物",
            "private note",
            "shop.example",
        ] {
            assert!(!payload
                .windows(secret.len())
                .any(|part| part == secret.as_bytes()));
        }

        let mut device_b = Vault::download("user-1", &store).unwrap();
        assert_eq!(
            device_b.list_hide_my_email_aliases(),
            Err(VaultError::Locked)
        );
        assert_eq!(
            device_b.unlock("wrong master password"),
            Err(VaultError::InvalidData)
        );
        device_b.unlock("correct horse battery staple").unwrap();
        assert_eq!(device_b.list_hide_my_email_aliases().unwrap(), vec![alias]);
    }

    #[test]
    fn tampered_sync_data_fails_closed() {
        let provider = StaticHideMyEmailProvider::new(vec![HideMyEmailAlias {
            provider_id: "provider-id".into(),
            address: "alias@example.com".into(),
            forwarding_address: None,
            label: None,
            note: None,
            origin: None,
            is_active: true,
        }]);
        let mut store = MemorySyncStore::default();
        let mut device_a = Vault::create("user-1", "correct horse battery staple").unwrap();
        sync_hide_my_email(&mut device_a, &provider).unwrap();
        device_a.upload(&mut store).unwrap();
        store.tamper_first_ciphertext("user-1").unwrap();

        let mut device_b = Vault::download("user-1", &store).unwrap();
        device_b.unlock("correct horse battery staple").unwrap();
        assert_eq!(
            device_b.list_hide_my_email_aliases(),
            Err(VaultError::InvalidData)
        );
    }

    #[test]
    fn refresh_updates_an_alias_without_duplicating_it() {
        let original = HideMyEmailAlias {
            provider_id: "provider-id".into(),
            address: "alias@example.com".into(),
            forwarding_address: None,
            label: Some("Old".into()),
            note: None,
            origin: None,
            is_active: true,
        };
        let mut updated = original.clone();
        updated.label = Some("New".into());
        let mut vault = Vault::create("user-1", "correct horse battery staple").unwrap();

        sync_hide_my_email(&mut vault, &StaticHideMyEmailProvider::new(vec![original])).unwrap();
        sync_hide_my_email(
            &mut vault,
            &StaticHideMyEmailProvider::new(vec![updated.clone()]),
        )
        .unwrap();

        assert_eq!(vault.list_hide_my_email_aliases().unwrap(), vec![updated]);
        assert_eq!(vault.record_count(), 1);
    }

    #[test]
    fn hostile_kdf_parameters_are_rejected_before_unlock() {
        let mut store = MemorySyncStore::default();
        Vault::create("user-1", "correct horse battery staple")
            .unwrap()
            .upload(&mut store)
            .unwrap();
        store.set_kdf_memory("user-1", u32::MAX).unwrap();

        assert!(matches!(
            Vault::download("user-1", &store),
            Err(VaultError::InvalidData)
        ));
    }

    #[test]
    fn invalid_provider_aliases_are_rejected() {
        let mut vault = Vault::create("user-1", "correct horse battery staple").unwrap();
        let provider = StaticHideMyEmailProvider::new(vec![HideMyEmailAlias {
            provider_id: " ".into(),
            address: "alias@example.com".into(),
            forwarding_address: None,
            label: None,
            note: None,
            origin: None,
            is_active: true,
        }]);

        assert_eq!(
            sync_hide_my_email(&mut vault, &provider),
            Err(VaultError::InvalidData)
        );
    }

    #[test]
    fn hostile_revision_overflow_fails_closed() {
        let alias = HideMyEmailAlias {
            provider_id: "provider-id".into(),
            address: "alias@example.com".into(),
            forwarding_address: None,
            label: None,
            note: None,
            origin: None,
            is_active: true,
        };
        let mut updated = alias.clone();
        updated.label = Some("updated".into());
        let mut vault = Vault::create("user-1", "correct horse battery staple").unwrap();
        sync_hide_my_email(&mut vault, &StaticHideMyEmailProvider::new(vec![alias])).unwrap();
        vault.set_first_revision(u64::MAX).unwrap();

        assert_eq!(
            sync_hide_my_email(&mut vault, &StaticHideMyEmailProvider::new(vec![updated])),
            Err(VaultError::InvalidData)
        );
    }
}
