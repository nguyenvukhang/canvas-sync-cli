use crate::error::{Error, Result};
use crate::types::FolderMap;
use crate::BINARY_NAME;
use serde::{Deserialize, Serialize};
use std::path::Path;

const CONFIG_NAME: &str = "config";

/// Serializeable version of the config.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    access_token: String,
    base_path: Option<String>,
    #[serde(rename = "folders")]
    folder_maps: Vec<FolderMap>,
    #[serde(skip)]
    config_path: String,
}

impl Config {
    /// Loads canvas-sync config
    pub fn load<P>(config_path: Option<P>, check_token: bool) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let cfg_path =
            confy::get_configuration_file_path(BINARY_NAME, Some(CONFIG_NAME))?;
        if !cfg_path.is_file() {
            println!(
                "New config file created at\n'{}'\n",
                cfg_path.to_string_lossy()
            )
        }
        let mut config: Self = match config_path {
            Some(v) => confy::load_path(v),
            None => confy::load(BINARY_NAME, Some(CONFIG_NAME)),
        }
        .map_err(Error::ConfyErr)?;

        if check_token && config.access_token.is_empty() {
            return Err(Error::EmptyToken);
        }

        // insert base paths into each folder map
        config.folder_maps.iter_mut().for_each(|fm| {
            fm.set(config.base_path.clone());
        });

        config.config_path = cfg_path.to_string_lossy().to_string();

        Ok(config)
    }

    pub fn path(&self) -> &str {
        &self.config_path
    }

    pub fn set_token(&mut self, token: &str) {
        self.access_token = token.to_string();
    }

    /// Saves the current state of canvas-sync config
    pub fn save(&self) -> Result<()> {
        confy::store(BINARY_NAME, Some(CONFIG_NAME), self).map_err(|e| e.into())
    }

    /// Get folder maps
    pub fn folder_maps(&self) -> &Vec<FolderMap> {
        &self.folder_maps
    }

    /// Get access token
    pub fn access_token(&self) -> &str {
        &self.access_token
    }
}
