use crate::config::Config;
use crate::error::Error;
use crate::types::{CanvasFile, Course, Folder};
use reqwest::Response;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct Api {
    token: String,
    course_folder_cache: HashMap<u32, Vec<Folder>>,
}

const TMP_JSON_PATH: &str = "tmp.json";
fn tmp_json() -> PathBuf {
    PathBuf::from(TMP_JSON_PATH)
}

impl Api {
    pub fn new(config: &Config) -> Result<Self, Error> {
        if config.token().is_empty() {
            return Err(Error::EmptyToken);
        }
        Ok(Self {
            token: config.token().to_string(),
            course_folder_cache: HashMap::new(),
        })
    }

    /// Get the data of a request in text form immediately
    async fn text(&self, url: &str) -> Result<String, Error> {
        self.get(url).await?.text().await.map_err(|v| v.into())
    }

    /// Send off an authorized request.
    async fn get(&self, url: &str) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let req = client.get(url);
        req.header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await
            .map_err(|v| v.into())
    }

    /// Get a list of files of a folder.
    pub async fn get_files(
        &self,
        folder: &Folder,
    ) -> Result<Vec<CanvasFile>, Error> {
        let text = self.text(folder.files_url()).await?;
        let json = serde_json::from_str::<serde_json::Value>(&text)?;
        Ok(CanvasFile::get(&json))
    }

    /// Get a list of courses of the current user.
    pub async fn course_folders(
        &mut self,
        course_id: u32,
    ) -> Result<Vec<Folder>, Error> {
        if let Some(vec) = self.course_folder_cache.get(&course_id) {
            return Ok(vec.clone());
        }
        let url = format!(
            "https://canvas.nus.edu.sg/api/v1/courses/{course_id}/folders"
        );
        let text = self.text(&url).await?;
        let json = serde_json::from_str::<serde_json::Value>(&text)?;
        let vec = Folder::get(&json);
        self.course_folder_cache.insert(course_id, vec.clone());
        Ok(vec)
    }

    /// Uses the `url` as a download link and sends the data to the
    /// file at `local_path`. Will create a new filename if that file
    /// already exists.
    pub async fn download<P: AsRef<Path>>(
        &self,
        url: &str,
        local_path: P,
    ) -> Result<(), Error> {
        let path = local_path.as_ref();
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() && !parent.is_dir() {
                return Err(Error::DownloadNoParentDir(path.to_path_buf()));
            }
        }
        let path = match path.is_file() {
            true => {
                let base_name = path
                    .file_name()
                    .and_then(|v| v.to_str())
                    .ok_or(Error::InvalidFilename(path.to_path_buf()))?;
                let mut path = path.to_path_buf();
                let mut idx = 1;
                loop {
                    path.set_file_name(format!("{base_name}_({idx})"));
                    if !path.is_file() {
                        break;
                    }
                    idx += 1;
                }
                path
            }
            false => path.to_path_buf(),
        };
        let response = reqwest::get(url).await?;
        let mut target = File::create(path)?;
        let mut content = std::io::Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut target)?;
        Ok(())
    }

    /// Get a list of courses of the current user.
    /// Probably not required because course id will be part of the
    /// url supplied in the user config.
    #[allow(unused)]
    pub async fn list_courses(&self) -> Result<Vec<Course>, Error> {
        let text =
            self.text("https://canvas.nus.edu.sg/api/v1/courses").await?;
        let json = serde_json::from_str::<serde_json::Value>(&text)?;
        Ok(Course::get(&json))
    }

    /// Send a request to a file for easy reading.
    #[allow(unused)]
    pub async fn send_to_file(&self, url: &str) -> Result<(), Error> {
        let mut json_file = File::create(tmp_json())?;
        let text = self.text(url).await?;
        json_file.write_fmt(format_args!("{text}")).map_err(|v| v.into())
    }
}
