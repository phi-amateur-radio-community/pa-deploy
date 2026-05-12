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

pub fn sign(key: &[u8], msg: &[u8]) -> Result<String, hmac::digest::InvalidLength> {
    let mut mac = HmacSha256::new_from_slice(key)?;
    mac.update(msg);

    let result = mac.finalize();
    let bytes = result.into_bytes();

    Ok(hex::encode(bytes))
}
