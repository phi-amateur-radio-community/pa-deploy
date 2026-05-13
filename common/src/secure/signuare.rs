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

pub trait Signable {
    fn sign(&self, msg: &[u8]) -> Result<SignatureItem, SignError>;
    fn verify(&self, msg: &[u8], signature: &[u8]) -> Result<bool, SignError>;
}

pub enum SignatureItem {
    Ed25519([u8; 64]),
    HmacSha256([u8; 32]),
}

#[derive(Debug, thiserror::Error)]
pub enum SignError {
    #[error("HMAC key invalid length")]
    HmacInvalidLength(#[from] hmac::digest::InvalidLength),
    #[error("Missing private key of Ed25519")]
    NoneKeyError,
    #[error("Ed25519 Error")]
    Ed25519Error(#[from] ed25519_dalek::ed25519::Error),
}

pub struct HmacSign {
    key: Vec<u8>,
}

pub enum Ed25519Sign {
    PrivateKey([u8; 32]),
    PublicKey([u8; 32]),
}

impl Signable for HmacSign {
    fn sign(&self, msg: &[u8]) -> Result<SignatureItem, SignError> {
        let mut mac = HmacSha256::new_from_slice(&self.key)?;
        mac.update(msg);

        let result = mac.finalize();
        let bytes = result.into_bytes();

        Ok(SignatureItem::HmacSha256(bytes.into()))
    }

    fn verify(&self, msg: &[u8], signature: &[u8]) -> Result<bool, SignError> {
        Ok(self.sign(msg)?.equal(signature))
    }
}

impl Signable for Ed25519Sign {
    fn sign(&self, msg: &[u8]) -> Result<SignatureItem, SignError> {
        let pri_key = match self {
            Ed25519Sign::PrivateKey(key) => SigningKey::from_bytes(key),
            Ed25519Sign::PublicKey(_) => return Err(SignError::NoneKeyError),
        };
        Ok(SignatureItem::Ed25519(pri_key.sign(msg).into()))
    }

    fn verify(&self, msg: &[u8], signature: &[u8]) -> Result<bool, SignError> {
        let verify_key: VerifyingKey = match self {
            Ed25519Sign::PrivateKey(key) => VerifyingKey::from_bytes(key)?,
            Ed25519Sign::PublicKey(key) => VerifyingKey::from_bytes(key)?,
        };
        Ok(verify_key
            .verify_strict(msg, &Signature::from_slice(signature)?)
            .is_ok())
    }
}

impl SignatureItem {
    fn equal(&self, signature: &[u8]) -> bool {
        self.get_value() == signature
    }

    pub fn get_value(&self) -> &[u8] {
        match self {
            SignatureItem::HmacSha256(value) => value,
            SignatureItem::Ed25519(value) => value,
        }
    }
}
