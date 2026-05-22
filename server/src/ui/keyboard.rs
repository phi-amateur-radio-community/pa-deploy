// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/ui/keyboard.rs
// Keyboard handler

use super::core::{ScreenData, ScreenStatus};
use ratatui::crossterm::event::KeyCode;
use tracing::{debug, info};

const WARNING_UNSAVE: &str = "The configuration is not saved. Enter '!' if you want to force quit";

pub enum HandlerStatus {
    Break,
    Continue,
    Warning(&'static str),
}

pub fn handler_keyboard(screen: &mut ScreenData, code: KeyCode) -> HandlerStatus {
    if matches!(screen.get_status(), ScreenStatus::Warning(_)) && code == KeyCode::Char('!') {
        info!(target: "ui/keyboard", "Force exit");
        return HandlerStatus::Break;
    }
    match code {
        KeyCode::Char('c') => handler_create(screen),
        KeyCode::Char('q') => return handler_quit(screen),
        KeyCode::Char('d') => return HandlerStatus::Continue, // TODO(ui): add create new configure server.
        KeyCode::Char('r') => return HandlerStatus::Continue, // TODO(ui): add create new configure server.
        KeyCode::Char('m') => return HandlerStatus::Continue, // TODO(ui): add create new configure server.
        KeyCode::Char('s') => return HandlerStatus::Continue, // TODO(ui): add remove new configure server.
        _ => return HandlerStatus::Continue,
    }
    HandlerStatus::Continue
}

fn handler_create(screen: &mut ScreenData) {
    debug!(target: "ui/keyboard", "Create entry");
    screen.get_config_mut().create();
}

fn handler_quit(screen: &mut ScreenData) -> HandlerStatus {
    if screen.get_config().is_changed() {
        info!(target: "ui/keyboard", "File is not saved but user try quit");
        return HandlerStatus::Warning(WARNING_UNSAVE);
    }
    info!(target: "ui/keyboard", "Common exit");
    HandlerStatus::Break
}
