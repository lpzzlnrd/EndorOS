use application::ports::encryption::{CryptoError, EncryptionPort};

/// Demo cipher: repeating-key XOR.
/// NOT cryptographically secure — used solely to demonstrate the adapter pattern.
pub struct XorCryptoAdapter;

impl XorCryptoAdapter {
    pub fn new() -> Self {
        Self
    }

    fn xor_with_key(data: &[u8], key: &[u8]) -> Vec<u8> {
        data.iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ key[i % key.len()])
            .collect()
    }
}

impl Default for XorCryptoAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl EncryptionPort for XorCryptoAdapter {
    fn encrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if key.is_empty() {
            return Err(CryptoError::InvalidKey);
        }
        Ok(Self::xor_with_key(data, key))
    }

    fn decrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if key.is_empty() {
            return Err(CryptoError::InvalidKey);
        }
        // XOR is its own inverse.
        Ok(Self::xor_with_key(data, key))
    }
}
