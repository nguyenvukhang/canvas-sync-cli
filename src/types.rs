use serde_json::Value;
use std::path::PathBuf;

/// Custom parser for an array of values stored within a
/// `serde_json::Value`
trait ToVec {
    /// If any of the `required_keys` are not present in the array
    /// element, skip that element. This allows for parsing of
    /// partial/incomplete json data.
    fn to_vec<T, F>(&self, required_keys: &[&str], f: F) -> Vec<T>
    where
        F: Fn(&Value) -> T;
}

impl ToVec for Value {
    fn to_vec<T, F>(&self, required_keys: &[&str], f: F) -> Vec<T>
    where
        F: Fn(&Value) -> T,
    {
        let array = match self.as_array() {
            Some(v) => v,
            None => return vec![],
        };
        array
            .into_iter()
            .filter(|v| required_keys.iter().all(|k| !v[k].is_null()))
            .map(f)
            .collect()
    }
}

/// Get a string without its quotes.
fn json_string(json: &Value) -> String {
    json.as_str().unwrap_or("").to_string()
}

#[derive(Debug, Clone)]
pub struct Folder {
    course_id: u32,
    remote_path: String,
    files_url: String,
}

impl Folder {
    pub fn get_vec(json: &Value, course_id: &u32) -> Vec<Folder> {
        let required_keys = ["id", "full_name"];
        json.to_vec(&required_keys, |j| {
            let remote_path = j["full_name"]
                .as_str()
                // canvas just pre-pends everything with "course files/"
                .map(|v| v.strip_prefix("course files/").unwrap_or(v))
                .unwrap_or("")
                .to_string();
            Folder {
                course_id: *course_id,
                files_url: json_string(&j["files_url"]),
                remote_path,
            }
        })
    }

    pub fn files_url(&self) -> &str {
        &self.files_url
    }

    pub fn remote_path(&self) -> &str {
        &self.remote_path
    }

    pub fn course_id(&self) -> &u32 {
        &self.course_id
    }
}

// TODO: handle version updates
// this happens when lecturers update a file but keeps the same filename
#[derive(Debug, Clone)]
pub struct FileMap {
    /// Location to send the download to.
    local_target: PathBuf,
    download_url: String,
}

impl FileMap {
    pub fn get_vec(json: &Value, local_dir: &PathBuf) -> Vec<Self> {
        let required_keys = ["uuid"];
        json.to_vec(&required_keys, |j| {
            let filename = json_string(&j["filename"]).replace("+", "_");
            Self {
                local_target: local_dir.join(&filename),
                download_url: json_string(&j["url"]),
            }
        })
    }

    pub fn download_url(&self) -> &str {
        &self.download_url
    }

    pub fn local_target(&self) -> &PathBuf {
        &self.local_target
    }
}

#[derive(Debug)]
pub struct Course {
    id: u32,
    name: String,
    // course_code: String,
}

impl Course {
    pub fn get_vec(json: &Value) -> Vec<Course> {
        let required_keys = ["uuid"];
        json.to_vec(&required_keys, |j| Course {
            id: j["id"].as_u64().unwrap_or(0) as u32,
            name: json_string(&j["name"]),
            // course_code: json_string(&j["course_code"]),
        })
    }

    pub fn id(&self) -> &u32 {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
pub struct User {
    id: u32,
    name: String,
}

impl User {
    pub fn get(json: &Value) -> Option<Self> {
        if json["id"].is_null() || json["name"].is_null() {
            return None;
        }
        Some(Self {
            id: json["id"].as_u64().unwrap_or(0) as u32,
            name: json_string(&json["name"]),
        })
    }

    pub fn id(&self) -> &u32 {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
