use crate::error::Error;
use crate::types::Course;
use std::io::ErrorKind;

use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigMapJson {
    url: String,
    path: String,
}

#[derive(Debug)]
pub struct ConfigMap {
    course_id: u32,
    folder_name: String,
    path: PathBuf,
    course_name: Option<String>,
}

impl ConfigMap {
    pub fn new<P: AsRef<Path>>(
        url: String,
        path: String,
        base_path: &Option<P>,
    ) -> Result<Self, Error> {
        let (course_id, folder_name) = parse_url(&url)?;
        let path = match base_path {
            Some(v) => v.as_ref().join(&path),
            None => replace_tilde(&path).unwrap_or(PathBuf::from(&path)),
        };
        // create the deepest directory, but not the parent.
        if let Some(parent) = path.parent() {
            if parent.is_dir() {
                fs::create_dir_all(&path)?;
            } else {
                return Err(Error::DownloadNoParentDir(path.to_owned()));
            }
        }

        Ok(Self { course_id, folder_name, path, course_name: None })
    }

    pub fn course_id(&self) -> &u32 {
        &self.course_id
    }

    pub fn course_name(&self) -> Option<&String> {
        self.course_name.as_ref()
    }

    pub fn folder_name(&self) -> &str {
        &self.folder_name
    }

    pub fn local_path(&self) -> &PathBuf {
        &self.path
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigJson {
    base_path: Option<String>,
    token: String,
    maps: Vec<ConfigMapJson>,
}

#[derive(Debug)]
pub struct Config {
    token: String,
    maps: Vec<ConfigMap>,
}

impl Config {
    /// Tries to get config from the path supplied. If no path supplied,
    /// then use the default path `./canvas.json`.
    pub fn new(path: PathBuf) -> Result<Config, Error> {
        let config_file = open_config(path)?;
        let mut config_json =
            serde_json::from_reader::<File, ConfigJson>(config_file)?;

        // load token from env if not found in json
        if config_json.token.is_empty() {
            config_json.token = get_canvas_token()?;
        }

        let base_path = match &config_json.base_path {
            Some(v) if v.is_empty() => None,
            None => None,
            Some(v) => Some(replace_tilde(&v).unwrap_or(PathBuf::from(v))),
        };

        let config_maps: Result<Vec<ConfigMap>, Error> = config_json
            .maps
            .into_iter()
            .map(|m| ConfigMap::new(m.url, m.path, &base_path))
            .collect();

        Ok(Config { maps: config_maps?, token: config_json.token })
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn maps(&self) -> &Vec<ConfigMap> {
        &self.maps
    }

    /// Modifies `self.map` to include course names, and thens sorts
    /// them by course name.
    pub fn load_course_names(
        &mut self,
        user_courses: &Vec<Course>,
    ) -> Result<(), Error> {
        for m in &mut self.maps {
            match user_courses.iter().find(|v| v.id().eq(&m.course_id)) {
                Some(found) => m.course_name = Some(found.name().to_string()),
                None => return Err(Error::CourseNotFound(m.course_id)),
            }
        }
        self.maps.sort_by(|a, b| a.course_name().cmp(&b.course_name()));
        Ok(())
    }
}

/// Opens the config and bubbles errors nicely.
fn open_config(path: PathBuf) -> Result<File, Error> {
    match File::open(&path) {
        Ok(v) => Ok(v),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Err(Error::ConfigNotFound(path)),
            _ => Err(e.into()),
        },
    }
}

/// Parses a url in a url-path config pair to extract course id and
/// full folder name.
///
/// Example input:
/// https://canvas.nus.edu.sg/courses/38518/files/folder/Lectures/Java%20Intro
///
/// Expected output:
/// (38518, "Lectures/Java Intro")
fn parse_url(url: &str) -> Result<(u32, String), Error> {
    let err = || Error::InvalidTrackingUrl(url.to_string());
    let url =
        url.strip_prefix("https://canvas.nus.edu.sg/courses/").ok_or(err())?;
    let (id, folder) = url.split_once("/").ok_or(err())?;
    let id = id.parse::<u32>().map_err(|_| err())?;
    let folder = folder.strip_prefix("files/folder/").ok_or(err())?;
    if let Ok(decoded) = urlencoding::decode(folder) {
        return Ok((id, decoded.to_string()));
    }
    Ok((id, folder.to_string()))
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

/// Reads the $CANVAS_TOKEN environment variable for the access token.
/// To obtain an access token: head over to
/// https://canvas.nus.edu.sg/profile/settings
/// and search for the word 'token'.
fn get_canvas_token() -> Result<String, Error> {
    std::env::var("CANVAS_TOKEN").map_err(|_| Error::CanvasEnvNotFound)
}
