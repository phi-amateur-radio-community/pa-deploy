// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/request/method.rs
// HTTP method enum.

#[allow(unused)]
pub enum HttpMethod {
    Get,
    Post,
}

#[allow(unused)]
impl HttpMethod {
    pub fn to_string(&self) -> &str {
        match self {
            HttpMethod::Get  => "GET",
            HttpMethod::Post => "POST",
        }
    }
}
