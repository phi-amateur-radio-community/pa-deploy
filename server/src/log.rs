// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/log.rs
// Logger

use std::{fs::create_dir_all, path::Path};
use tracing_appender::rolling;

pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

pub enum LogMode<'a> {
    File(&'a String),
    Stdout,
}

pub fn init_log(level: LogLevel, mode: LogMode) -> Result<(), std::io::Error> {
    let level = match level {
        LogLevel::Trace => "trace",
        LogLevel::Debug => "debug",
        LogLevel::Info => "info",
        LogLevel::Warning => "warning",
        LogLevel::Error => "error",
    };
    let builder = tracing_subscriber::fmt().with_env_filter(level);
    match mode {
        LogMode::Stdout => builder.init(),
        LogMode::File(path) => {
            if !Path::new(path).exists() {
                create_dir_all(path)?;
            }
            let appender = rolling::daily(path, "pa-deploy.log");
            builder.with_writer(appender).init();
        }
    };
    Ok(())
}
