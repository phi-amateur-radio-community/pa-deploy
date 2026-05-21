// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/log.rs
// Logger

pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

pub fn init_log(level: LogLevel) -> Result<(), std::io::Error> {
    let level = match level {
        LogLevel::Trace => "trace",
        LogLevel::Debug => "debug",
        LogLevel::Info => "info",
        LogLevel::Warning => "warning",
        LogLevel::Error => "error",
    };
    tracing_subscriber::fmt()
        .with_writer(std::fs::File::create("pa-deploy.log")?)
        .with_env_filter(level)
        .init();
    Ok(())
}
