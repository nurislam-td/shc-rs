use crate::service::crypt::errors::CryptoServiceError;

pub type CryptoServiceResult<T> = Result<T, CryptoServiceError>;
