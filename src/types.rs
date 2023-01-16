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
    #[allow(unused)]
    id: u32,
    full_name: String,
    files_url: String,
}

impl Folder {
    pub fn get_vec(json: &Value) -> Vec<Folder> {
        let required_keys = ["id", "full_name"];
        json.to_vec(&required_keys, |j| {
            let full_name = j["full_name"]
                .as_str()
                // canvas just pre-pends everything with "course files/"
                .map(|v| v.strip_prefix("course files/").unwrap_or(v))
                .unwrap_or("")
                .to_string();
            Folder {
                id: j["id"].as_u64().unwrap_or(0) as u32,
                files_url: json_string(&j["files_url"]),
                full_name,
            }
        })
    }

    pub fn find<'a>(haystack: &'a Vec<Self>, folder: &str) -> Option<&'a Self> {
        haystack.iter().find(|v| v.full_name.eq(folder))
    }

    pub fn files_url(&self) -> &str {
        &self.files_url
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }
}

// TODO: handle version updates
// this happens when lecturers update a file but keeps the same filename
#[derive(Debug)]
pub struct CanvasFile {
    #[allow(unused)]
    id: u32,
    filename: String,
    full_name: PathBuf,
    download_url: String,
}

impl CanvasFile {
    pub fn get_vec(json: &Value, parent_folder: &str) -> Vec<Self> {
        let required_keys = ["uuid"];
        let parent = PathBuf::from(parent_folder);
        json.to_vec(&required_keys, |j| {
            let filename = json_string(&j["filename"]).replace("+", "_");
            Self {
                id: j["id"].as_u64().unwrap_or(0) as u32,
                full_name: parent.join(&filename),
                filename,
                download_url: json_string(&j["url"]),
            }
        })
    }

    pub fn download_url(&self) -> &str {
        &self.download_url
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn full_name(&self) -> &PathBuf {
        &self.full_name
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
