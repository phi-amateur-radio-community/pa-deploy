// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/secure/signuare/common.rs
// API of signer

use super::{SignError, ed25519::*, hmac::*};
use ed25519_dalek::{SigningKey, VerifyingKey};

pub struct CommonSigner {
    item: SignItem,
}

pub enum KeyType {
    PrivateKey([u8; 32]),
    PublicKey([u8; 32]),
    SymmetricKey(Vec<u8>),
}

pub enum SignatureItem {
    Ed25519([u8; 64]),
    HmacSha256([u8; 32]),
}

enum SignItem {
    HmacSha256(HmacSign),
    Ed25519(Box<Ed25519Sign>),
}

impl CommonSigner {
    pub fn new(key: KeyType) -> Result<Self, SignError> {
        Ok(CommonSigner {
            item: match key {
                KeyType::PrivateKey(key) => SignItem::Ed25519(Box::new(Ed25519Sign::PrivateKey(
                    SigningKey::from_bytes(&key),
                ))),
                KeyType::PublicKey(key) => SignItem::Ed25519(Box::new(Ed25519Sign::PublicKey(
                    VerifyingKey::from_bytes(&key)?,
                ))),
                KeyType::SymmetricKey(key) => SignItem::HmacSha256(HmacSign { key }),
            },
        })
    }

    pub fn sign(&self, msg: &[u8]) -> Result<SignatureItem, SignError> {
        Ok(match &self.item {
            SignItem::HmacSha256(signer) => SignatureItem::HmacSha256(signer.sign(msg)?),
            SignItem::Ed25519(signer) => SignatureItem::Ed25519(signer.sign(msg)?),
        })
    }

    pub fn verify(&self, msg: &[u8], signature: &[u8]) -> Result<bool, SignError> {
        match &self.item {
            SignItem::HmacSha256(verifier) => verifier.verify(msg, signature),
            SignItem::Ed25519(verifier) => verifier.verify(msg, signature),
        }
    }
}
