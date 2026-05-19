// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/ui/core.rs
// Structure and enum definition of TUI.

use crate::conf::{Config, ConfigServer};

#[derive(Debug, thiserror::Error)]
pub enum UiError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
}

#[allow(unused)]
struct ScreenData {
    status: ScreenStatus,
    focus_index: Option<usize>,
    config: ConfigData,
}

#[allow(unused)]
enum ScreenStatus {
    Move,
    Input,
}

#[allow(unused)]
struct ConfigData {
    config: Config,
    ptr: usize,
}

#[allow(unused)]
impl ScreenData {
    fn new(config: Config) -> Self {
        let status = ScreenStatus::Move;
        let focus_index = None;
        let config = ConfigData::new(config);
        ScreenData {
            status,
            focus_index,
            config,
        }
    }

    fn get_config(&mut self) -> &mut ConfigData {
        &mut self.config
    }

    fn free(self) -> Config {
        self.config.free()
    }
}

#[allow(unused)]
impl ConfigData {
    fn new(config: Config) -> Self {
        let ptr = 0;
        ConfigData { config, ptr }
    }

    fn get_server(&self) -> Option<&ConfigServer> {
        let (_, server) = self.config.get_map().get_index(self.ptr)?;
        Some(server)
    }

    fn create(&mut self) {
        let map = self.config.get_map_mut();
        map.insert_before(self.ptr, String::new(), ConfigServer::new());
    }

    fn edit(&mut self, config: ConfigServer) {
        let map = self.config.get_map_mut();
        map.insert_before(self.ptr, String::new(), config);
    }

    fn delete(&mut self) {
        let map = self.config.get_map_mut();
        map.shift_remove_index(self.ptr);
    }

    fn free(self) -> Config {
        self.config
    }
}
