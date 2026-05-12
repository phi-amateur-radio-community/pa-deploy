// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/request/version.rs
// HTTP version enum.

#[allow(unused)]
pub enum HttpVersion {
    V1_1,
    V2,
}

#[allow(unused)]
impl HttpVersion {
    pub fn default() -> Self {
        HttpVersion::V2
    }

    pub fn to_string(&self) -> &str {
        match self {
            HttpVersion::V1_1 => "HTTP/1.1",
            HttpVersion::V2 => "HTTP2",
        }
    }
}
