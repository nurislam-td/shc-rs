use aes_gcm::{
    aead::{Aead, AeadCore, OsRng}, Aes256Gcm, KeyInit,
    Nonce,
};
use argon2::{password_hash::SaltString, Argon2};

use crate::service::crypt::{errors::CryptoServiceError, types::CryptoServiceResult};

fn derive_key(passphrase: &str, salt: &str) -> CryptoServiceResult<[u8; 32]> {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(passphrase.as_bytes(), salt.as_bytes(), &mut key)
        .map_err(|e| CryptoServiceError::KeyDerivationError(e.to_string()))?;
    Ok(key)
}

pub fn encrypt(password: &str, passphrase: &str) -> CryptoServiceResult<Vec<u8>> {
    let salt = SaltString::generate(&mut OsRng);
    let key = derive_key(passphrase, &salt.as_str())?;

    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| CryptoServiceError::EncodingError(e.to_string()))?;

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, password.as_bytes())
        .map_err(|e| CryptoServiceError::EncodingError(e.to_string()))?;

    let salt_bytes = salt.as_str().as_bytes();
    let mut blob = Vec::new();
    blob.push(salt_bytes.len() as u8);
    blob.extend_from_slice(salt_bytes);
    blob.extend_from_slice(&nonce);
    blob.extend_from_slice(&ciphertext);

    Ok(blob)
}

pub fn decrypt(pswd_blob: &Vec<u8>, passphrase: &str) -> CryptoServiceResult<String> {
    if pswd_blob.is_empty() {
        return Err(CryptoServiceError::DecodingError(
            "Empty ciphertext".to_string(),
        ));
    }

    let salt_len = pswd_blob[0] as usize;
    let salt_end = 1 + salt_len;
    let salt_str = std::str::from_utf8(&pswd_blob[1..salt_end])
        .map_err(|_| CryptoServiceError::DecodingError("Incorrect salt".into()))?;

    let nonce_start = salt_end;
    let nonce_end = 12 + nonce_start;
    let nonce = Nonce::from_slice(&pswd_blob[nonce_start..nonce_end]);

    let ciphertext = &pswd_blob[nonce_end..];

    let key = derive_key(passphrase, salt_str)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| CryptoServiceError::DecodingError(e.to_string()))?;

    let decrypted_bytes = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| CryptoServiceError::DecodingError(e.to_string()))?;
    String::from_utf8(decrypted_bytes).map_err(|e| CryptoServiceError::EncodingError(e.to_string()))
}
