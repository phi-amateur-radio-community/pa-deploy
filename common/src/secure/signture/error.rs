// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/secure/signuare/error.rs
// Error definition for sign.

use ed25519_dalek;
use hmac;

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
