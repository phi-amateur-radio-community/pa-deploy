// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/arg.rs
// Handle command.

use crate::{
    conf::{Config, ConfigError},
    ui,
};
use clap::{Arg, ArgAction, Command};

#[derive(Debug, thiserror::Error)]
pub enum ArgError {
    #[error("Configure error")]
    Config(#[from] ConfigError),
    #[error("Missing path")]
    MissingPath,
    #[error("Ui error")]
    Ui(#[from] ui::UiError),
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
    match matches.subcommand() {
        Some(("config", sub_m)) => {
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
            println!("[config]: Unknown Arg");
        }
    }
    Ok(())
}
