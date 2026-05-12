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

pub struct HttpRequest {
    method: HttpMethod,
    uri: String,
    version: HttpVersion,
    header: HashMap<String, String>,
    body: String,
}

impl HttpRequest {
    pub fn new(method: HttpMethod, uri: String, version: HttpVersion) -> Self {
        HttpRequest {
            method,
            uri,
            version,
            header: HashMap::new(),
            body: String::from(""),
        }
    }

    pub fn add_header(&mut self, key: String, value: String) {
        self.header.insert(key, value);
    }

    pub fn serialize(&self) -> String {
        let mut s = String::from("");
        s.push_str(self.method.to_string());
        s.push(' ');
        s.push_str(&self.uri);
        s.push(' ');
        s.push_str(self.version.to_string());
        add_crlf(&mut s);

        let mut keys: Vec<&str> = self.header.keys().map(|s| s.as_str()).collect();
        keys.sort();
        for key in keys {
            self.push_header(&mut s, key);
        }
        add_crlf(&mut s);

        s.push_str(&self.body);
        s
    }

    pub fn remove_header(&mut self, key: &str) {
        self.header.remove(key);
    }

    fn push_header(&self, s: &mut String, key: &str) {
        let value = &self.header[key];
        s.push_str(key);
        s.push_str(": ");
        s.push_str(value);
        add_crlf(s);
    }
}

fn add_crlf(s: &mut String) {
    s.push_str("\r\n")
}
