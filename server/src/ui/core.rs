// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/ui/core.rs
// Structure and enum definition of TUI.

use super::{
    keyboard::{HandlerAction, MoveAction, handler_keyboard},
    render::*,
};
use crate::conf::{Config, ConfigServer, ConfigServerMap};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    layout::{Constraint, Direction, Layout, Rect},
};
use std::io;
use tracing::{debug, trace};

/*
#[derive(Debug, thiserror::Error)]
pub enum UiError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
}

pub struct ScreenData {
    pub(crate) status: ScreenStatus,
    pub(crate) map_cache: ConfigServerMap,
    pub(crate) config: Config,
    pub(crate) ptr: usize,
    pub(crate) ptr_name: &mut String,
    pub(crate) detail_ptr: Option<usize>,
    pub(crate) changed: bool,
}

#[derive(Debug)]
pub enum ScreenStatus {
    Move,
    Input(usize, &mut String),
    Warning(&'static str),
}

impl ScreenData {
    pub fn new(config: Config) -> Self {
        trace!(target: "ui/core", "Initialize ScreenData");
        let status = ScreenStatus::Move;
        let map_cache = ConfigServerMap::new();
        let ptr = 0;
        let ptr_name = config.get_entry_key(ptr);
        let detail_ptr = None;
        let changed = false;
        ScreenData {
            status,
            map_cache,
            config,
            ptr,
            ptr_name,
            detail_ptr,
            changed,
        }
    }

    pub fn display(&mut self) -> Result<(), UiError> {
        debug!(target: "ui/core", "Display the tui");
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        let _ = execute!(stdout, EnterAlternateScreen);
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        trace!(target: "ui/core", "Tui initialization successful");

        loop {
            terminal.draw(|f| {
                let area = f.area();

                let [body, footer] =
                    Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).areas(area);

                let body = render_header(f, body);

                {
                    let [explorer, _detail] =
                        Layout::horizontal([Constraint::Length(24), Constraint::Fill(1)])
                            .areas(body);
                    let explorer = render_line(f, explorer);

                    self.render_explorer(f, explorer);
                }

                let mode = match self.status {
                    ScreenStatus::Warning(msg) => FooterMode::Warning(msg),
                    _ => FooterMode::Explorer,
                };

                render_footer(f, mode, footer);
            })?;

            if event::poll(std::time::Duration::from_millis(50))?
                && let Event::Key(key) = event::read()?
            {
                trace!(target: "ui/core", key = ?key.code, "Press the keyboard");
                match handler_keyboard(self, key) {
                    HandlerAction::Break => break,
                    HandlerAction::Continue => {}
                    HandlerAction::Create => self.create(),
                    HandlerAction::Quit => self.quit(),
                    HandlerAction::ChangeStatus(status) => self.status = status,
                    HandlerAction::Warning(msg) => self.status = ScreenStatus::Warning(msg),
                    HandlerAction::Move(action) => self.move(action),
                }
                trace!(target: "ui/core", screen_status = ?self.status, "Screen status");
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        Ok(())
    }

    pub fn free(self) -> Config {
        self.config
    }

    fn render_explorer(&self, f: &mut Frame, area: Rect) {
        let items = self.config.get_map();
        let size = items.len();
        let constraints = vec![Constraint::Fill(1); size];
        let areas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(area);
        for i in 0..size {
            let (key, _) = match items.get_index(i) {
                Some(item) => item,
                None => break,
            };
            if (self.ptr != i) {
                render_explorer_item(f, areas[i], key, TextStyle::Common);
            }
            if let ScreenStatus::Input(location) = self.status
                && self.detail_ptr.is_none()
            {
                render_edit(f, areas[i], key, location);
            }
        }
    }

    fn is_changed(&self) -> bool {
        self.changed
    }

    pub fn create(&mut self) {
        let map = self.config.get_map_mut();
        trace!(target: "ui/core", "Create configure entry");
        map.insert_before(self.ptr, String::new(), ConfigServer::new());
        self.changed = true;
        self.rename();
    }

    pub fn rename(&mut self) -> Option<()> {
        let (key, _) = self.config.get_map().get_index(self.ptr)?;
        self.status = ScreenStatus::Input(key.len());
        Some(())
    }

    pub fn get_ptr_len(&self) -> Option<usize> {
        match self.detail_ptr {
            Some(location) => self.config.get,
            None => {}
        }
    }

    pub fn get_entry(&self) -> Option<Entry<String, ConfigServer>> {
        self.config.get_map().get_index(self.ptr)?
    }

    pub fn get_detail_location() {}

    pub fn edit_content(&mut self) {
        if let Some(len) = self.get_ptr_len() {
            self.status = ScreenStatus::Input(len);
        }
    }

    fn edit(&mut self, config: ConfigServer) {
        let map = self.config.get_map_mut();
        map.insert_before(self.ptr, String::new(), config);
        self.changed = true;
    }

    fn delete(&mut self) {
        let map = self.config.get_map_mut();
        map.shift_remove_index(self.ptr);
        self.changed = true;
    }
}
*/
