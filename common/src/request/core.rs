// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/request/core.rs
// Core of HTTPS request.

use super::HttpMethod;
use super::HttpVersion;
use std::collections::HashMap;

#[allow(unused)]
pub struct HttpRequest {
    method: HttpMethod,
    version: HttpVersion,
    header: HashMap<String, String>,
    body: String,
}
