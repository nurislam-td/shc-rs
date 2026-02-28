use aes_gcm::{
    aead::{Aead, AeadCore, OsRng}, Aes256Gcm, KeyInit,
    Nonce,
};
use argon2::{password_hash::SaltString, Argon2};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoServiceError {
    #[error("random number generator failed: {0}")]
    EntropyError(String),

    #[error("Key derivation function failed: {0}")]
    KeyDerivationError(String),

    #[error("Encoding error: {0}")]
    EncodingError(String),

    #[error("Decoding error: {0}")]
    DecodingError(String),

    #[error("Invalid lengths of parameters: {0}")]
    InvalidLengthError(String),
}

type CryptoServiceResult<T> = Result<T, CryptoServiceError>;

pub struct EncryptedBundle {
    pub ciphertext: Vec<u8>,
    salt: String,
    nonce: Vec<u8>,
}

fn derive_key(passphrase: &str, salt: &str) -> CryptoServiceResult<[u8; 32]> {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(passphrase.as_bytes(), salt.as_bytes(), &mut key)
        .map_err(|e| CryptoServiceError::KeyDerivationError(e.to_string()))?;
    Ok(key)
}

pub fn encrypt(password: &str, passphrase: &str) -> CryptoServiceResult<EncryptedBundle> {
    let salt = SaltString::generate(&mut OsRng);
    let key = derive_key(passphrase, &salt.as_str())?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| CryptoServiceError::EncodingError(e.to_string()))?;
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, password.as_bytes())
        .map_err(|e| CryptoServiceError::EncodingError(e.to_string()))?;

    Ok(EncryptedBundle {
        ciphertext,
        salt: salt.to_string(),
        nonce: nonce.to_vec(),
    })
}

pub fn decrypt(bundle: &EncryptedBundle, passprhase: &str) -> CryptoServiceResult<String> {
    let key = derive_key(passprhase, &bundle.salt.as_str())?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| CryptoServiceError::DecodingError(e.to_string()))?;
    let nonce = Nonce::from_slice(&bundle.nonce);

    let decrypted_bytes = cipher
        .decrypt(nonce, bundle.ciphertext.as_ref())
        .map_err(|e| CryptoServiceError::DecodingError(e.to_string()))?;
    String::from_utf8(decrypted_bytes).map_err(|e| CryptoServiceError::EncodingError(e.to_string()))
}
