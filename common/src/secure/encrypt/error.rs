// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/secure/encrypt/error.rs
// Error definition for encrypt.

#[derive(Debug, thiserror::Error)]
pub enum EncryptError {
    #[error("Aes invalid key length")]
    AesInvalidLength,
    #[error("Aes encrypt failed")]
    AesEncryptFailed,
    #[error("Aes authentication failed")]
    AesAuthFailed,
}
