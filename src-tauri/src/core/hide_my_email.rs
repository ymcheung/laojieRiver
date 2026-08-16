use std::collections::{BTreeMap, HashSet};

use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

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

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsDestination {
    pub id: String,
    pub masked_number: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum AuthState {
    Disconnected,
    VerificationRequired {
        trusted_device_available: bool,
        sms_destinations: Vec<SmsDestination>,
    },
    SmsCodeRequired {
        masked_number: String,
    },
    Connected {
        masked_account: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HideMyEmailError {
    InvalidCredentials,
    InvalidVerificationCode,
    SessionExpired,
    SecurityKeyUnsupported,
    InvalidResponse,
    NotConnected,
    VerificationNotPending,
    ServiceUnavailable,
}

impl std::fmt::Display for HideMyEmailError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::InvalidCredentials => "Apple Account sign-in failed.",
            Self::InvalidVerificationCode => "The verification code is invalid.",
            Self::SessionExpired => "The Apple Account session has expired.",
            Self::SecurityKeyUnsupported => "Security-key Apple Accounts are not supported.",
            Self::InvalidResponse => "Apple returned an invalid response.",
            Self::NotConnected => "Connect an Apple Account first.",
            Self::VerificationNotPending => "No verification is pending.",
            Self::ServiceUnavailable => "Apple's Hide My Email service is unavailable.",
        })
    }
}

impl std::error::Error for HideMyEmailError {}

pub struct SessionRestore {
    pub account_identifier: Zeroizing<String>,
    pub session: Zeroizing<String>,
}

pub struct AuthSession {
    pub session: Zeroizing<String>,
}

pub enum SignInResponse {
    Connected(AuthSession),
    VerificationRequired {
        context: Zeroizing<String>,
        trusted_device_available: bool,
        sms_destinations: Vec<SmsDestination>,
    },
    SecurityKeyRequired,
}

pub trait AppleTransport {
    fn sign_in(
        &mut self,
        account_identifier: &str,
        password: &str,
    ) -> Result<SignInResponse, HideMyEmailError>;
    fn verify_trusted_device(
        &mut self,
        context: &str,
        code: &str,
    ) -> Result<AuthSession, HideMyEmailError>;
    fn request_sms(&mut self, context: &str, destination_id: &str) -> Result<(), HideMyEmailError>;
    fn verify_sms(
        &mut self,
        context: &str,
        destination_id: &str,
        code: &str,
    ) -> Result<AuthSession, HideMyEmailError>;
    fn restore_session(&mut self, session: &str) -> Result<(), HideMyEmailError>;
    fn list_aliases(&mut self, session: &str) -> Result<Vec<HideMyEmailAlias>, HideMyEmailError>;
}

#[derive(Default)]
pub struct UnavailableAppleTransport;

impl AppleTransport for UnavailableAppleTransport {
    fn sign_in(&mut self, _: &str, _: &str) -> Result<SignInResponse, HideMyEmailError> {
        Err(HideMyEmailError::ServiceUnavailable)
    }

    fn verify_trusted_device(&mut self, _: &str, _: &str) -> Result<AuthSession, HideMyEmailError> {
        Err(HideMyEmailError::ServiceUnavailable)
    }

    fn request_sms(&mut self, _: &str, _: &str) -> Result<(), HideMyEmailError> {
        Err(HideMyEmailError::ServiceUnavailable)
    }

    fn verify_sms(&mut self, _: &str, _: &str, _: &str) -> Result<AuthSession, HideMyEmailError> {
        Err(HideMyEmailError::ServiceUnavailable)
    }

    fn restore_session(&mut self, _: &str) -> Result<(), HideMyEmailError> {
        Err(HideMyEmailError::SessionExpired)
    }

    fn list_aliases(&mut self, _: &str) -> Result<Vec<HideMyEmailAlias>, HideMyEmailError> {
        Err(HideMyEmailError::ServiceUnavailable)
    }
}

struct PendingVerification {
    context: Zeroizing<String>,
    sms_destinations: Vec<SmsDestination>,
    selected_sms_id: Option<String>,
}

/// Owns authentication state while leaving protocol HTTP/SRP details behind one transport seam.
pub struct AppleHideMyEmailProvider<T> {
    transport: T,
    account_identifier: Option<Zeroizing<String>>,
    session: Option<Zeroizing<String>>,
    pending: Option<PendingVerification>,
    state: AuthState,
}

impl<T: AppleTransport> AppleHideMyEmailProvider<T> {
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            account_identifier: None,
            session: None,
            pending: None,
            state: AuthState::Disconnected,
        }
    }

    pub fn state(&self) -> &AuthState {
        &self.state
    }

    pub fn start_connect(
        &mut self,
        account_identifier: &str,
        password: &str,
    ) -> Result<&AuthState, HideMyEmailError> {
        if account_identifier.trim().is_empty() || password.is_empty() {
            return Err(HideMyEmailError::InvalidCredentials);
        }
        self.clear_authentication();
        let response = self
            .transport
            .sign_in(account_identifier.trim(), password)?;
        self.account_identifier = Some(Zeroizing::new(account_identifier.trim().to_owned()));
        self.accept_sign_in(response)?;
        Ok(&self.state)
    }

    pub fn submit_trusted_device_code(
        &mut self,
        code: &str,
    ) -> Result<&AuthState, HideMyEmailError> {
        validate_code(code)?;
        let pending = self
            .pending
            .as_ref()
            .ok_or(HideMyEmailError::VerificationNotPending)?;
        let session = self
            .transport
            .verify_trusted_device(&pending.context, code)?;
        self.connect(session);
        Ok(&self.state)
    }

    pub fn request_sms_code(
        &mut self,
        destination_id: &str,
    ) -> Result<&AuthState, HideMyEmailError> {
        let pending = self
            .pending
            .as_mut()
            .ok_or(HideMyEmailError::VerificationNotPending)?;
        let destination = pending
            .sms_destinations
            .iter()
            .find(|destination| destination.id == destination_id)
            .ok_or(HideMyEmailError::InvalidResponse)?;
        self.transport
            .request_sms(&pending.context, destination_id)?;
        pending.selected_sms_id = Some(destination_id.to_owned());
        self.state = AuthState::SmsCodeRequired {
            masked_number: destination.masked_number.clone(),
        };
        Ok(&self.state)
    }

    pub fn submit_sms_code(&mut self, code: &str) -> Result<&AuthState, HideMyEmailError> {
        validate_code(code)?;
        let pending = self
            .pending
            .as_ref()
            .ok_or(HideMyEmailError::VerificationNotPending)?;
        let destination_id = pending
            .selected_sms_id
            .as_deref()
            .ok_or(HideMyEmailError::VerificationNotPending)?;
        let session = self
            .transport
            .verify_sms(&pending.context, destination_id, code)?;
        self.connect(session);
        Ok(&self.state)
    }

    pub fn restore(&mut self, restore: SessionRestore) -> Result<&AuthState, HideMyEmailError> {
        if restore.account_identifier.trim().is_empty() || restore.session.is_empty() {
            return Err(HideMyEmailError::InvalidResponse);
        }
        self.clear_authentication();
        self.transport.restore_session(&restore.session)?;
        self.account_identifier = Some(restore.account_identifier);
        self.connect(AuthSession {
            session: restore.session,
        });
        Ok(&self.state)
    }

    pub fn list(&mut self) -> Result<Vec<HideMyEmailAlias>, HideMyEmailError> {
        let session = self
            .session
            .as_ref()
            .ok_or(HideMyEmailError::NotConnected)?;
        match self.transport.list_aliases(session) {
            Ok(aliases) => normalize_aliases(aliases),
            Err(HideMyEmailError::SessionExpired) => {
                self.clear_authentication();
                Err(HideMyEmailError::SessionExpired)
            }
            Err(error) => Err(error),
        }
    }

    /// Returns protected material for native Keychain persistence; never serialize this to Svelte.
    pub(crate) fn session_for_keychain(&self) -> Option<SessionRestore> {
        Some(SessionRestore {
            account_identifier: Zeroizing::new(self.account_identifier.as_ref()?.to_string()),
            session: Zeroizing::new(self.session.as_ref()?.to_string()),
        })
    }

    pub fn disconnect(&mut self) {
        self.clear_authentication();
    }

    pub fn into_transport(self) -> T {
        self.transport
    }

    fn accept_sign_in(&mut self, response: SignInResponse) -> Result<(), HideMyEmailError> {
        match response {
            SignInResponse::Connected(session) => self.connect(session),
            SignInResponse::VerificationRequired {
                context,
                trusted_device_available,
                sms_destinations,
            } => {
                validate_destinations(&sms_destinations)?;
                self.state = AuthState::VerificationRequired {
                    trusted_device_available,
                    sms_destinations: sms_destinations.clone(),
                };
                self.pending = Some(PendingVerification {
                    context,
                    sms_destinations,
                    selected_sms_id: None,
                });
            }
            SignInResponse::SecurityKeyRequired => {
                self.clear_authentication();
                return Err(HideMyEmailError::SecurityKeyUnsupported);
            }
        }
        Ok(())
    }

    fn connect(&mut self, session: AuthSession) {
        self.session = Some(session.session);
        self.pending = None;
        self.state = AuthState::Connected {
            masked_account: mask_account(
                self.account_identifier
                    .as_deref()
                    .map(String::as_str)
                    .unwrap_or_default(),
            ),
        };
    }

    fn clear_authentication(&mut self) {
        self.account_identifier = None;
        self.session = None;
        self.pending = None;
        self.state = AuthState::Disconnected;
    }
}

fn validate_code(code: &str) -> Result<(), HideMyEmailError> {
    if (4..=8).contains(&code.len()) && code.bytes().all(|byte| byte.is_ascii_digit()) {
        Ok(())
    } else {
        Err(HideMyEmailError::InvalidVerificationCode)
    }
}

fn validate_destinations(destinations: &[SmsDestination]) -> Result<(), HideMyEmailError> {
    let mut ids = HashSet::new();
    if destinations.iter().any(|destination| {
        destination.id.trim().is_empty()
            || destination.masked_number.trim().is_empty()
            || !ids.insert(&destination.id)
    }) {
        Err(HideMyEmailError::InvalidResponse)
    } else {
        Ok(())
    }
}

fn mask_account(account: &str) -> String {
    let Some((name, domain)) = account.split_once('@') else {
        return "•••".to_owned();
    };
    format!("{}•••@{domain}", name.chars().next().unwrap_or('•'))
}

pub fn normalize_aliases(
    aliases: Vec<HideMyEmailAlias>,
) -> Result<Vec<HideMyEmailAlias>, HideMyEmailError> {
    let mut normalized = BTreeMap::new();
    for mut alias in aliases {
        alias.provider_id = alias.provider_id.trim().to_owned();
        alias.address = alias.address.trim().to_ascii_lowercase();
        alias.forwarding_address = normalize_optional(alias.forwarding_address);
        alias.label = normalize_optional(alias.label);
        alias.note = normalize_optional(alias.note);
        alias.origin = normalize_optional(alias.origin);
        if !valid_alias(&alias) {
            return Err(HideMyEmailError::InvalidResponse);
        }
        match normalized.get(&alias.provider_id) {
            Some(existing) if existing != &alias => return Err(HideMyEmailError::InvalidResponse),
            _ => {
                normalized.insert(alias.provider_id.clone(), alias);
            }
        }
    }
    let mut aliases: Vec<_> = normalized.into_values().collect();
    aliases.sort_by(|left, right| {
        right
            .is_active
            .cmp(&left.is_active)
            .then_with(|| left.label.cmp(&right.label))
            .then_with(|| left.address.cmp(&right.address))
    });
    Ok(aliases)
}

fn normalize_optional(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let value = value.trim();
        (!value.is_empty()).then(|| value.to_owned())
    })
}

fn valid_alias(alias: &HideMyEmailAlias) -> bool {
    !alias.provider_id.is_empty()
        && alias.provider_id.len() <= 512
        && alias.address.len() <= 320
        && !alias.address.bytes().any(|byte| byte.is_ascii_control())
        && alias.address.split_once('@').is_some_and(|(name, domain)| {
            !name.is_empty() && !domain.is_empty() && !domain.contains('@')
        })
        && [
            alias.forwarding_address.as_deref(),
            alias.label.as_deref(),
            alias.note.as_deref(),
            alias.origin.as_deref(),
        ]
        .into_iter()
        .flatten()
        .all(|value| value.len() <= 4096 && !value.bytes().any(|byte| byte.is_ascii_control()))
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReconciliationCounts {
    pub added: usize,
    pub updated: usize,
    pub unchanged: usize,
    pub marked_inactive: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Reconciliation {
    pub aliases_to_store: Vec<HideMyEmailAlias>,
    pub counts: ReconciliationCounts,
}

/// Produces a complete validated write set before persistence begins. Missing aliases are retained.
pub fn reconcile_aliases(
    existing: &[HideMyEmailAlias],
    incoming: Vec<HideMyEmailAlias>,
) -> Result<Reconciliation, HideMyEmailError> {
    let incoming = normalize_aliases(incoming)?;
    let existing: BTreeMap<_, _> = existing
        .iter()
        .map(|alias| (alias.provider_id.as_str(), alias))
        .collect();
    let mut result = Reconciliation {
        aliases_to_store: Vec::new(),
        counts: ReconciliationCounts::default(),
    };
    for alias in incoming {
        match existing.get(alias.provider_id.as_str()) {
            None => {
                result.counts.added += 1;
                result.aliases_to_store.push(alias);
            }
            Some(current) if *current == &alias => result.counts.unchanged += 1,
            Some(current) => {
                result.counts.updated += 1;
                result.counts.marked_inactive += usize::from(current.is_active && !alias.is_active);
                result.aliases_to_store.push(alias);
            }
        }
    }
    Ok(result)
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
    let existing = vault.list_hide_my_email_aliases()?;
    let reconciliation = reconcile_aliases(&existing, provider.list_aliases()?)
        .map_err(|_| VaultError::InvalidData)?;
    let count = reconciliation.counts.added
        + reconciliation.counts.updated
        + reconciliation.counts.unchanged;
    for alias in reconciliation.aliases_to_store {
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
    use zeroize::Zeroizing;

    use crate::core::{
        hide_my_email::{
            reconcile_aliases, sync_hide_my_email, AppleHideMyEmailProvider, AppleTransport,
            AuthSession, AuthState, HideMyEmailAlias, HideMyEmailError, SessionRestore,
            SignInResponse, SmsDestination, StaticHideMyEmailProvider,
        },
        vault::{MemorySyncStore, Vault, VaultError},
    };

    struct FixtureTransport {
        sign_in: Option<Result<SignInResponse, HideMyEmailError>>,
        aliases: Result<Vec<HideMyEmailAlias>, HideMyEmailError>,
        restored: bool,
        requested_sms: Option<String>,
    }

    impl FixtureTransport {
        fn verification_required() -> Self {
            Self {
                sign_in: Some(Ok(SignInResponse::VerificationRequired {
                    context: Zeroizing::new("private-context".into()),
                    trusted_device_available: true,
                    sms_destinations: vec![SmsDestination {
                        id: "phone-1".into(),
                        masked_number: "••• ••• 1234".into(),
                    }],
                })),
                aliases: Ok(Vec::new()),
                restored: false,
                requested_sms: None,
            }
        }
    }

    impl AppleTransport for FixtureTransport {
        fn sign_in(
            &mut self,
            _account_identifier: &str,
            _password: &str,
        ) -> Result<SignInResponse, HideMyEmailError> {
            self.sign_in.take().unwrap()
        }

        fn verify_trusted_device(
            &mut self,
            _context: &str,
            code: &str,
        ) -> Result<AuthSession, HideMyEmailError> {
            (code == "123456")
                .then(|| AuthSession {
                    session: Zeroizing::new("trusted-session".into()),
                })
                .ok_or(HideMyEmailError::InvalidVerificationCode)
        }

        fn request_sms(
            &mut self,
            _context: &str,
            destination_id: &str,
        ) -> Result<(), HideMyEmailError> {
            self.requested_sms = Some(destination_id.into());
            Ok(())
        }

        fn verify_sms(
            &mut self,
            _context: &str,
            destination_id: &str,
            code: &str,
        ) -> Result<AuthSession, HideMyEmailError> {
            (destination_id == "phone-1" && code == "654321")
                .then(|| AuthSession {
                    session: Zeroizing::new("sms-session".into()),
                })
                .ok_or(HideMyEmailError::InvalidVerificationCode)
        }

        fn restore_session(&mut self, session: &str) -> Result<(), HideMyEmailError> {
            self.restored = session == "stored-session";
            self.restored
                .then_some(())
                .ok_or(HideMyEmailError::SessionExpired)
        }

        fn list_aliases(
            &mut self,
            _session: &str,
        ) -> Result<Vec<HideMyEmailAlias>, HideMyEmailError> {
            self.aliases.clone()
        }
    }

    fn alias(id: &str, label: &str, active: bool) -> HideMyEmailAlias {
        HideMyEmailAlias {
            provider_id: id.into(),
            address: format!("{id}@privaterelay.appleid.com"),
            forwarding_address: None,
            label: Some(label.into()),
            note: None,
            origin: None,
            is_active: active,
        }
    }

    #[test]
    fn auth_state_machine_supports_trusted_device_and_sms_fallback() {
        let mut provider = AppleHideMyEmailProvider::new(FixtureTransport::verification_required());
        assert!(matches!(
            provider.start_connect("owner@example.com", "not-retained"),
            Ok(AuthState::VerificationRequired { .. })
        ));
        assert!(matches!(
            provider.submit_trusted_device_code("12ab"),
            Err(HideMyEmailError::InvalidVerificationCode)
        ));
        assert_eq!(
            provider.submit_trusted_device_code("123456").unwrap(),
            &AuthState::Connected {
                masked_account: "o•••@example.com".into()
            }
        );
        assert!(provider.session_for_keychain().is_some());

        let mut provider = AppleHideMyEmailProvider::new(FixtureTransport::verification_required());
        provider
            .start_connect("owner@example.com", "also-not-retained")
            .unwrap();
        assert_eq!(
            provider.request_sms_code("phone-1").unwrap(),
            &AuthState::SmsCodeRequired {
                masked_number: "••• ••• 1234".into()
            }
        );
        assert!(matches!(
            provider.submit_sms_code("654321"),
            Ok(AuthState::Connected { .. })
        ));
        assert_eq!(
            provider.into_transport().requested_sms.as_deref(),
            Some("phone-1")
        );
    }

    #[test]
    fn restored_session_lists_only_normalized_valid_aliases() {
        let mut transport = FixtureTransport::verification_required();
        transport.aliases = Ok(vec![HideMyEmailAlias {
            provider_id: " alias-id ".into(),
            address: " QUIET@PrivateRelay.AppleID.com ".into(),
            forwarding_address: Some(" ".into()),
            label: Some(" Shopping ".into()),
            note: None,
            origin: Some(" shop.example ".into()),
            is_active: true,
        }]);
        let mut provider = AppleHideMyEmailProvider::new(transport);
        provider
            .restore(SessionRestore {
                account_identifier: Zeroizing::new("owner@example.com".into()),
                session: Zeroizing::new("stored-session".into()),
            })
            .unwrap();

        let aliases = provider.list().unwrap();
        assert_eq!(aliases[0].provider_id, "alias-id");
        assert_eq!(aliases[0].address, "quiet@privaterelay.appleid.com");
        assert_eq!(aliases[0].label.as_deref(), Some("Shopping"));
        assert_eq!(aliases[0].forwarding_address, None);
    }

    #[test]
    fn reconciliation_validates_everything_before_returning_a_write_set() {
        let existing = vec![alias("same", "Same", true), alias("old", "Old", true)];
        let mut updated = alias("old", "Updated", false);
        updated.address = "OLD@privaterelay.appleid.com".into();
        let result = reconcile_aliases(
            &existing,
            vec![
                alias("same", "Same", true),
                updated,
                alias("new", "New", true),
            ],
        )
        .unwrap();

        assert_eq!(result.counts.added, 1);
        assert_eq!(result.counts.updated, 1);
        assert_eq!(result.counts.unchanged, 1);
        assert_eq!(result.counts.marked_inactive, 1);
        assert_eq!(result.aliases_to_store.len(), 2);

        let invalid = HideMyEmailAlias {
            address: "not-an-email".into(),
            ..alias("bad", "Bad", true)
        };
        assert_eq!(
            reconcile_aliases(&existing, vec![alias("new", "New", true), invalid]),
            Err(HideMyEmailError::InvalidResponse)
        );
    }

    #[test]
    fn provider_errors_are_safe_to_display() {
        assert_eq!(
            HideMyEmailError::InvalidCredentials.to_string(),
            "Apple Account sign-in failed."
        );
        assert!(!HideMyEmailError::InvalidCredentials
            .to_string()
            .contains("owner@example.com"));
    }

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
