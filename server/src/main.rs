// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/main.rs
// Main of PA Deploy client.

mod arg;
mod conf;
mod log;
// mod ui;

fn main() {
    let _ = arg::handle_cli();
}
