use crate::error::{Error, Result};
use crate::traits::*;
use futures::{FutureExt, StreamExt};
use reqwest::Response;
use serde_json::Value;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Api {
    /// Canvas access token
    access_token: String,
}

impl Api {
    pub fn new(access_token: &str) -> Self {
        Self { access_token: access_token.to_string() }
    }

    /// Send off an authorized request.
    async fn get(&self, url: &str) -> Result<Response> {
        let client = reqwest::Client::new();
        let req = client.get(url);
        Ok(req.bearer_auth(&self.access_token).send().await?)
    }

    /// Get the data of a request in json form.
    async fn json(&self, url: &str) -> Result<Value> {
        let json = self.get(url).await?.json::<Value>().await?;
        if json["errors"][0]["message"].eq("Invalid access token.") {
            return Err(Error::InvalidToken);
        }
        if json["errors"].as_array().map_or(false, |v| v.len() > 0) {
            let msg = json["errors"][0]["message"].to_str();
            return Err(Error::CanvasError {
                msg: msg.to_string(),
                url: url.to_string(),
            });
        }
        Ok(json)
    }

    /// Prints basic information about the user to make sure that the
    /// access token is present and valid.
    pub async fn profile(&self) -> Result<Value> {
        self.json("https://canvas.nus.edu.sg/api/v1/users/self/profile").await
    }

    /// Get a list of courses of the current user.
    pub async fn courses(&self) -> Result<Value> {
        self.json("https://canvas.nus.edu.sg/api/v1/courses?per_page=420").await
    }

    /// Get the files of a folder.
    pub async fn files(&self, folder_id: u32) -> Result<Value> {
        log::debug!("[API::FILES] {folder_id}");
        let url = format!(
            "https://canvas.nus.edu.sg/api/v1/folders/{folder_id}/files"
        );
        self.json(&url).await
    }

    /// Get the folders of a particular course id.
    pub async fn course_folders(&self, course_id: u32) -> Result<Value> {
        // log::info!("[API::COURSE FOLDERS] {course_id}");
        let url = format!(
            "https://canvas.nus.edu.sg/api/v1/courses/{course_id}/folders"
        );
        self.json(&url).await
    }

    /// Follows `url` to a file and downloads it to `filepath`.
    pub async fn download(self, url: String, filepath: PathBuf) -> Result<()> {
        log::info!("[API::DOWNLOAD] {filepath:?}");
        if filepath.is_file() {
            std::fs::remove_file(&filepath).ok();
        }
        if let Some(parent) = filepath.parent() {
            if parent.as_os_str().is_empty() || !parent.is_dir() {
                return Err(Error::DownloadNoParentDir(filepath.to_path_buf()));
            }
        }
        let mut target = File::create(&filepath)?;
        let response = reqwest::get(&url)
            .await
            .map_err(|e| Error::DownloadErr(url.to_string(), e))?;
        let mut content = std::io::Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut target)?;
        Ok(())
    }
}

/// Resolves handles in batches of size `threads`
pub async fn resolve<I, F>(handles: I, threads: usize) -> Vec<F::Output>
where
    I: IntoIterator<Item = F>,
    F: FutureExt,
{
    futures::stream::iter(handles)
        .buffer_unordered(threads)
        .collect::<Vec<F::Output>>()
        .await
}
