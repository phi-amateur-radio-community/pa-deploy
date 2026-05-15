// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/conf.rs
// Manage configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    server: HashMap<String, ConfigServer>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ConfigServer {
    ports: Vec<u16>,
    bind: String,
}

#[allow(unused)]
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Read file error")]
    FileError(#[from] std::io::Error),
    #[error("Deserialize error")]
    DeserializeError(#[from] toml::de::Error),
}

#[allow(unused)]
impl Config {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        Ok(if path.as_ref().exists() {
            let file = fs::read_to_string(path)?;
            toml::from_str(&file)?
        } else {
            Config {
                server: HashMap::<String, ConfigServer>::new(),
            }
        })
    }
}
