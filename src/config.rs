use crate::api::Api;
use crate::error::Error;
use crate::folder_map::{FolderMap, SFolderMap};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const APP_NAME: &str = "canvas-sync";
const CONFIG_NAME: &str = "config";

/// Serializeable version of the config.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SConfig {
    access_token: String,
    base_path: Option<String>,
    folders: Vec<SFolderMap>,
}

#[derive(Debug)]
pub struct Config {
    access_token: String,
    folders: Vec<FolderMap>,
    api: Api,
}

// Serializeable Config {{{
impl SConfig {
    /// Loads canvas-sync config
    fn load<P>(config_path: Option<P>) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        match config_path {
            Some(v) => confy::load_path(v),
            None => confy::load(APP_NAME, Some(CONFIG_NAME)),
        }
        .map_err(|e| e.into())
    }

    /// Saves the current state of canvas-sync config
    fn save(&self) -> Result<(), Error> {
        confy::store(APP_NAME, Some(CONFIG_NAME), self).map_err(|e| e.into())
    }

    /// Gets the path to canvas-sync config
    fn get_path() -> Result<PathBuf, Error> {
        confy::get_configuration_file_path(APP_NAME, Some(CONFIG_NAME))
            .map_err(|e| e.into())
    }
}
/// }}}

impl Config {
    pub fn load<P>(config_path: Option<P>) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        // Parse the raw config file with the serializeable struct
        let sconfig = SConfig::load(config_path)?;

        if sconfig.access_token.is_empty() {
            return Err(Error::EmptyToken);
        }

        // un-tilde the base_path
        let base_path = sconfig.base_path.and_then(|v| replace_tilde(&v));

        // map each serializeable folder to its proper form
        let folders: Result<Vec<FolderMap>, Error> = sconfig
            .folders
            .into_iter()
            .map(|v| FolderMap::new(v, &base_path))
            .collect();

        Ok(Self {
            folders: folders?,
            api: Api::new(&sconfig.access_token),
            access_token: sconfig.access_token,
        })
    }

    pub async fn fetch_all_folders(&self) {
        let course_ids = &self.folders.iter().map(|v| v.course_id());

        // let url = format!(
        //     "https://canvas.nus.edu.sg/api/v1/courses/{course_id}/folders"
        // );
        // let text = self.text(&url).await?;
        // let json = serde_json::from_str::<serde_json::Value>(&text)?;
    }

    /// Modifies `self.folders` to include course names, and then sorts
    /// them by course name.
    pub async fn load_course_names(&mut self) -> Result<(), Error> {
        let user_courses = self.api.list_courses().await?;
        for f in &mut self.folders {
            match user_courses.iter().find(|v| v.id().eq(f.course_id())) {
                Some(found) => f.set_course_name(&found.name()),
                None => {
                    return Err(Error::CourseNotFound(
                        *f.course_id(),
                        f.url().to_string(),
                    ))
                }
            }
        }
        self.folders.sort_by(|a, b| a.course_name().cmp(&b.course_name()));
        Ok(())
    }

    /// Get a canvas api handler.
    pub fn get_api(&self) -> Api {
        Api::new(&self.access_token)
    }

    /// Get a reference to the folders.
    pub fn folders(&self) -> &Vec<FolderMap> {
        &self.folders
    }
}

/// Replace tilde with home.
fn replace_tilde(v: &str) -> Option<PathBuf> {
    if let Some(homeless) = v.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return Some(home.join(homeless));
        }
    }
    None
}
