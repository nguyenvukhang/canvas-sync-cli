use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Serializable folder map
#[derive(Serialize, Deserialize, Debug)]
pub struct SFolderMap {
    /// url of the user-facing folder page.
    url: String,
    /// local path to track the folder that the url points to.
    path: String,
}

#[derive(Debug)]
pub struct FolderMap {
    course_id: u32,
    folder_name: String,
    course_name: String,
    path: PathBuf,
    url: String,
}

impl FolderMap {
    pub fn new(
        sfolder_map: SFolderMap,
        base_path: &Option<PathBuf>,
    ) -> Result<Self, Error> {
        let (url, path) = (sfolder_map.url, sfolder_map.path);
        let (course_id, folder_name) = parse_url(&url)?;
        let path = match base_path {
            Some(v) => v.join(&path),
            None => PathBuf::from(&path),
        };

        match path.parent() {
            Some(v) if v.is_dir() => {}
            _ => return Err(Error::DownloadNoParentDir(path.to_owned())),
        };

        fs::create_dir_all(&path)?;

        let url = match urlencoding::decode(&url) {
            Ok(v) => v.to_string(),
            _ => url,
        };

        Ok(Self {
            course_id,
            folder_name,
            path,
            course_name: String::new(),
            url,
        })
    }

    pub fn course_id(&self) -> &u32 {
        &self.course_id
    }

    pub fn course_name(&self) -> &String {
        &self.course_name
    }

    pub fn set_course_name(&mut self, course_name: &str) {
        self.course_name = course_name.to_string()
    }

    pub fn folder_name(&self) -> &str {
        &self.folder_name
    }

    pub fn local_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn url(&self) -> &str {
        &self.url
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
