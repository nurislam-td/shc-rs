use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoServiceError {
    #[error("Key derivation function failed: {0}")]
    KeyDerivationError(String),

    #[error("Encoding error: {0}")]
    EncodingError(String),

    #[error("Decoding error: {0}")]
    DecodingError(String),
}
