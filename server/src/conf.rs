// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/conf.rs
// Manage configuration.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    server: Vec<(String, ConfigServer)>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigServer {
    port: u16,
    bind: String,
}

pub struct ConfigServerPlat {
    name: String,
    data: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Deserialize error")]
    Deserialize(#[from] toml::de::Error),
    #[error("Serialize error")]
    Serialize(#[from] toml::ser::Error),
    #[error("Unknown error")]
    Unknown,
}

#[allow(unused)]
impl Config {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        Ok(if path.as_ref().exists() {
            let file = fs::read_to_string(path)?;
            toml::from_str(&file)?
        } else {
            Config {
                server: Vec::<(String, ConfigServer)>::new(),
            }
        })
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), ConfigError> {
        let text = toml::to_string_pretty(self)?;
        fs::write(path.as_ref(), text)?;
        Ok(())
    }

    pub fn get_entry(&self, ptr: usize) -> ConfigServerPlat {
        let (name, data) = &self.server[ptr].clone();
        let name = name.to_string();
        let data = data.get();
        ConfigServerPlat { name, data }
    }

    pub fn set_entry(&mut self, ptr: usize, data: ConfigServerPlat) -> Result<(), ConfigError> {
        let ConfigServerPlat { name, data } = data;

        let data = ConfigServer::set(data)?;

        self.server[ptr] = (name, data);
        Ok(())
    }
}

impl ConfigServer {
    pub fn get(&self) -> Vec<String> {
        vec![self.bind.clone(), self.port.to_string()]
    }

    pub fn set(data: Vec<String>) -> Result<Self, ConfigError> {
        let mut data_iter = data.into_iter();
        let bind = data_iter.next().ok_or(ConfigError::Unknown)?;
        let port = data_iter
            .next()
            .ok_or(ConfigError::Unknown)?
            .parse::<u16>()
            .unwrap_or_default();
        Ok(ConfigServer { bind, port })
    }
}
