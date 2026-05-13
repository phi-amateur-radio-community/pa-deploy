// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/secure/signuare.rs
// Sign and check the HTTP request.

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub enum SignatureItem {
    Ed25519([u8; 64]),
    HmacSha256([u8; 32]),
}

#[derive(Debug, thiserror::Error)]
pub enum SignError {
    #[error("HMAC key invalid length")]
    HmacInvalidLength(#[from] hmac::digest::InvalidLength),
    #[error("Ed25519 Error")]
    Ed25519Error(#[from] ed25519_dalek::ed25519::Error),
    #[error("Missing private key of Ed25519")]
    NoneKeyError,
    #[error("Mismatched key")]
    UnmatchKeyError,
}

pub enum KeyType {
    PrivateKey([u8; 32]),
    PublicKey([u8; 32]),
    SymmetricKey(Vec<u8>),
}

pub struct HmacSign {
    key: Vec<u8>,
}

impl HmacSign {
    pub fn new(key: Vec<u8>) -> Result<Self, SignError> {
        Ok(HmacSign { key })
    }

    pub fn sign(&self, msg: &[u8]) -> Result<[u8; 32], SignError> {
        let mut mac = HmacSha256::new_from_slice(&self.key)?;
        mac.update(msg);

        let result = mac.finalize();
        let bytes = result.into_bytes();

        Ok(bytes.into())
    }

    pub fn verify(&self, msg: &[u8], signature: &[u8]) -> Result<bool, SignError> {
        Ok(self.sign(msg)? == signature)
    }
}

pub enum Ed25519Sign {
    PrivateKey(SigningKey),
    PublicKey(VerifyingKey),
}

impl Ed25519Sign {
    pub fn new(key: &KeyType) -> Result<Self, SignError> {
        Ok(match key {
            KeyType::PrivateKey(key) => Ed25519Sign::PrivateKey(SigningKey::from_bytes(key)),
            KeyType::PublicKey(key) => Ed25519Sign::PublicKey(VerifyingKey::from_bytes(key)?),
            _ => return Err(SignError::UnmatchKeyError),
        })
    }

    pub fn sign(&self, msg: &[u8]) -> Result<[u8; 64], SignError> {
        let pri_key: &SigningKey = match self {
            Ed25519Sign::PrivateKey(key) => key,
            Ed25519Sign::PublicKey(_) => return Err(SignError::NoneKeyError),
        };
        Ok(pri_key.sign(msg).into())
    }

    pub fn verify(&self, msg: &[u8], signature: &[u8]) -> Result<bool, SignError> {
        let verify_key: &VerifyingKey = match self {
            Ed25519Sign::PrivateKey(key) => &key.verifying_key(),
            Ed25519Sign::PublicKey(key) => key,
        };
        Ok(verify_key
            .verify_strict(msg, &Signature::from_slice(signature)?)
            .is_ok())
    }
}
