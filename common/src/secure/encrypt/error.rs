// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/secure/encrypt/error.rs
// Error definition for encrypt.

#[derive(Debug, thiserror::Error)]
pub enum EncryptError {
    #[error("Invalid key length")]
    InvalidLength,
    #[error("Encrypt failed")]
    EncryptFailed,
    #[error("Authentication failed")]
    AuthFailed,
    #[error("Wrong key type")]
    KeyDisallow,
    #[error("This encrypt method is not surport")]
    MethodDisable,
}
