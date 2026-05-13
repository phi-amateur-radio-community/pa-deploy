// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/secure/signuare/ed25519.rs
// Ed25519 signer and verifier

use super::SignError;
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};

pub enum Ed25519Sign {
    PrivateKey(SigningKey),
    PublicKey(VerifyingKey),
}

impl Ed25519Sign {
    pub fn sign(&self, msg: &[u8]) -> Result<[u8; 64], SignError> {
        let pri_key: &SigningKey = match self {
            Ed25519Sign::PrivateKey(key) => key,
            Ed25519Sign::PublicKey(_) => return Err(SignError::NoneKeyError),
        };
        Ok(pri_key.sign(msg).into())
    }

    pub fn verify(&self, msg: &[u8], signature: &[u8]) -> Result<bool, SignError> {
        let verifying_key: &VerifyingKey = match self {
            Ed25519Sign::PrivateKey(key) => &key.verifying_key(),
            Ed25519Sign::PublicKey(key) => key,
        };
        Ok(verifying_key
            .verify_strict(msg, &Signature::from_slice(signature)?)
            .is_ok())
    }
}
