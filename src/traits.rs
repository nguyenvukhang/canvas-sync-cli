use serde_json::Value;
use std::path::{Path, PathBuf};

pub trait EasyJson {
    /// String slice or blank string slice.
    fn to_str(&self) -> &str;
    /// u32 or zero.
    fn to_u32(&self) -> u32;
    // Vec or empty vec.
    fn to_value_vec(&self) -> Vec<Value>;
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
