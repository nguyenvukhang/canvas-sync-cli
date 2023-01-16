use crate::error::Error;
use crate::folder_map::FolderMap;
use crate::types::{Course, FileMap, Folder, User};
use futures::prelude::*;
use futures::FutureExt;
use reqwest::Response;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Api {
    /// `Bearer <canvas access token>`
    auth_header: String,
}

impl Api {
    pub fn new(access_token: &str) -> Self {
        debug_assert!(!access_token.is_empty());
        Self { auth_header: format!("Bearer {}", access_token) }
    }

    /// Send off an authorized request.
    async fn get(&self, url: &str) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let req = client.get(url);
        Ok(req.header("Authorization", &self.auth_header).send().await?)
    }

    /// Get the data of a request in json form.
    async fn json(&self, url: &str) -> Result<serde_json::Value, Error> {
        Ok(self.get(url).await?.json::<serde_json::Value>().await?)
    }

    /// Get the folders of a particular course id.
    async fn get_course_folders(
        &self,
        course_id: &u32,
    ) -> Result<Vec<Folder>, Error> {
        let url = format!(
            "https://canvas.nus.edu.sg/api/v1/courses/{course_id}/folders"
        );
        self.json(&url).await.map(|v| Folder::get_vec(&v, course_id))
    }

    /// Get a list of files of a folder.
    async fn get_files(
        &self,
        files_url: &str,
        parent: &PathBuf,
    ) -> Result<Vec<FileMap>, Error> {
        let json = self.json(files_url).await;
        json.map(|v| FileMap::get_vec(&v, parent))
    }

    /// Get a list of courses of the current user.
    pub async fn list_courses(&self) -> Result<Vec<Course>, Error> {
        let json =
            self.json("https://canvas.nus.edu.sg/api/v1/courses").await?;
        Ok(Course::get_vec(&json))
    }

    /// Gets all courses' folder contents in one parallel async call.
    pub async fn all_course_folders(
        &self,
        course_ids: &Vec<u32>,
    ) -> Result<Vec<Folder>, Error> {
        let handles = course_ids
            .iter()
            .map(|id| async move { self.get_course_folders(id).await });
        let results = resolve(handles, 5).await;
        fail_fast(results)
    }

    /// In each tracked folder, get all the remote files and load them
    /// into memory.
    pub async fn all_tracked_files(
        &self,
        folder_maps: &mut Vec<FolderMap>,
    ) -> Result<(), Error> {
        let handles = folder_maps.into_iter().map(|fm| async move {
            let files_url = fm.files_url().ok_or(Error::FilesUrlNotFound)?;
            let fetched = self.get_files(files_url, fm.local_dir()).await?;
            fm.set_file_map(fetched);
            Ok(())
        });
        let results = resolve(handles, 5).await;
        for result in results {
            if let Err(e) = result {
                return Err(e);
            }
        }
        Ok(())
    }

    pub async fn download_many(
        &self,
        maps: &Vec<FileMap>,
    ) -> Result<usize, Error> {
        let handles = maps.iter().map(|v| async move {
            self.download(v.download_url(), v.local_target()).await
        });
        let results = resolve(handles, 5).await;
        let mut count = 0;
        for result in results {
            match result {
                Ok(_) => count += 1,
                Err(e) => return Err(e),
            }
        }
        Ok(count)
    }

    /// Uses the `url` as a download link and sends the data to the
    /// file at `local_dir`. Will create a new filename if that file
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
        let path = avoid_filename_collision(&path)?;
        let response = reqwest::get(url)
            .await
            .map_err(|e| Error::DownloadErr(url.to_string(), e))?;
        let mut target = File::create(path)?;
        let mut content = std::io::Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut target)?;
        Ok(())
    }

    /// Prints basic information about the user to make sure that the
    /// access token is present and valid.
    pub async fn hello(&self) -> Result<(), Error> {
        let result =
            self.json("https://canvas.nus.edu.sg/api/v1/users/self").await;
        if let Ok(result) = result {
            if let Some(user) = User::get(&result) {
                println!("id: {}, name: {}", user.id(), user.name());
                return Ok(());
            }
        }
        Err(Error::UnableToGetUserData)
    }
}

/// Resolves handles in batches of size `threads`
async fn resolve<I, F>(handles: I, threads: usize) -> Vec<F::Output>
where
    I: IntoIterator<Item = F>,
    F: FutureExt,
{
    futures::stream::iter(handles)
        .buffer_unordered(threads)
        .collect::<Vec<F::Output>>()
        .await
}

/// Takes a list of results of lists and fails on the first one found.
/// If none fails, return the joined list.
fn fail_fast<T>(results: Vec<Result<Vec<T>, Error>>) -> Result<Vec<T>, Error> {
    let mut all = vec![];
    for result in results {
        let item = match result {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        all.extend(item);
    }
    Ok(all)
}

/// Keep incrementing the index number until the pathbuf doesn't exist
/// as a file. This is useful for handling collisions when downloading.
fn avoid_filename_collision(path: &PathBuf) -> Result<PathBuf, Error> {
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
    Ok(path)
}
