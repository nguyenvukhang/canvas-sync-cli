use crate::api::Api;
use crate::config::Config;
use crate::error::{Error, Result};
use crate::traits::*;
use futures::{FutureExt, StreamExt};
use std::collections::HashMap;

#[derive(Debug)]
struct Folder {
    id: u32,
    name: String,
    path: String,
    folders: Vec<Folder>,
    files: Vec<File>,
}

impl Folder {
    pub fn new(id: u32, path: &str) -> Self {
        let name = match path.rfind('/') {
            Some(v) => &path[v + 1..],
            None => path,
        }
        .to_string();

        Self {
            id,
            name,
            files: vec![],
            folders: vec![],
            path: path.to_string(),
        }
    }
}

#[derive(Debug)]
struct File {
    id: u32,
    name: String,
}

#[derive(Debug)]
struct Course {
    id: u32,
    name: String,
    files: Vec<File>,
    folders: Vec<Folder>,
}

#[derive(Debug)]
struct Courses {
    db: HashMap<u32, Course>,
    names: HashMap<u32, String>,
    api: Api,
}

trait InsertByPath {
    fn insert_by_path(&mut self, current_dir: &str, folder: Folder);
}

impl InsertByPath for &mut Vec<Folder> {
    fn insert_by_path(&mut self, current_dir: &str, folder: Folder) {
        let target_dir = folder.path.to_string();
        if !target_dir.starts_with(current_dir) {
            panic!("Path {target_dir} does not belong to {current_dir}")
        }
        let mut target_dir = &target_dir[current_dir.len()..];
        if target_dir.starts_with('/') {
            target_dir = &target_dir[1..];
        }

        // found it, send it
        if !target_dir.contains('/') {
            log::info!("sent {} to {}", folder.name, current_dir);
            log::debug!("target dir: {target_dir}");
            self.push(folder);
            return;
        }

        // target_dir is guaranteed to have a '/'
        // `next_dir` is equal to the value of target_dir before the first slash
        let next_dir = &target_dir[..target_dir.find('/').unwrap()];
        if let Some(next_target) = self.iter_mut().find(|v| v.name == next_dir)
        {
            let mut ptr = &mut next_target.folders;
            ptr.insert_by_path(&next_target.path, folder)
        }
    }
}

impl Courses {
    pub fn new(api: Api) -> Self {
        Self { db: HashMap::new(), api, names: HashMap::new() }
    }

    pub async fn load_summary(&self) -> Result<HashMap<u32, String>> {
        let data = self.api.courses().await?.to_value_vec();
        let mut hs = HashMap::new();
        data.into_iter().for_each(|course| {
            if course["id"].is_null() || course["name"].is_null() {
                return;
            }
            let id = course["id"].to_u32();
            let name = course["name"].to_str().to_string();
            hs.insert(id, name);
        });
        Ok(hs)
    }

    pub async fn load_all_courses(&self) -> Result<Vec<Course>> {
        let courses = self.load_summary().await?;
        let handles = courses
            .into_iter()
            // .filter(|v| v.0 == 24857)
            .map(|(id, name)| async move { self.load_course(id, name).await });
        let course_data: Vec<Result<_>> =
            futures::stream::iter(handles).buffer_unordered(10).collect().await;
        course_data.into_iter().collect()
    }

    pub async fn load_course(
        &self,
        course_id: u32,
        course_name: String,
    ) -> Result<Course> {
        if self.db.contains_key(&course_id) {
            return Err(Error::of("Course already downloaded."));
        };

        let mut course = Course {
            id: course_id,
            name: course_name,
            files: vec![],
            folders: vec![],
        };

        let folders = self.api.course_folders(course_id).map(|folders| {
            let folders = match folders {
                Err(e) => return Err(e),
                Ok(v) => v.to_value_vec(),
            };
            Ok(folders
                .into_iter()
                .map(|folder| {
                    let path = folder["full_name"].to_str().to_string();
                    let id = folder["id"].to_u32();
                    let name = folder["name"].to_str().to_string();
                    Folder { id, name, path, files: vec![], folders: vec![] }
                })
                .collect::<Vec<Folder>>())
        });

        let files = self.api.course_files(course_id).map(|files| {
            let err = files.is_err();
            let files = match files {
                Err(_) => vec![],
                Ok(v) => v.to_value_vec(),
            };
            files.into_iter().filter(|_| !err).for_each(|file| {
                let f = File {
                    id: file["id"].to_u32(),
                    name: file["filename"].to_str().to_string(),
                };
                course.files.push(f);
            });
        });

        let mut folders = folders.await?;
        files.await;
        folders.sort_by(|a, b| a.path.len().cmp(&b.path.len()));
        let mut c_folders = &mut course.folders;
        folders.into_iter().for_each(|folder| {
            c_folders.insert_by_path("", folder);
        });
        Ok(course)
    }
}

pub async fn main() -> Result<()> {
    let config = Config::load::<&str>(None, true)?;
    let api = Api::new(config.access_token());
    let courses = Courses::new(api);
    let db = courses.load_all_courses().await?;
    println!("courses -> {:?}", db);

    Ok(())
}
