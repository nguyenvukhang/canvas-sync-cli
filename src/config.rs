use crate::error::Error;
use std::io::ErrorKind;

use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::PathBuf;

const DEFAULT_CONFIG_PATH: &str = "./canvas.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigMap {
    course_id: Option<u32>,
    folder_name: Option<String>,
    url: String,
    path: String,
    processed_path: Option<PathBuf>,
}

impl ConfigMap {
    pub fn course_id(&self) -> Option<&u32> {
        self.course_id.as_ref()
    }

    pub fn folder_name(&self) -> Option<&String> {
        self.folder_name.as_ref()
    }

    pub fn local_path(&self) -> Option<&PathBuf> {
        self.processed_path.as_ref()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    base_path: Option<String>,
    token: String,
    maps: Vec<ConfigMap>,
}

impl Config {
    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn maps(&self) -> &Vec<ConfigMap> {
        &self.maps
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

/// Tries to get config from the path supplied. If no path supplied,
/// then use the default path `./canvas.json`.
pub fn get_config(path: Option<PathBuf>) -> Result<Config, Error> {
    let path = path.unwrap_or(PathBuf::from(DEFAULT_CONFIG_PATH));
    let config_file = open_config(path)?;
    let mut config = serde_json::from_reader::<File, Config>(config_file)?;
    if config.token.is_empty() {
        config.token = get_canvas_token()?;
    }
    let base_path = match &config.base_path {
        Some(v) if v.is_empty() => None,
        None => None,
        Some(v) => Some(replace_tilde(&v).unwrap_or(PathBuf::from(v))),
    };
    for m in config.maps.iter_mut() {
        let (course_id, folder_name) = parse_url(&m.url)?;
        m.course_id = Some(course_id);
        m.folder_name = Some(folder_name);
        let processed_path = match base_path.as_ref() {
            Some(v) => v.join(&m.path),
            None => replace_tilde(&m.path).unwrap_or(PathBuf::from(&m.path)),
        };
        // create the deepest directory, but not the parent.
        if let Some(parent) = processed_path.parent() {
            if parent.is_dir() {
                fs::create_dir_all(&processed_path)?;
            } else {
                let path = processed_path.to_owned();
                return Err(Error::DownloadNoParentDir(path));
            }
        }
        m.processed_path = Some(processed_path)
    }
    // TODO: implement base config checks
    // * validate path (don't create new directories)
    // * validate url
    Ok(config)
}

/// Reads the $CANVAS_TOKEN environment variable for the access token.
/// To obtain an access token: head over to
/// https://canvas.nus.edu.sg/profile/settings
/// and search for the word 'token'.
fn get_canvas_token() -> Result<String, Error> {
    std::env::var("CANVAS_TOKEN").map_err(|_| Error::CanvasEnvNotFound)
}
