// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/secure/encrypt/aes.rs
// AES-256-GCM encrypt.

use super::EncryptError;
use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead};

pub struct AesEncrypt {
    aes: Aes256Gcm,
    counter: u64,
    session_id: u32,
}

impl AesEncrypt {
    pub fn new(key: &[u8; 32], session_id: u32) -> Result<Self, EncryptError> {
        Ok(AesEncrypt {
            aes: Aes256Gcm::new_from_slice(key).map_err(|_| EncryptError::AesInvalidLength)?,
            counter: 0u64,
            session_id,
        })
    }

    pub fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, EncryptError> {
        let mut nonce = [0u8; 12];
        nonce[..8].copy_from_slice(&self.counter.to_be_bytes());
        nonce[8..12].copy_from_slice(&self.session_id.to_be_bytes());
        self.counter += 1;

        let ciphertext = self
            .aes
            .encrypt(&nonce.into(), data)
            .map_err(|_| EncryptError::AesEncryptFailed)?;

        let mut out = Vec::new();
        out.extend_from_slice(&nonce);
        out.extend_from_slice(&ciphertext);

        Ok(out)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, EncryptError> {
        let (nonce, enc) = data.split_at(12);
        self.aes
            .decrypt(nonce.into(), enc)
            .map_err(|_| EncryptError::AesAuthFailed)
    }
}
