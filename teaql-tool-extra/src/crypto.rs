use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};
use teaql_tool_core::{Result, TeaQLToolError};

pub struct CryptoTool;

impl CryptoTool {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_key(&self) -> Vec<u8> {
        Aes256Gcm::generate_key(OsRng).to_vec()
    }

    pub fn encrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        let key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits
        
        let encrypted = cipher.encrypt(&nonce, data).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        
        // Prepend nonce to ciphertext
        let mut result = nonce.to_vec();
        result.extend_from_slice(&encrypted);
        Ok(result)
    }

    pub fn decrypt(&self, encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        if encrypted_data.len() < 12 {
            return Err(TeaQLToolError::InvalidArgument("Invalid encrypted data length".to_string()));
        }
        let key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        
        let nonce = Nonce::from_slice(&encrypted_data[0..12]);
        let ciphertext = &encrypted_data[12..];
        
        cipher.decrypt(nonce, ciphertext).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))
    }
}

impl Default for CryptoTool {
    fn default() -> Self {
        Self::new()
    }
}
