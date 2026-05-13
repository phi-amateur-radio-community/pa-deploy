// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/secure/encrypt/common.rs
// API of encrypt module.

use super::*;

pub enum EncryptMethod {
    Unencrypt,
    OnlyAes(Box<aes::AesEncrypt>),
    RsaAes,    // TODO(encrypt): RSA-OAEP + AES-256-GCM
    X25519Aes, // TODO(encrypt): X25519 + AES-256-GCM
    X25519Cha, // TODO(encrypt): X25519 + ChaCha20-Poly1305
}

pub enum EncryptKey {
    NoneKey,
    AesKey([u8; 32], u32),
    RsaKey,
    X25519AesKey,
    X25519ChaKey,
}

impl EncryptMethod {
    pub fn new(key: EncryptKey) -> Result<Self, EncryptError> {
        Ok(match key {
            EncryptKey::NoneKey => EncryptMethod::Unencrypt,
            EncryptKey::AesKey(key, session_id) => {
                EncryptMethod::OnlyAes(Box::new(aes::AesEncrypt::new(&key, session_id)?))
            }
            _ => EncryptMethod::Unencrypt,
        })
    }

    pub fn encrypt(&mut self, msg: &[u8]) -> Result<Vec<u8>, EncryptError> {
        match self {
            EncryptMethod::Unencrypt => Ok(msg.to_vec()),
            EncryptMethod::OnlyAes(encrypt) => encrypt.encrypt(msg),
            _ => Ok(Vec::<u8>::new()),
        }
    }

    pub fn decrypt(&self, msg: &[u8]) -> Result<Vec<u8>, EncryptError> {
        match self {
            EncryptMethod::Unencrypt => Ok(msg.to_vec()),
            EncryptMethod::OnlyAes(encrypt) => encrypt.decrypt(msg),
            _ => Ok(Vec::<u8>::new()),
        }
    }
}
