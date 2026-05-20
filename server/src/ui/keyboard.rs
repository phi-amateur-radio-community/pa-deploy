// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/ui/keyboard.rs
// Keyboard handler

use super::core::ScreenData;
use ratatui::crossterm::event::KeyCode;

pub enum HandlerStatus {
    Break,
    Continue,
}

pub fn handler_keyboard(screen: &mut ScreenData, code: KeyCode) -> HandlerStatus {
    match code {
        KeyCode::Char('c') => handler_create(screen),
        KeyCode::Char('q') => return HandlerStatus::Break,
        KeyCode::Char('d') => return HandlerStatus::Continue, // TODO(ui): add create new configure server.
        KeyCode::Char('r') => return HandlerStatus::Continue, // TODO(ui): add create new configure server.
        KeyCode::Char('m') => return HandlerStatus::Continue, // TODO(ui): add create new configure server.
        KeyCode::Char('s') => return HandlerStatus::Continue, // TODO(ui): add remove new configure server.
        _ => return HandlerStatus::Continue,
    }
    HandlerStatus::Continue
}

fn handler_create(screen: &mut ScreenData) {
    screen.get_config_mut().create();
}
