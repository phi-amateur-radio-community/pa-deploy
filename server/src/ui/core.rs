// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/ui/core.rs
// Structure and enum definition of TUI.

use super::render::*;
use crate::conf::{Config, ConfigServer};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    layout::{Constraint, Direction, Layout, Rect},
};
use std::io;

#[derive(Debug, thiserror::Error)]
pub enum UiError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
}

#[allow(unused)]
pub struct ScreenData {
    status: ScreenStatus,
    focus_index: Option<usize>,
    config: ConfigData,
}

#[allow(unused)]
enum ScreenStatus {
    Move,
    Input,
}

struct ConfigData {
    config: Config,
    ptr: usize,
}

#[allow(unused)]
impl ScreenData {
    pub fn new(config: Config) -> Self {
        let status = ScreenStatus::Move;
        let focus_index = None;
        let config = ConfigData::new(config);
        ScreenData {
            status,
            focus_index,
            config,
        }
    }

    pub fn display(&mut self) -> Result<(), UiError> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        let _ = execute!(stdout, EnterAlternateScreen);
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

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

                    self.render(f, explorer);
                }

                let _ = render_footer(f, FooterMode::Explorer, footer);
            })?;

            if event::poll(std::time::Duration::from_millis(50))?
                && let Event::Key(key) = event::read()?
            {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') => continue, // TODO(ui): add create new configure server.
                    KeyCode::Char('d') => continue, // TODO(ui): add remove new configure server.
                    _ => continue,
                }
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        Ok(())
    }

    fn get_config(&self) -> &ConfigData {
        &self.config
    }

    fn get_config_mut(&mut self) -> &mut ConfigData {
        &mut self.config
    }

    fn free(self) -> Config {
        self.config.free()
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let items = self.config.config.get_map();
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
            render_explorer_item(f, areas[i], key, ExplorerStyle::Common);
        }
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
