// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/secure/encrypt/mod.rs
// Encrypt module.

mod aes;
mod error;

pub use aes::AesEncrypt;
pub use error::EncryptError;
