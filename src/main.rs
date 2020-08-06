//! Simple CLI utility to navigate to directories

#![deny(clippy::all)]

use anyhow::Result;
use colored::Colorize;
use fern::{
    colors::{Color, ColoredLevelConfig},
    Dispatch,
};
use log::{debug, error, info, LevelFilter};
use std::{io, path::Path};
use structopt::StructOpt;

mod config;
use config::Config;
mod commands;
use commands::{command_add, command_list, command_remove};

const SCRIPT: &str = include_str!("jt");

#[derive(Debug, StructOpt)]
enum Command {
    /// List all stored directories
    List {},
    /// Add another directory to quickly jump to. 'name' is optional;
    /// it will default to your current directory
    Add { name: String, path: Option<String> },
    /// Remove a saved directory
    Remove { name: String },
    /// Show the Bash script that needs to be used alongside this binary
    Script {},
}

#[derive(Debug, StructOpt)]
struct CliOptions {
    /// Whether to enable debug logging
    #[structopt(short, long)]
    debug: bool,

    #[structopt(subcommand)]
    cmd: Option<Command>,

    /// Name of the stored directory to jump to
    name: Option<String>,
}

/// Setup logging.
///
/// Minimum logging level is either Debug if the passed bool is true,
/// or Info otherwise. Logging is to stdout.
fn setup_logging(debug: bool) -> Result<()> {
    let level = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    let base_config = Dispatch::new().level(level);
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);
    let stdout_config = Dispatch::new()
        .format(move |out, message, record| {
            if record.level() == LevelFilter::Info {
                out.finish(format_args!("{}", message))
            } else {
                out.finish(format_args!(
                    "[{}] {} {}",
                    record.target(),
                    colors.color(record.level()),
                    message
                ))
            }
        })
        .chain(io::stdout());
    base_config.chain(stdout_config).apply()?;
    Ok(())
}

/// Entry point.
fn main() {
    let cli_options = CliOptions::from_args();
    setup_logging(cli_options.debug).expect("Could not set up logging");
    debug!("Runtime options: {:?}", cli_options);
    let mut config = Config::load_from_disk().expect("Could not load/create config");
    debug!("Config: {:?}", config);

    colored::control::set_override(true);

    if let Some(name) = cli_options.name {
        if let Some(path) = config.directories.get(&name) {
            if !Path::new(path).exists() {
                error!("Directory '{}' does not exist", path);
            } else {
                info!("{} {}", "Jumping to".green().bold(), path.cyan().bold());
            }
        } else {
            error!("Unknown jump name '{}'", name);
            info!("You can see the paths you have saved with 'jumpto list'")
        }
        return;
    }

    match cli_options.cmd {
        Some(Command::List {}) | None => {
            if cli_options.cmd.is_none() {
                info!("{}", "Showing saved paths ...".green());
            }
            if let Err(e) = command_list(&config) {
                error!("Could not list paths: {}", e);
            }
        }
        Some(Command::Add { name, path }) => {
            if let Err(e) = command_add(&mut config, &name, &path) {
                error!("Could not add new path: {}", e);
            }
        }
        Some(Command::Remove { name }) => {
            if let Err(e) = command_remove(&mut config, &name) {
                error!("Could not remove path: {}", e);
            }
        }
        Some(Command::Script {}) => {
            info!("{}", SCRIPT);
        }
    }
}
