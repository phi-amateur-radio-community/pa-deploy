// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/conf.rs
// Manage configuration.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    server: IndexMap<String, ConfigServer>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigServer {
    ports: Vec<u16>,
    bind: String,
}

#[allow(unused)]
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Deserialize error")]
    Deserialize(#[from] toml::de::Error),
    #[error("Serialize error")]
    Serialize(#[from] toml::ser::Error),
}

#[allow(unused)]
impl Config {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        Ok(if path.as_ref().exists() {
            let file = fs::read_to_string(path)?;
            toml::from_str(&file)?
        } else {
            Config {
                server: IndexMap::<String, ConfigServer>::new(),
            }
        })
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), ConfigError> {
        let text = toml::to_string_pretty(self)?;
        fs::write(path.as_ref(), text)?;
        Ok(())
    }

    pub fn get_map(&self) -> &IndexMap<String, ConfigServer> {
        &self.server
    }

    pub fn get_map_mut(&mut self) -> &mut IndexMap<String, ConfigServer> {
        &mut self.server
    }
}

#[allow(unused)]
impl ConfigServer {
    pub fn new() -> Self {
        ConfigServer {
            ports: Vec::<u16>::new(),
            bind: String::new(),
        }
    }

    pub fn get_ports(&mut self) -> &mut Vec<u16> {
        &mut self.ports
    }

    pub fn get_bind(&mut self) -> &mut String {
        &mut self.bind
    }
}
