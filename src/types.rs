use crate::error::{Error, Result};
use crate::string::parse_url;
use crate::traits::*;
use crate::BINARY_NAME;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Thread-safe queue
pub struct Tsq<T> {
    db: Arc<Mutex<Vec<T>>>,
}

impl<T> Tsq<T> {
    pub fn new() -> Self {
        Self { db: Arc::new(Mutex::new(vec![])) }
    }

    pub fn push(&self, e: T) {
        let mut db = self.db.lock().unwrap();
        db.push(e);
    }

    pub fn get_ref(&self) -> Self {
        Self { db: Arc::clone(&self.db) }
    }

    pub fn len(&self) -> usize {
        self.db.lock().unwrap().len()
    }

    pub fn extract(self) -> Result<Vec<T>> {
        let arc = Arc::try_unwrap(self.db)
            .map_err(|_| Error::of("Can't unwrap Arc"))?;
        let db =
            arc.into_inner().map_err(|_| Error::of("Can't unwrap Mutex"))?;
        Ok(db)
    }
}

#[derive(Debug)]
pub struct Update {
    pub course_id: u32,
    pub remote_path: PathBuf,
}

/// Serializable folder map
#[derive(Serialize, Deserialize, Debug)]
pub struct FolderMap {
    /// url of the user-facing folder page.
    url: String,
    /// local dir to track the folder that the url points to.
    path: String,
    /// base path (taken from the config)
    base: Option<String>,
}

impl FolderMap {
    /// parse course_id from folder map's url
    pub fn course_id(&self) -> Result<u32> {
        parse_url(&self.url).map(|v| v.0)
    }

    /// parse remote directory from folder map's url
    pub fn remote_dir(&self) -> Result<String> {
        parse_url(&self.url).map(|v| v.1)
    }

    /// get local directory that tracks the url folder.
    pub fn local_dir(&self) -> PathBuf {
        let path = match &self.base {
            Some(v) => Path::new(v).join(&self.path),
            None => PathBuf::from(&self.path),
        };
        path.resolve().unwrap_or(path)
    }

    /// check that local dir's parent exists to minimize creating new
    /// directories.
    pub fn parent_exists(&self) -> bool {
        match self.local_dir().parent() {
            None => false,
            Some(v) => v.is_dir(),
        }
    }

    /// only to be used when parsing the config file for the first time
    pub fn set(&mut self, base: Option<String>) {
        if let Ok(url) = urlencoding::decode(&self.url) {
            self.url = url.to_string()
        }
        self.base = base
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

/// Corresponds to one `Profile` over on canvas.
/// https://canvas.instructure.com/doc/api/users.html#Profile
#[derive(Debug, Deserialize)]
pub struct User {
    id: u32,
    name: String,
    integration_id: String,
    primary_email: String,
}

impl User {
    pub fn display(&self) {
        println!(
            "\
{BINARY_NAME}

user data
  * canvas id: {}
  * name:      {}
  * email:     {}
  * matric:    {}",
            self.id, self.name, self.primary_email, self.integration_id
        )
    }
}

impl From<serde_json::Value> for User {
    fn from(json: serde_json::Value) -> Self {
        Self {
            id: json["id"].to_u32(),
            integration_id: json["integration_id"].to_str().to_string(),
            primary_email: json["primary_email"].to_str().to_string(),
            name: json["name"].to_str().to_string(),
        }
    }
}
