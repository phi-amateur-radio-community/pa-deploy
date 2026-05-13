// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/secure/signuare/hmac.rs
// HMAC-SHA256 signer and verifier

use super::SignError;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub struct HmacSign {
    pub key: Vec<u8>,
}

impl HmacSign {
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
