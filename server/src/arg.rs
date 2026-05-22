// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/arg.rs
// Handle command.

use crate::{
    conf::{Config, ConfigError},
    log::{LogLevel, LogMode, init_log},
    ui,
};
use clap::{Arg, ArgAction, Command};
use tracing::{error, info};

#[derive(Debug, thiserror::Error)]
pub enum ArgError {
    #[error("Configure error")]
    Config(#[from] ConfigError),
    #[error("Missing path")]
    MissingPath,
    #[error("Unknown error")]
    Unknown,
    #[error("Ui error")]
    Ui(#[from] ui::UiError),
    #[error("IO error")]
    Io(#[from] std::io::Error),
}

fn spawn_common_help(cmd: Command) -> Command {
    cmd
        .after_help("Author: Phiarc Team <phiarc@hotmail.com>\n\nLicense:\n  Copyright (c) 2026 Phiarc Team and St Rangeset\n  Licensed under the GPLv3 or later License.")
        .arg_required_else_help(true)
}

fn build_cli() -> Command {
    spawn_common_help(
        Command::new("PA Deploy Client Configure Manager")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about("PA Deploy Client")
            .disable_version_flag(true)
            .arg(
                Arg::new("version")
                    .long("version")
                    .short('V')
                    .help("Show version information")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("log-level")
                    .long("log-level")
                    .help("Log level: TRACE | DEBUG | INFO | WARNING | ERROR | NONE")
                    .default_value("INFO")
                    .value_name("LOG_LEVEL"),
            )
            .arg(
                Arg::new("log-mode")
                    .long("log-mode")
                    .help("Log out mode: stdout | file | disable")
                    .default_value("stdout")
                    .value_name("LOG_OUT_MODE"),
            )
            .arg(
                Arg::new("log-path")
                    .long("log-path")
                    .help("Set the path to save the log file")
                    .default_value("/var/log/pa-deploy")
                    .value_name("LOG_PATH"),
            )
            .subcommand(
                Command::new("config")
                    .about("Manage configuration files and options")
                    .arg(
                        Arg::new("path")
                            .help("Path to the configuration file, default /etc/padeploy/conf.toml")
                            .default_value("/etc/padeploy/conf.toml")
                            .value_name("PATH"),
                    ),
            ),
    )
}

pub fn handle_cli() -> Result<(), ArgError> {
    let matches = build_cli().get_matches();
    if matches.get_flag("version") {
        println!(
            "PA Deploy Client {}\nCopyright (c) 2026 Phiarc Team and St Rangeset\nLicensed under the GPLv3 or later License.",
            env!("CARGO_PKG_VERSION")
        );
        return Ok(());
    }
    let log_level = match matches
        .get_one::<String>("log-level")
        .ok_or(ArgError::Unknown)?
        .to_ascii_uppercase()
        .as_str()
    {
        "TRACE" => LogLevel::Trace,
        "DEBUG" => LogLevel::Debug,
        "INFO" => LogLevel::Info,
        "WARNING" => LogLevel::Warning,
        "ERROR" => LogLevel::Error,
        "NONE" => LogLevel::None,
        msg => {
            eprintln!("[arg] Unknown log level: {}", msg);
            return Ok(());
        }
    };
    let log_mode = match matches
        .get_one::<String>("log-mode")
        .ok_or(ArgError::Unknown)?
        .to_ascii_lowercase()
        .as_str()
    {
        "stdout" => LogMode::Stdout,
        "file" => LogMode::File(
            matches
                .get_one::<String>("log-path")
                .ok_or(ArgError::Unknown)?,
        ),
        "disable" => LogMode::Disable,
        msg => {
            eprintln!("[arg] Unknown log mode: {}", msg);
            return Ok(());
        }
    };
    init_log(&log_level, &log_mode)?;
    info!(target: "arg", argv = ?std::env::args().collect::<Vec<_>>(), "exec");
    match matches.subcommand() {
        Some(("config", sub_m)) => {
            if matches!(log_mode, LogMode::Stdout) {
                error!(target: "arg", "Terminal UI is not supported stdout log mode");
                return Ok(());
            }
            let path = sub_m
                .get_one::<String>("path")
                .ok_or(ArgError::MissingPath)?;
            let config = Config::new(path)?;
            let mut screen = ui::ScreenData::new(config);
            screen.display()?;
            let config = screen.free();
            config.save(path)?;
        }
        _ => {
            error!(target: "arg", "Unknown command");
        }
    }
    Ok(())
}
