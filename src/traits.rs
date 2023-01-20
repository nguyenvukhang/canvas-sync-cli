use crate::string::normalize_filename;
use serde_json::Value;
use std::path::{Path, PathBuf};

pub trait EasyJson {
    /// String slice or blank string slice.
    fn to_str(&self) -> &str;

    /// u32 or zero.
    fn to_u32(&self) -> u32;

    // Vec or empty vec.
    fn to_value_vec(&self) -> Vec<Value>;

    // Extract a folder id and a remote path from a JSON object.
    // https://canvas.instructure.com/doc/api/files.html#Folder
    //
    // if tracked_remote_dir is blank, then the tracking begins from
    // the root folder.
    fn to_remote_folder(
        &self,
        tracked_remote_dir: &str,
    ) -> Option<(u32, String)>;

    fn to_normalized_filename(&self) -> Option<String>;
}

impl EasyJson for Value {
    fn to_str(&self) -> &str {
        self.as_str().unwrap_or("")
    }

    fn to_u32(&self) -> u32 {
        self.as_u64().unwrap_or(0) as u32
    }

    fn to_value_vec(&self) -> Vec<Value> {
        match self.as_array() {
            None => vec![],
            Some(v) => v.into_iter().map(|v| v.to_owned()).collect(),
        }
    }

    fn to_remote_folder(
        &self,
        tracked_remote_dir: &str,
    ) -> Option<(u32, String)> {
        if self["id"].is_null() || self["full_name"].is_null() {
            return None;
        }
        let folder_id = self["id"].as_u64()? as u32;
        let t = tracked_remote_dir;

        // expected value of full_path:
        // `course files/path/of/actual/folder`
        let full_path = self["full_name"].as_str()?;
        let remote_dir = full_path.strip_prefix("course files/")?;

        if tracked_remote_dir.is_empty() {
            return Some((folder_id, remote_dir.to_string()));
        }

        if remote_dir.eq(t) {
            return Some((folder_id, "".to_string()));
        }

        if remote_dir.starts_with(t) && remote_dir[t.len()..].starts_with('/') {
            let remote_dir = remote_dir[t.len() + 1..].to_string();
            return Some((folder_id, remote_dir));
        }

        None
    }

    fn to_normalized_filename(&self) -> Option<String> {
        let filename = self["filename"].as_str()?;
        Some(normalize_filename(filename))
    }
}

pub trait ResolvePath {
    fn resolve(&self) -> Option<PathBuf>;
}

impl ResolvePath for PathBuf {
    fn resolve(&self) -> Option<PathBuf> {
        _resolve(self)
    }
}

impl ResolvePath for Path {
    fn resolve(&self) -> Option<PathBuf> {
        _resolve(self)
    }
}

fn _resolve<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let path = path.as_ref();
    if !path.starts_with("~") {
        return Some(path.to_path_buf());
    }
    if path == Path::new("~") {
        return dirs::home_dir();
    }
    dirs::home_dir().map(|mut home| {
        // unwrap safety guaranteed because logically
        // path already starts with "~/" at this point.
        if home == Path::new("/") {
            // Corner case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            path.strip_prefix("~").unwrap().to_path_buf()
        } else {
            home.push(path.strip_prefix("~/").unwrap());
            home
        }
    })
}
