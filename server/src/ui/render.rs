// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/ui/render.rs
// Render for TUI

use super::UiError;
use crate::conf::Config;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};
use std::{io, rc::Rc};

const FOOTER_CONTENT: [&str; 6] = [
    "Create [C]",
    "Quit [Q]",
    "Delete [D]",
    "Rename [R]",
    "Move [M]",
    "Save [S]",
];

#[allow(unused)]
const STYLE_COMMON: Style = Style::new().bg(Color::Black).fg(Color::White);
#[allow(unused)]
const STYLE_FOCUS: Style = Style::new().bg(Color::White).fg(Color::Black);
#[allow(unused)]
const STYLE_INPUT: Style = Style::new()
    .bg(Color::Black)
    .fg(Color::White)
    .add_modifier(Modifier::BOLD);
#[allow(unused)]
const STYLE_ERROR: Style = Style::new()
    .bg(Color::Black)
    .fg(Color::Red)
    .add_modifier(Modifier::BOLD);

pub fn config(_config: &mut Config) -> Result<(), UiError> {
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

            let body = {
                let title = format!(
                    "PA Deploy Client Configuration Manager Version {}",
                    env!("CARGO_PKG_VERSION")
                );
                let title_block = Block::new()
                    .borders(Borders::ALL)
                    .title(Line::raw(title).alignment(Alignment::Center));
                f.render_widget(&title_block, body);
                title_block.inner(body)
            };

            {
                let [explorer, _detail] =
                    Layout::horizontal([Constraint::Length(24), Constraint::Fill(1)]).areas(body);
                f.render_widget(Block::default().borders(Borders::RIGHT), explorer);
            }

            let _ = render_footer(f, &FOOTER_CONTENT, footer);
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

pub fn render_explorer_item(f: &mut Frame, area: Rect, name: &str, _is_focus: bool) {
    let text = Paragraph::new(name).style(STYLE_COMMON);
    f.render_widget(text, area);
}
