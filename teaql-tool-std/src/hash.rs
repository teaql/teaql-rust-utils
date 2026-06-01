use sha2::{Sha256, Sha512, Digest};
use blake3;
use crc32fast::Hasher as Crc32Hasher;

pub struct HashTool;

impl HashTool {
    pub fn new() -> Self {
        Self
    }

    pub fn sha256(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    pub fn sha512(&self, data: &[u8]) -> String {
        let mut hasher = Sha512::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    pub fn blake3(&self, data: &[u8]) -> String {
        let hash = blake3::hash(data);
        hash.to_hex().to_string()
    }

    pub fn crc32(&self, data: &[u8]) -> u32 {
        let mut hasher = Crc32Hasher::new();
        hasher.update(data);
        hasher.finalize()
    }
}

impl Default for HashTool {
    fn default() -> Self {
        Self::new()
    }
}
