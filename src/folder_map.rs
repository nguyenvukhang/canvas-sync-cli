use crate::error::Error;
use crate::string::{parse_url, replace_tilde};
use crate::types::FileMap;
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
    /// Path on canvas
    remote_path: String,
    course_name: String,
    local_dir: PathBuf,
    url: String,
    files_url: Option<String>,
    file_map: Vec<FileMap>,
}

impl FolderMap {
    pub fn new(
        sfolder_map: SFolderMap,
        base_path: &Option<PathBuf>,
    ) -> Result<Self, Error> {
        let (url, path) = (sfolder_map.url, sfolder_map.path);
        let (course_id, remote_path) = parse_url(&url)?;
        let local_dir = match base_path {
            Some(v) => v.join(&path),
            None => match replace_tilde(&path) {
                Some(v) => v,
                None => {
                    return Err(Error::InvalidFilename(PathBuf::from(path)))
                }
            },
        };

        match local_dir.parent() {
            Some(v) if v.is_dir() => {}
            _ => return Err(Error::DownloadNoParentDir(local_dir.to_owned())),
        };

        fs::create_dir_all(&local_dir)?;

        let url = match urlencoding::decode(&url) {
            Ok(v) => v.to_string(),
            _ => url,
        };

        Ok(Self {
            file_map: vec![],
            files_url: None,
            course_id,
            remote_path,
            local_dir,
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

    pub fn set_files_url(&mut self, files_url: &str) {
        self.files_url = Some(files_url.to_string())
    }

    pub fn remote_path(&self) -> &str {
        &self.remote_path
    }

    pub fn local_dir(&self) -> &PathBuf {
        &self.local_dir
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn files_url(&self) -> Option<&String> {
        self.files_url.as_ref()
    }

    pub fn set_file_map(&mut self, file_map: Vec<FileMap>) {
        self.file_map = file_map
    }

    pub fn file_maps(&self) -> &Vec<FileMap> {
        &self.file_map
    }
}
