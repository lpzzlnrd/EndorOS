use alloc::vec::Vec;

/// Errors returned by cryptographic operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CryptoError {
    InvalidKey,
    DecryptionFailed,
    EncryptionFailed,
}

/// Port (trait) that abstracts symmetric encryption and decryption.
pub trait EncryptionPort {
    /// Encrypt `data` with `key` and return the ciphertext.
    fn encrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError>;

    /// Decrypt `data` with `key` and return the plaintext.
    fn decrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError>;
}
