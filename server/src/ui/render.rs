// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/ui/render.rs
// Render for TUI

/*

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub enum TextStyle {
    Common,
    Focus,
    Error,
}

impl TextStyle {
    pub fn get_color(&self) -> Style {
        match self {
            TextStyle::Common => Style::new().bg(Color::Black).fg(Color::White),
            TextStyle::Focus => Style::new().bg(Color::White).fg(Color::Black),
            TextStyle::Error => Style::new()
                .bg(Color::Black)
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        }
    }
}

pub enum FooterMode {
    Explorer,
    Detail,
    Warning(&'static str),
}

impl FooterMode {
    pub fn get_content(&self) -> &[&str] {
        match self {
            FooterMode::Explorer => &[
                "Create [C]",
                "Quit [Q]",
                "Delete [D]",
                "Rename [R]",
                "Move [M]",
                "Save [S]",
            ],
            FooterMode::Detail => &[],
            FooterMode::Warning(_) => &[],
        }
    }
}

pub fn render_header(f: &mut Frame, area: Rect) -> Rect {
    let title = format!(
        "PA Deploy Client Configuration Manager Version {}",
        env!("CARGO_PKG_VERSION")
    );
    let title_block = Block::new()
        .borders(Borders::ALL)
        .title(Line::raw(title).alignment(Alignment::Center));
    f.render_widget(&title_block, area);
    title_block.inner(area)
}

pub fn render_line(f: &mut Frame, area: Rect) -> Rect {
    let line = Block::default().borders(Borders::RIGHT);
    f.render_widget(&line, area);
    line.inner(area)
}

pub fn render_footer(f: &mut Frame, mode: FooterMode, area: Rect) {
    let contents = mode.get_content();
    let count = contents.len();
    let constraints = vec![Constraint::Fill(1); count];
    let footer = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);
    match mode {
        FooterMode::Explorer => {
            for (msg, location) in contents.iter().zip(footer.iter()) {
                let text = Paragraph::new(*msg).alignment(Alignment::Center);
                f.render_widget(&text, *location);
            }
        }
        FooterMode::Warning(msg) => {
            let text = Paragraph::new(msg)
                .alignment(Alignment::Center)
                .style(TextStyle::Error.get_color());
            f.render_widget(&text, area);
        }
        _ => {
            // TODO(ui): Add footer render for detail.
        }
    }
}

pub fn render_explorer_item(f: &mut Frame, area: Rect, name: &str, style: TextStyle) {
    let text = Paragraph::new(name).style(style.get_color());
    f.render_widget(text, area);
}

pub fn render_edit(f: &mut Frame, area: Rect, text: &str, location: usize) {
    let mut spans = Vec::new();
    let mut in_str = false;
    for (i, c) in text.chars().enumerate() {
        let style = if i == location {
            in_str = true;
            Style::new()
                .bg(Color::Black)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::new()
                .bg(Color::White)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
        };

        spans.push(Span::styled(c.to_string(), style));
    }
    if !in_str {
        spans.push(Span::styled(
            " ",
            Style::new()
                .bg(Color::Black)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ));
    }
    let text = Paragraph::new(Line::from(spans));
    f.render_widget(text, area);
}

impl ConfigServerMap {
    pub fn render(&self, area: Rect) {}
}

*/
