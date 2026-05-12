// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/request/version.rs
// HTTP version enum.

#[derive(Default)]
pub enum HttpVersion {
    V1_1,
    #[default]
    V2,
}

impl HttpVersion {
    pub fn to_string(&self) -> &str {
        match self {
            HttpVersion::V1_1 => "HTTP/1.1",
            HttpVersion::V2 => "HTTP2",
        }
    }
}
