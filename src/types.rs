use serde::Deserialize;
use serde_json::Value;
use std::path::PathBuf;

/// Corresponds to one `Foler` over on canvas.
/// https://canvas.instructure.com/doc/api/files.html#Folder
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

    /// For mapping each folder back to its home course.
    pub fn matches(&self, remote_path: &str, course_id: &u32) -> bool {
        self.remote_path.eq(remote_path) && self.course_id.eq(course_id)
    }
}

/// Corresponds to one `File` over on canvas.
/// https://canvas.instructure.com/doc/api/files.html#File
#[derive(Debug, Clone)]
pub struct FileMap {
    /// Url that, when followed, will return a byte stream that is the
    /// requested file.
    download_url: String,
    /// Location to send the download to.
    local_target: PathBuf,
}

impl FileMap {
    pub fn get_vec(json: &Value, local_dir: &PathBuf) -> Vec<Self> {
        let required_keys = ["uuid", "filename", "url"];
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
Canvas User Data
  * canvas id: {}
  * name:      {}
  * email:     {}
  * matric:    {}",
            self.id, self.name, self.primary_email, self.integration_id
        )
    }
}

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
