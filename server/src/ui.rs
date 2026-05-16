// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/ui.rs
// TUI

use crate::conf::Config;
use ratatui::crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::io;

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
};

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
            let block = Block::default()
                .title("PA Deploy Client Configuration Manager")
                .borders(Borders::ALL);

            let text = Paragraph::new("Hello World").block(block);
            f.render_widget(text, area);
        })?;

        if event::poll(std::time::Duration::from_millis(50))?
            && let Event::Key(key) = event::read()?
            && key.code == KeyCode::Char('q')
        {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
