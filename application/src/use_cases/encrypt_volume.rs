use alloc::vec::Vec;
use crate::ports::encryption::{CryptoError, EncryptionPort};

/// Use-case: encrypt an in-memory volume (byte slice) with the provided key.
pub struct EncryptVolume<E: EncryptionPort> {
    crypto: E,
}

impl<E: EncryptionPort> EncryptVolume<E> {
    pub fn new(crypto: E) -> Self {
        Self { crypto }
    }

    /// Encrypt `data` with `key`. Returns ciphertext on success.
    pub fn execute(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if key.is_empty() {
            return Err(CryptoError::InvalidKey);
        }
        self.crypto.encrypt(data, key)
    }

    /// Decrypt `data` with `key`. Returns plaintext on success.
    pub fn decrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if key.is_empty() {
            return Err(CryptoError::InvalidKey);
        }
        self.crypto.decrypt(data, key)
    }

    pub fn crypto(&self) -> &E {
        &self.crypto
    }
}
