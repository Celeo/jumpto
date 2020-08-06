use crate::config::Config;
use anyhow::Result;
use colored::Colorize;
use log::{debug, error, info, warn};
use prettytable::{cell, row, Table};
use std::env::current_dir;

pub(crate) fn command_list(config: &Config) -> Result<()> {
    if config.directories.is_empty() {
        info!("No directories saved!");
        info!("Use 'jumpto add <reference name> <path>' to add one");
        return Ok(());
    }
    let mut table = Table::new();
    table.set_titles(row!["Name", "Path"]);
    for dir in &config.directories {
        table.add_row(row![dir.0, dir.1]);
    }
    info!("{}", table);
    Ok(())
}

pub(crate) fn command_add(config: &mut Config, name: &str, path: &Option<String>) -> Result<()> {
    let path = match path {
        Some(p) => p.to_owned(),
        None => format!("{}", current_dir()?.display()),
    };
    let exists = config.directories.contains_key(name);
    debug!(
        "Adding path: name: '{}', path: '{}', exists: '{}'",
        name, path, exists
    );
    if exists {
        warn!("Overwriting previous stored name");
    }
    config.directories.insert(name.to_owned(), path);
    config.save()?;
    info!(
        "{}{}{}",
        "Added new entry under '".green(),
        name.cyan(),
        "'".green()
    );
    Ok(())
}

pub(crate) fn command_remove(config: &mut Config, name: &str) -> Result<()> {
    if !config.directories.contains_key(name) {
        error!("Did not find '{}' in the config", name);
        return Ok(());
    }
    config.directories.remove(name);
    config.save()?;
    info!("{}{}{}", "Key '".green(), name.cyan(), "' removed".green());
    Ok(())
}
