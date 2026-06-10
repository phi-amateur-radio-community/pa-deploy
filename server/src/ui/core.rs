// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/ui/core.rs
// Structure and enum definition of TUI.

use crate::conf::Config;

#[allow(unused)]
pub struct Screen<'a> {
    config: Config,
    status: ScreenStatus<'a>,
    ptr: usize,
    detail_ptr: Option<usize>,
    changed: bool,
}

#[allow(unused)]
pub enum ScreenStatus<'a> {
    Move,
    Input(usize, &'a mut String),
}

#[allow(unused)]
impl Screen<'_> {
    fn input(&mut self) {
        // TODO(ui): input
    }
}
