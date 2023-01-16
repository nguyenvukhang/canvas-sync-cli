use crate::api::Api;
use crate::error::Error;
use crate::string::replace_tilde;
use crate::folder_map::{FolderMap, SFolderMap};
use crate::types::FileMap;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    folder_maps: Vec<FolderMap>,
    api: Api,
}

// Serializeable Config {{{
impl SConfig {
    /// Loads canvas-sync config
    pub fn load<P>(config_path: Option<P>) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        match config_path {
            Some(v) => confy::load_path(v),
            None => confy::load(APP_NAME, Some(CONFIG_NAME)),
        }
        .map_err(|e| e.into())
    }

    pub fn set_token(&mut self, token: &str) {
        self.access_token = token.to_string();
    }

    /// Saves the current state of canvas-sync config
    pub fn save(&self) -> Result<(), Error> {
        confy::store(APP_NAME, Some(CONFIG_NAME), self).map_err(|e| e.into())
    }

    /// Gets the path to canvas-sync config
    fn get_path(&self) -> Result<PathBuf, Error> {
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

        Ok(Self { folder_maps: folders?, api: Api::new(&sconfig.access_token) })
    }

    /// Gets the path to canvas-sync config
    pub fn get_path(&self) -> Result<PathBuf, Error> {
        confy::get_configuration_file_path(APP_NAME, Some(CONFIG_NAME))
            .map_err(|e| e.into())
    }

    /// Modifies `self.folders` to include course names, and then sorts
    /// them by course name.
    pub async fn load_course_names(&mut self) -> Result<(), Error> {
        let user_courses = self.api.list_courses().await?;
        for f in &mut self.folder_maps {
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
        self.folder_maps.sort_by(|a, b| a.course_name().cmp(&b.course_name()));
        Ok(())
    }

    /// Fetches all folders in a parallel API call. Requires all
    /// course ids to be valid.
    pub async fn fetch_all_folders(&mut self) -> Result<(), Error> {
        let unique_course_ids = self.unique_course_ids();
        let mut folders =
            self.api.all_course_folders(&unique_course_ids).await?;
        log::info!(
            "{} courses, fetched {} folders",
            unique_course_ids.len(),
            folders.len()
        );
        for fm in &mut self.folder_maps {
            let hit = folders.iter().position(|v| {
                v.course_id().eq(fm.course_id())
                    && v.remote_path().eq(fm.remote_path())
            });
            match hit {
                None => {
                    return Err(Error::FolderNotFound(
                        fm.course_name().to_string(),
                        fm.remote_path().to_string(),
                    ))
                }
                Some(v) => fm.set_files_url(folders.swap_remove(v).files_url()),
            };
        }
        Ok(())
    }

    /// Fetches all files that are being tracked, using
    /// `self.folder_maps` as a checklist, and then loads them into
    /// `self.folder_maps`.
    pub async fn fetch_all_files(&mut self) -> Result<(), Error> {
        self.api.all_tracked_files(&mut self.folder_maps).await
    }

    /// Get a list of unique course ids.
    fn unique_course_ids(&self) -> Vec<u32> {
        let mut ids =
            self.folder_maps.iter().map(|v| *v.course_id()).collect::<Vec<_>>();
        ids.sort();
        ids.dedup();
        ids
    }

    pub fn get_updates(&self) -> Vec<FileMap> {
        let mut updates: HashMap<String, Vec<FileMap>> = HashMap::new();
        for fm in &self.folder_maps {
            let file_maps = fm.file_maps();
            let course_name = fm.course_name();
            for file_map in file_maps {
                let local = file_map.local_target();
                if local.is_file() {
                    continue;
                }
                updates
                    .entry(course_name.to_string())
                    .and_modify(|v| v.push(file_map.clone()))
                    .or_insert(vec![file_map.clone()]);
            }
        }
        let mut updates = Vec::from_iter(updates);
        if updates.is_empty() {
            return vec![];
        }
        println!("{}", "UPDATES".green());
        updates.sort_by(|a, b| a.0.cmp(&b.0));
        for (course, file_maps) in &updates {
            println!("{course}");
            for file_map in file_maps {
                println!(
                    "  {}",
                    file_map.local_target().as_os_str().to_string_lossy()
                )
            }
        }
        updates.into_iter().flat_map(|v| v.1).collect()
    }

    pub async fn run(&self, download: bool) -> Result<(), Error> {
        println!("{}", "SCAN LIST".green());

        let mut prev_course = "";
        let asterisk = "*".green();
        for fm in &self.folder_maps {
            // print course name once per course
            if !fm.course_name().eq(prev_course) {
                prev_course = fm.course_name();
                println!("{}", fm.course_name());
            }
            println!("  {asterisk} {}", fm.remote_path());
        }
        println!();

        let updates = self.get_updates();

        if updates.is_empty() {
            println!("No new files found. All up to date!");
            return Ok(());
        }

        let count = updates.len();

        if !download {
            println!("{count} new files found. Skipping download.");
        }

        println!("Downloading {count} files...");
        let result = self.api.download_many(&updates).await?;
        println!("Successfully downloaded {result} files");
        Ok(())
    }

    pub async fn hello(&self) -> Result<(), Error> {
        self.api.hello().await
    }
}
