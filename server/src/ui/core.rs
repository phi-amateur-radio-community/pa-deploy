// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/ui/core.rs
// Structure and enum definition of TUI.

use super::{
    keyboard::{HandlerStatus, MoveAction, handler_keyboard},
    render::*,
};
use crate::conf::{Config, ConfigServer};
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

#[derive(Debug, thiserror::Error)]
pub enum UiError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
}

#[allow(unused)]
pub struct ScreenData {
    status: ScreenStatus,
    focus_index: Option<usize>,
    config: Config,
    ptr: usize,
    detail_ptr: Option<usize>,
    changed: bool,
}

#[allow(unused)]
#[derive(Debug)]
pub enum ScreenStatus {
    Move,
    Input,
    Warning(&'static str),
}

#[allow(unused)]
impl ScreenData {
    pub fn new(config: Config) -> Self {
        trace!(target: "ui/core", "Initialize ScreenData");
        let status = ScreenStatus::Move;
        let focus_index = None;
        let ptr = 0;
        let detail_ptr = None;
        let changed = false;
        ScreenData {
            status,
            focus_index,
            config,
            ptr,
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
                && !matches!(self.status, ScreenStatus::Input)
            {
                trace!(target: "ui/core", key = ?key.code, "Press the keyboard");
                match handler_keyboard(self, key) {
                    HandlerStatus::Break => break,
                    HandlerStatus::Continue => {
                        if matches!(self.status, ScreenStatus::Warning(_)) {
                            self.status = ScreenStatus::Move;
                        }
                    }
                    HandlerStatus::Warning(msg) => self.status = ScreenStatus::Warning(msg),
                    HandlerStatus::Move(action) => self.move_action(action),
                }
                trace!(target: "ui/core", screen_status = ?self.status, "Screen status");
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        Ok(())
    }

    pub fn get_status(&self) -> &ScreenStatus {
        &self.status
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_config_mut(&mut self) -> &mut Config {
        &mut self.config
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
            render_explorer_item(f, areas[i], key, TextStyle::Common);
        }
    }

    pub fn is_changed(&self) -> bool {
        self.changed
    }

    pub fn get_server(&self) -> Option<&ConfigServer> {
        let (_, server) = self.config.get_map().get_index(self.ptr)?;
        Some(server)
    }

    pub fn create(&mut self) {
        let map = self.config.get_map_mut();
        trace!(target: "ui/core", "Create configure entry");
        map.insert_before(self.ptr, String::new(), ConfigServer::new());
        self.changed = true;
    }

    pub fn rename(&mut self) {
        self.status = ScreenStatus::Input;
    }

    fn move_action(&mut self, action: MoveAction) {
        match self.detail_ptr {
            Some(mut ptr) => match action {
                MoveAction::Up => ptr_loop(&mut ptr, false, self.config.get_size()),
                MoveAction::Down => ptr_loop(&mut ptr, false, self.config.get_size()),
                MoveAction::Left => self.detail_ptr = None,
                MoveAction::Right => {}
            },
            None => match action {
                MoveAction::Up => ptr_loop(&mut self.ptr, true, self.config.get_size()),
                MoveAction::Down => ptr_loop(&mut self.ptr, false, self.config.get_size()),
                MoveAction::Left => {}
                MoveAction::Right => self.detail_ptr = Some(0),
            },
        }
        trace!(target: "ui/core", explorer_ptr = self.ptr, detail_ptr = self.detail_ptr, "Screen status");
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

fn ptr_loop(ptr: &mut usize, is_up: bool, max: usize) {
    if max == 0 {
        return;
    }
    if is_up {
        *ptr += 1;
        if *ptr == max {
            *ptr = 0;
        }
    } else {
        if *ptr == 0 {
            *ptr = max - 1;
        } else {
            *ptr -= 1;
        }
    }
}
