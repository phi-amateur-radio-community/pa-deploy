// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/ui/keyboard.rs
// Keyboard handler

use super::core::{ScreenData, ScreenStatus};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tracing::{debug, info, trace};

/*

const WARNING_UNSAVE: &str = "The configuration is not saved. Enter '!' if you want to force quit";
const WARNING_ASCII: &str = "The input is only support ascii";

pub enum HandlerAction {
    Break,
    Continue,
    Create,
    Quit,
    Move(MoveAction),
    ChangeStatus(ScreenStatus),
}

pub enum MoveAction {
    Up,
    Down,
    Left,
    Right,
}

impl ScreenData {
fn handler_keyboard(&self, key: KeyEvent) -> HandlerAction {
    match self.status {
        ScreenStatus::Input(p, s) => self.handler_keyboard_input(p, s, key),
        _ => self.handler_keyboard_move(key),
    }
}

fn handler_keyboard_input(&self, ptr: usize, string: &mut String, key: KeyEvent) -> HandlerAction {
    match key.code {
        KeyCode::Esc => HandlerAction::ChangeStatus(ScreenStatus::Move),
        KeyCode::Left => {
            if let Input(location) = screen.get_status()
                && location > 0
            {
                HandlerAction::ChangeStatus(ScreenStatus::Input(location - 1, string))
            } else {
                HandlerAction::Continue
            }
        }
        KeyCode::Right => {
            let (key, _) = screen.get_config().get_map().get_index(self.ptr)?;
            if let Input(location) = screen.get_status()
                && location > 0
            {
                HandlerAction::ChangeStatus(ScreenStatus::Input(location - 1, string))
            } else {
                HandlerAction::Continue
            }
        }
        KeyCode::Backspace => {
            if let Some(location) = ptr.checked_sub(1) {
                remove_at(string, location);
            }
            HandlerAction::Continue
        }
        KeyCode::Delete => {
            remove_at(string, ptr);
            HandlerAction::Continue
        }
        KeyCode::Char(c) => {
            if c.is_ascii() {
                string.insert(ptr, c);
                HandlerAction::Continue
            } else {
                HandlerAction::ChangeStatus(ScreenStatus::Warning(WARNING_ASCII))
            }
        }
        _ => HandlerAction::Continue;
    }
}

fn handler_keyboard_move(&self, key: KeyEvent) -> HandlerAction {
    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        info!(target: "ui/keyboard", "Ctrl + C exit");
        return HandlerAction::Break;
    }
    if matches!(self.status, ScreenStatus::Warning(_)) && key.code == KeyCode::Char('!') {
        info!(target: "ui/keyboard", "Force exit");
        return HandlerAction::Break;
    }
    match key.code {
        KeyCode::Up => HandlerAction::Move(MoveAction::Up),
        KeyCode::Down => HandlerAction::Move(MoveAction::Down),
        KeyCode::Left => HandlerAction::Move(MoveAction::Left),
        KeyCode::Right => HandlerAction::Move(MoveAction::Right),
        KeyCode::Char('c') => HandlerAction::Create,
        KeyCode::Char('q') => HandlerAction::Quit,
        KeyCode::Char('d') => HandlerAction::Continue, // TODO(ui): add create new configure server.
        KeyCode::Char('r') => HandlerAction::Continue, // TODO(ui): add create new configure server.
        KeyCode::Char('m') => HandlerAction::Continue, // TODO(ui): add create new configure server.
        KeyCode::Char('s') => HandlerAction::Continue, // TODO(ui): add remove new configure server.
        _ => HandlerAction::Continue,
    }
}
}

fn remove_at(string: &mut String, location: usize) {
    string.remove(location);
}

*/
