use anyhow::{anyhow, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

const CONFIG_FILE_NAME: &str = ".jumpto.json";

#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct Config {
    pub(crate) directories: HashMap<String, String>,
}

impl Config {
    /// Return the path to the config file.
    pub(crate) fn path_to_file() -> Result<PathBuf> {
        Ok(Path::join(
            &home::home_dir().ok_or_else(|| anyhow!("Could not determine home directory"))?,
            CONFIG_FILE_NAME,
        ))
    }

    /// Load the config file from disk.
    ///
    /// If no config file is found, a new, empty config is
    /// returned instead. If this happens, the config file
    /// is still not created until `Config::save` is called.
    pub(crate) fn load_from_disk() -> Result<Config> {
        let path = Self::path_to_file()?;
        if path.exists() {
            debug!("Loading config from {}", path.display());
            let content = fs::read_to_string(path)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            debug!("No config file found; creating a new, empty config");
            Ok(Config::default())
        }
    }

    /// Save the config to disk.
    pub(crate) fn save(&self) -> Result<()> {
        let path = Self::path_to_file()?;
        let content = serde_json::to_string(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}
