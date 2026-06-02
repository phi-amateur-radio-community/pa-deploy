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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    server: IndexMap<String, ConfigServer>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigServer {
    port: u16,
    bind: String,
}

#[allow(unused)]
pub struct ConfigServerMap {
    content: IndexMap<String, String>,
    change: bool,
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

    pub fn get_entry(&self, ptr: usize) -> Option<(&String, &ConfigServer)> {
        self.server.get_index(ptr)
    }

    pub fn get_size(&self) -> usize {
        self.server.len()
    }
}

#[allow(unused)]
impl ConfigServer {
    pub fn edit(&mut self, content: ConfigServerMap) -> Self {
        ConfigServer {
            port: 0,
            bind: String::new(),
        }
    }

    pub fn get_port(&self) -> &u16 {
        &self.port
    }

    pub fn get_bind(&self) -> &String {
        &self.bind
    }
}

#[allow(unused)]
impl ConfigServerMap {
    pub fn new() -> Self {
        let mut map = IndexMap::<String, String>::new();
        map.insert("port".to_string(), String::new());
        map.insert("bind".to_string(), String::new());
        ConfigServerMap {
            content: map,
            change: false,
        }
    }

    pub fn map(source: &ConfigServer) -> Self {
        let mut map = IndexMap::<String, String>::new();
        map.insert("port".to_string(), source.get_port().to_string());
        map.insert("bind".to_string(), source.get_bind().to_string());
        ConfigServerMap {
            content: map,
            change: false,
        }
    }

    fn unmap(mut self) -> Result<ConfigServer, ConfigError> {
        let port = self
            .content
            .shift_remove("port")
            .ok_or(ConfigError::Unknown)?
            .parse::<u16>()
            .unwrap_or_default();
        let bind = self
            .content
            .shift_remove("bind")
            .ok_or(ConfigError::Unknown)?;
        Ok(ConfigServer { port, bind })
    }

    pub fn get_mut_index(&mut self, ptr: usize) -> &mut String {
        &mut self.content[ptr]
    }

    pub fn get_len(&self) -> usize {
        self.content.len()
    }
}
