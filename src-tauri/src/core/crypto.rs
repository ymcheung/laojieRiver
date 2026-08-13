use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::{
    aead::{Aead, Payload},
    KeyInit, XChaCha20Poly1305, XNonce,
};
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const KEY_LEN: usize = 32;
const NONCE_LEN: usize = 24;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KdfParams {
    pub version: u16,
    pub memory_kib: u32,
    pub iterations: u32,
    pub parallelism: u32,
}

impl Default for KdfParams {
    fn default() -> Self {
        Self {
            version: 1,
            memory_kib: 64 * 1024,
            iterations: 3,
            parallelism: 1,
        }
    }
}

impl KdfParams {
    pub fn is_supported(&self) -> bool {
        self.version == 1
            && self.memory_kib == 64 * 1024
            && self.iterations == 3
            && self.parallelism == 1
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedEnvelope {
    pub version: u16,
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CryptoError {
    #[error("Unsupported encrypted data version.")]
    UnsupportedVersion,
    #[error("Encrypted data is invalid.")]
    InvalidData,
}

pub fn random_bytes<const N: usize>() -> [u8; N] {
    let mut bytes = [0; N];
    OsRng.fill_bytes(&mut bytes);
    bytes
}

pub fn derive_key(
    password: &str,
    salt: &[u8],
    params: &KdfParams,
) -> Result<[u8; KEY_LEN], CryptoError> {
    if !params.is_supported() {
        return Err(CryptoError::UnsupportedVersion);
    }
    let mut key = [0; KEY_LEN];
    let argon_params = Params::new(
        params.memory_kib,
        params.iterations,
        params.parallelism,
        Some(KEY_LEN),
    )
    .map_err(|_| CryptoError::InvalidData)?;
    Argon2::new(Algorithm::Argon2id, Version::V0x13, argon_params)
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|_| CryptoError::InvalidData)?;
    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authenticated_envelope_rejects_wrong_keys_tampering_and_versions() {
        let key = random_bytes::<KEY_LEN>();
        let other_key = random_bytes::<KEY_LEN>();
        let mut envelope = encrypt(&key, b"secret", b"item-1").unwrap();
        let second = encrypt(&key, b"secret", b"item-1").unwrap();

        assert_eq!(decrypt(&key, &envelope, b"item-1").unwrap(), b"secret");
        assert_ne!(envelope.nonce, second.nonce);
        assert_eq!(
            decrypt(&other_key, &envelope, b"item-1"),
            Err(CryptoError::InvalidData)
        );
        envelope.ciphertext[0] ^= 1;
        assert_eq!(
            decrypt(&key, &envelope, b"item-1"),
            Err(CryptoError::InvalidData)
        );
        envelope.version = 2;
        assert_eq!(
            decrypt(&key, &envelope, b"item-1"),
            Err(CryptoError::UnsupportedVersion)
        );
    }
}

pub fn encrypt(
    key: &[u8; KEY_LEN],
    plaintext: &[u8],
    associated_data: &[u8],
) -> Result<EncryptedEnvelope, CryptoError> {
    let nonce = random_bytes::<NONCE_LEN>();
    let cipher = XChaCha20Poly1305::new(key.into());
    let ciphertext = cipher
        .encrypt(
            XNonce::from_slice(&nonce),
            Payload {
                msg: plaintext,
                aad: associated_data,
            },
        )
        .map_err(|_| CryptoError::InvalidData)?;

    Ok(EncryptedEnvelope {
        version: 1,
        nonce: nonce.to_vec(),
        ciphertext,
    })
}

pub fn decrypt(
    key: &[u8; KEY_LEN],
    envelope: &EncryptedEnvelope,
    associated_data: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    if envelope.version != 1 {
        return Err(CryptoError::UnsupportedVersion);
    }
    if envelope.nonce.len() != NONCE_LEN {
        return Err(CryptoError::InvalidData);
    }

    XChaCha20Poly1305::new(key.into())
        .decrypt(
            XNonce::from_slice(&envelope.nonce),
            Payload {
                msg: &envelope.ciphertext,
                aad: associated_data,
            },
        )
        .map_err(|_| CryptoError::InvalidData)
}
