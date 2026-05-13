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

struct HmacSign {
    key: Vec<u8>,
}

impl HmacSign {
    fn sign(&self, msg: &[u8]) -> Result<[u8; 32], SignError> {
        let mut mac = HmacSha256::new_from_slice(&self.key)?;
        mac.update(msg);

        let result = mac.finalize();
        let bytes = result.into_bytes();

        Ok(bytes.into())
    }

    fn verify(&self, msg: &[u8], signature: &[u8]) -> Result<bool, SignError> {
        Ok(self.sign(msg)? == signature)
    }
}

enum Ed25519Sign {
    PrivateKey(SigningKey),
    PublicKey(VerifyingKey),
}

impl Ed25519Sign {
    fn sign(&self, msg: &[u8]) -> Result<[u8; 64], SignError> {
        let pri_key: &SigningKey = match self {
            Ed25519Sign::PrivateKey(key) => key,
            Ed25519Sign::PublicKey(_) => return Err(SignError::NoneKeyError),
        };
        Ok(pri_key.sign(msg).into())
    }

    fn verify(&self, msg: &[u8], signature: &[u8]) -> Result<bool, SignError> {
        let verifying_key: &VerifyingKey = match self {
            Ed25519Sign::PrivateKey(key) => &key.verifying_key(),
            Ed25519Sign::PublicKey(key) => key,
        };
        Ok(verifying_key
            .verify_strict(msg, &Signature::from_slice(signature)?)
            .is_ok())
    }
}
