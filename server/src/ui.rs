// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/ui.rs
// TUI

use crate::conf::{Config, ConfigServer};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};
use std::io;
use std::rc::Rc;

const FOOTER_CONTENT: [&str; 6] = [
    "Create [C]",
    "Quit [Q]",
    "Delete [D]",
    "Rename [R]",
    "Move [M]",
    "Save [S]",
];

#[derive(Debug, thiserror::Error)]
pub enum UiError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
}

pub fn config(_config: &mut Config) -> Result<(), UiError> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let _ = execute!(stdout, EnterAlternateScreen);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let area = f.area();

            let screen = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(area);

            let title = format!(
                "PA Deploy Client Configuration Manager Version {}",
                env!("CARGO_PKG_VERSION")
            );
            let title_block = Block::new()
                .borders(Borders::ALL)
                .title(Line::raw(title).alignment(Alignment::Center));
            f.render_widget(&title_block, screen[0]);

            let _ = render_footer(f, &FOOTER_CONTENT, screen[1]);

            let _inner = title_block.inner(screen[0]);
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

fn render_footer(f: &mut Frame, contents: &[&str], area: Rect) -> Rc<[Rect]> {
    let count = contents.len();
    let constraints = vec![Constraint::Fill(1); count];
    let footer = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);
    for (msg, location) in contents.iter().zip(footer.iter()) {
        let text = Paragraph::new(*msg).alignment(Alignment::Center);
        f.render_widget(&text, *location);
    }
    footer
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
