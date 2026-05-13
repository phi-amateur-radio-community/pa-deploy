// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/secure/signuare.rs
// Sign and check the HTTP request.

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub trait Signable {
    fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, SignError>;
    fn verify(&self, msg: &[u8], signature: &[u8]) -> Result<bool, SignError>;
}

#[derive(Debug, thiserror::Error)]
pub enum SignError {
    #[error("HMAC key invalid length")]
    HmacInvalidLength(#[from] hmac::digest::InvalidLength),
}

pub struct HmacSign {
    key: Vec<u8>,
}

impl Signable for HmacSign {
    fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, SignError> {
        let mut mac = HmacSha256::new_from_slice(&self.key)?;
        mac.update(msg);

        let result = mac.finalize();
        let bytes = result.into_bytes();

        Ok(bytes.to_vec())
    }

    fn verify(&self, msg: &[u8], signature: &[u8]) -> Result<bool, SignError> {
        Ok(self.sign(msg)? == signature)
    }
}

// TODO(secure): add Ed25519 signature
