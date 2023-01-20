use crate::api::{self, Api};
use crate::error::{Error, Result};
use crate::traits::*;
use crate::types::{FolderMap, Update};

use futures::future::try_join_all;
use futures::FutureExt;

use std::collections::HashMap;
use std::mem;
use std::path::Path;

pub struct Sync<'a> {
    api: &'a Api,
    fm: &'a FolderMap,
    course_id: u32,
    remote_dir: String,
    download: bool,
}

impl<'a> Sync<'a> {
    pub fn new(
        api: &'a Api,
        fm: &'a FolderMap,
        download: bool,
    ) -> Result<Self> {
        if !fm.parent_exists() {
            return Err(Error::DownloadNoParentDir(fm.local_dir()));
        }
        let course_id = fm.course_id()?;
        let remote_dir = fm.remote_dir()?;
        Ok(Self { api, fm, download, course_id, remote_dir })
    }

    /// Terminal function of the `Sync` struct. Returns a list of all
    /// downloadables (url -> local path) pairs, and a list of all
    /// updates found.
    pub async fn get_updates(self) -> Result<Vec<Update>> {
        let course_id = self.course_id;
        let folders = self.api.course_folders(self.course_id).await?;
        let folders: Vec<(u32, String)> = folders
            .as_array()
            .ok_or(Error::NoFoldersFoundInCourse {
                url: self.fm.url().to_string(),
            })?
            .into_iter()
            .filter_map(|v| v.to_remote_folder(&self.remote_dir))
            .collect();

        if folders.is_empty() {
            let url = self.fm.url().to_string();
            return Err(Error::NoFoldersFoundInCourse { url });
        }

        let futures =
            folders.into_iter().map(|(folder_id, remote_path)| async move {
                self.api
                    .files(folder_id)
                    .map(move |files| files.map(|f| (remote_path, f)))
                    .await
            });
        let folders = api::resolve(futures, 10).await;
        let folders = folders.into_iter().collect::<Result<Vec<_>>>()?;

        let local_dir = self.fm.local_dir();

        // Parse a `Vec` out of the JSON within `files`
        let folders = folders.into_iter().map(|(remote_path, files)| {
            let files = match files.as_array() {
                Some(v) => v,
                None => {
                    let err = Error::NoFoldersFoundInCourse {
                        url: self.fm.url().to_string(),
                    };
                    return Err(err);
                }
            };
            let remote_path = Path::new(&remote_path);
            let updates: Vec<Update> = files
                .into_iter()
                .filter_map(|f| {
                    let url = f["url"].as_str()?.to_string();
                    let filename = f.to_normalized_filename()?;
                    let final_dir = local_dir.join(&remote_path);
                    let target_file = final_dir.join(&filename);
                    let has = target_file.is_file();
                    let get = self.download && !has;
                    if get {
                        std::fs::create_dir_all(&final_dir).ok();
                    }
                    (!has).then(|| {
                        Update::new(
                            course_id,
                            remote_path.join(&filename),
                            get.then(|| (url, target_file)),
                        )
                    })
                })
                .collect();
            Ok(updates)
        });

        let folders: Result<Vec<Vec<_>>> = folders.collect();
        Ok(folders?.into_iter().flatten().collect())
    }

    pub async fn run(
        api: &Api,
        updates: Vec<Update>,
        download: bool,
    ) -> Result<()> {
        let mut downloads = vec![];

        let mut updates = updates
            .into_iter()
            .map(|mut v| {
                if let Some((url, local_path)) = mem::take(&mut v.download) {
                    downloads.push(api.clone().download(url, local_path))
                }
                v
            })
            .collect::<Vec<Update>>();

        let user_courses = api.courses().await?.to_value_vec();
        let course_hash: HashMap<u32, &str> = user_courses
            .iter()
            .map(|j| {
                let course_id = j["id"].to_u32();
                let course_name = j["name"].to_str();
                (course_id, course_name)
            })
            .collect();

        // sort updates by module name
        updates.sort_by(|a, b| {
            course_hash.get(&a.course_id).cmp(&course_hash.get(&b.course_id))
        });

        display_updates(&updates, &course_hash);
        if !download && !updates.is_empty() {
            println!("! Fetch only. Nothing downloaded.");
        }
        try_join_all(downloads).await?;
        Ok(())
    }
}

fn display_updates(updates: &Vec<Update>, course_names: &HashMap<u32, &str>) {
    if updates.is_empty() {
        println!("No new files found. All up to date!");
        return;
    }
    let mut prev_id = 0;
    for update in updates {
        if update.course_id != prev_id {
            prev_id = update.course_id;
            match course_names.get(&update.course_id) {
                Some(v) => println!("{v}"),
                None => println!(
                    "Error: failed to fetch course with id {}",
                    update.course_id
                ),
            }
        }
        println!("  + {}", update.remote_path.to_string_lossy());
    }
}
