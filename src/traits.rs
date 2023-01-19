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
    // if tracked_remote_path is blank, then the tracking begins from
    // the root folder.
    fn to_remote_folder(
        &self,
        tracked_remote_path: &str,
    ) -> Option<(u32, String)>;
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
        tracked_remote_path: &str,
    ) -> Option<(u32, String)> {
        if self["id"].is_null() || self["full_name"].is_null() {
            return None;
        }
        let folder_id = self["id"].as_u64()? as u32;
        let t = tracked_remote_path;

        // expected value of full_path:
        // `course files/path/of/actual/folder`
        let full_path = self["full_name"].as_str()?;
        let remote_path = full_path.strip_prefix("course files/")?;

        if remote_path.eq(t) || tracked_remote_path.is_empty() {
            let remote_path = remote_path.to_string();
            return Some((folder_id, remote_path));
        }

        if remote_path.starts_with(t) && remote_path[t.len()..].starts_with('/')
        {
            let remote_path = remote_path[t.len() + 1..].to_string();
            return Some((folder_id, remote_path));
        }

        None
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
