// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /common/src/request/mod.rs
// Request of HTTP(s)

mod core;
mod method;
mod version;

pub use core::HttpRequest;
pub use method::HttpMethod;
pub use version::HttpVersion;
