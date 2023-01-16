use crate::error::Error;
use crate::types::{CanvasFile, Course, Folder};
use futures::prelude::*;
use reqwest::Response;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Api {
    token: String,
    course_folder_cache: HashMap<u32, Vec<Folder>>,
}

impl Api {
    pub fn new(token: &str) -> Self {
        debug_assert!(!token.is_empty());
        Self { token: token.to_string(), course_folder_cache: HashMap::new() }
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
        Ok(CanvasFile::get_vec(&json, folder.full_name()))
    }

    /// Cached operation to get a list of folders within a course.
    /// This is needed to obtain each folder's id.
    pub async fn all_course_folders(
        &self,
        course_ids: &Vec<&u32>,
    ) -> Result<HashMap<u32, Vec<Folder>>, Error> {
        let url = |id| {
            format!("https://canvas.nus.edu.sg/api/v1/courses/{id}/folders")
        };
        let handles = course_ids.into_iter().map(|id| async move {
            let text = self.text(&url(id)).await.and_then(|v| {
                Ok((**id, serde_json::from_str::<serde_json::Value>(&v)?))
            });
            text
        });
        let results = futures::stream::iter(handles)
            .buffer_unordered(5)
            .map(|v| match v {
                Ok((id, v)) => Ok((id, Folder::get_vec(&v))),
                Err(e) => Err(e),
            })
            .collect::<Vec<_>>()
            .await;
        Ok(HashMap::from_iter(
            results.into_iter().collect::<Result<Vec<_>, Error>>()?,
        ))
    }

    // pub async fn

    /// Uses the `url` as a download link and sends the data to the
    /// file at `local_path`. Will create a new filename if that file
    /// already exists.
    pub async fn download(
        &self,
        url: &str,
        path: &PathBuf,
    ) -> Result<(), Error> {
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
        let response = reqwest::get(url)
            .await
            .map_err(|e| Error::DownloadErr(url.to_string(), e))?;
        let mut target = File::create(path)?;
        let mut content = std::io::Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut target)?;
        Ok(())
    }

    /// Get a list of courses of the current user.
    pub async fn list_courses(&self) -> Result<Vec<Course>, Error> {
        let text =
            self.text("https://canvas.nus.edu.sg/api/v1/courses").await?;
        let json = serde_json::from_str::<serde_json::Value>(&text)?;
        Ok(Course::get_vec(&json))
    }
}
