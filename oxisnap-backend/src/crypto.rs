use std::path::Path;

use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};

use crate::error::BackendError;

pub fn load_or_create_key(path: &Path) -> Result<[u8; 32], std::io::Error> {
    if path.exists() {
        let bytes = std::fs::read(path)?;
        bytes.try_into().map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "key file must be exactly 32 bytes",
            )
        })
    } else {
        let key: [u8; 32] = Aes256Gcm::generate_key(OsRng).into();
        std::fs::write(path, key)?;
        Ok(key)
    }
}

pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<(Vec<u8>, [u8; 12]), BackendError> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Aes256Gcm::generate_nonce(OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_| BackendError::Encryption)?;
    Ok((ciphertext, nonce.into()))
}

pub fn decrypt(
    key: &[u8; 32],
    ciphertext: &[u8],
    nonce_bytes: &[u8; 12],
) -> Result<Vec<u8>, BackendError> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| BackendError::Encryption)
}
