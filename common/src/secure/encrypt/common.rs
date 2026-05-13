// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/secure/encrypt/common.rs
// API of encrypt module.

pub enum EncryptMethod {
    Unencrypt,
    OnlyAes,   // TODO(encrypt): AES-256-GCM
    RsaAes,    // TODO(encrypt): RSA-OAEP + AES-256-GCM
    X25519Aes, // TODO(encrypt): X25519 + AES-256-GCM
    X25519Cha, // TODO(encrypt): X25519 + ChaCha20-Poly1305
}

impl EncryptMethod {
    pub fn encrypt(&self, msg: &[u8]) -> Vec<u8> {
        match self {
            EncryptMethod::Unencrypt => msg.to_vec(),
            _ => Vec::<u8>::new(),
        }
    }

    pub fn decrypt(&self, msg: &[u8]) -> Vec<u8> {
        match self {
            EncryptMethod::Unencrypt => msg.to_vec(),
            _ => Vec::<u8>::new(),
        }
    }
}
