use thiserror::Error;
use zeroize::Zeroizing;

const SERVICE: &str = "com.laojieriver.app.icloud-session";
const MAX_SESSION_BYTES: usize = 256 * 1024;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum KeychainError {
    #[error("Protected session data is invalid.")]
    InvalidData,
    #[error("macOS Keychain is unavailable.")]
    Unavailable,
    #[cfg(not(target_os = "macos"))]
    #[error("Protected session storage is unsupported on this platform.")]
    Unsupported,
}

pub trait ProtectedSessionStore {
    fn load(&self, vault_id: &str) -> Result<Option<Zeroizing<String>>, KeychainError>;
    fn store(&self, vault_id: &str, session: &str) -> Result<(), KeychainError>;
    fn delete(&self, vault_id: &str) -> Result<(), KeychainError>;
}

#[derive(Default)]
pub struct KeychainSessionStore;

impl ProtectedSessionStore for KeychainSessionStore {
    fn load(&self, vault_id: &str) -> Result<Option<Zeroizing<String>>, KeychainError> {
        validate_vault_id(vault_id)?;
        load(vault_id)
    }

    fn store(&self, vault_id: &str, session: &str) -> Result<(), KeychainError> {
        validate_vault_id(vault_id)?;
        if session.is_empty() || session.len() > MAX_SESSION_BYTES {
            return Err(KeychainError::InvalidData);
        }
        store(vault_id, session)
    }

    fn delete(&self, vault_id: &str) -> Result<(), KeychainError> {
        validate_vault_id(vault_id)?;
        delete(vault_id)
    }
}

fn validate_vault_id(vault_id: &str) -> Result<(), KeychainError> {
    if vault_id.is_empty()
        || vault_id.len() > 128
        || !vault_id.bytes().all(|b| b.is_ascii_graphic())
    {
        return Err(KeychainError::InvalidData);
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn entry(vault_id: &str) -> Result<keyring::Entry, KeychainError> {
    keyring::Entry::new(SERVICE, vault_id).map_err(|_| KeychainError::Unavailable)
}

#[cfg(target_os = "macos")]
fn load(vault_id: &str) -> Result<Option<Zeroizing<String>>, KeychainError> {
    match entry(vault_id)?.get_password() {
        Ok(session) => Ok(Some(Zeroizing::new(session))),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(_) => Err(KeychainError::Unavailable),
    }
}

#[cfg(not(target_os = "macos"))]
fn load(_: &str) -> Result<Option<Zeroizing<String>>, KeychainError> {
    Err(KeychainError::Unsupported)
}

#[cfg(target_os = "macos")]
fn store(vault_id: &str, session: &str) -> Result<(), KeychainError> {
    entry(vault_id)?
        .set_password(session)
        .map_err(|_| KeychainError::Unavailable)
}

#[cfg(not(target_os = "macos"))]
fn store(_: &str, _: &str) -> Result<(), KeychainError> {
    Err(KeychainError::Unsupported)
}

#[cfg(target_os = "macos")]
fn delete(vault_id: &str) -> Result<(), KeychainError> {
    match entry(vault_id)?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(_) => Err(KeychainError::Unavailable),
    }
}

#[cfg(not(target_os = "macos"))]
fn delete(_: &str) -> Result<(), KeychainError> {
    Err(KeychainError::Unsupported)
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, collections::BTreeMap};

    use super::*;

    #[derive(Default)]
    struct MemorySessionStore(RefCell<BTreeMap<String, String>>);

    impl ProtectedSessionStore for MemorySessionStore {
        fn load(&self, vault_id: &str) -> Result<Option<Zeroizing<String>>, KeychainError> {
            Ok(self.0.borrow().get(vault_id).cloned().map(Zeroizing::new))
        }

        fn store(&self, vault_id: &str, session: &str) -> Result<(), KeychainError> {
            self.0.borrow_mut().insert(vault_id.into(), session.into());
            Ok(())
        }

        fn delete(&self, vault_id: &str) -> Result<(), KeychainError> {
            self.0.borrow_mut().remove(vault_id);
            Ok(())
        }
    }

    #[test]
    fn protected_session_contract_supports_create_overwrite_and_delete() {
        let store = MemorySessionStore::default();

        assert!(store.load("vault-1").unwrap().is_none());
        store.store("vault-1", "session-one").unwrap();
        store.store("vault-1", "session-two").unwrap();
        let session = store.load("vault-1").unwrap().unwrap();
        assert_eq!(&**session, "session-two");
        store.delete("vault-1").unwrap();
        assert!(store.load("vault-1").unwrap().is_none());
    }

    #[test]
    fn keychain_rejects_invalid_keys_and_session_values_before_platform_access() {
        let store = KeychainSessionStore;

        assert_eq!(store.load(""), Err(KeychainError::InvalidData));
        assert_eq!(store.store("vault-1", ""), Err(KeychainError::InvalidData));
        assert_eq!(
            store.store("vault-1", &"x".repeat(MAX_SESSION_BYTES + 1)),
            Err(KeychainError::InvalidData)
        );
    }
}
