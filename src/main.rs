//! Simple CLI utility to navigate to directories

#![deny(clippy::all)]

use anyhow::Result;
use colored::Colorize;
use fern::{
    colors::{Color, ColoredLevelConfig},
    Dispatch,
};
use log::{debug, error, info, LevelFilter};
use std::{env, io};
use structopt::StructOpt;

mod config;
use config::Config;
mod commands;
use commands::{command_add, command_list, command_remove};

#[derive(Debug, StructOpt)]
enum Command {
    /// List all stored directories
    List {},
    /// Add another directory to quickly jump to. 'where_to' is optional;
    /// it will default to your current directory
    Add {
        name: String,
        where_to: Option<String>,
    },
    /// Remove a saved directory
    Remove { name: String },
}

#[derive(Debug, StructOpt)]
struct CliOptions {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(subcommand)]
    cmd: Option<Command>,

    where_to: Option<String>,
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

    if let Some(where_to) = cli_options.where_to {
        if let Some(path) = config.directories.get(&where_to) {
            info!("{} {}", "Jumping to".green().bold(), path.cyan().bold());
            if let Err(e) = env::set_current_dir(path) {
                error!("Could not change directories: {}", e);
                return;
            }
        /*
            TODO
                Successful directory change.
                Now need to set up the shell wrapper around this binary so
                that the shell will take the output of this, determine
                whether or not to take that string as the new directory,
                and then 'cd' there.
        */
        } else {
            error!("Unknown jump name '{}'", where_to);
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
        Some(Command::Add { name, where_to }) => {
            if let Err(e) = command_add(&mut config, &name, &where_to) {
                error!("Could not add new path: {}", e);
            }
        }
        Some(Command::Remove { name }) => {
            if let Err(e) = command_remove(&mut config, &name) {
                error!("Could not remove path: {}", e);
            }
        }
    }
}
