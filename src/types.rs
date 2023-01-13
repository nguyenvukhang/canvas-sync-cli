use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
    id: u32,
    name: String,
    course_code: String,
}

impl Course {
    pub fn get(json: &serde_json::Value) -> Vec<Course> {
        let mut i = 0;
        let mut res = vec![];
        while let Some(j) = json.get(i) {
            i += 1;
            if j["uuid"].is_null() {
                continue;
            }
            res.push(Course {
                id: j["id"].as_u64().unwrap_or(0) as u32,
                name: json_string(&j["name"]),
                course_code: json_string(&j["course_code"]),
            })
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Folder {
    id: u32,
    full_name: String,
    name: String,
    files_url: String,
}

impl Folder {
    pub fn get(json: &serde_json::Value) -> Vec<Folder> {
        let mut i = 0;
        let mut res = vec![];
        while let Some(j) = json.get(i) {
            i += 1;
            if j["id"].is_null() {
                continue;
            }
            res.push(Folder {
                id: j["id"].as_u64().unwrap_or(0) as u32,
                name: j["name"].to_string(),
                full_name: json_string(&j["full_name"]),
                files_url: json_string(&j["files_url"]),
            })
        }
        res
    }

    pub fn find<'a>(haystack: &'a Vec<Self>, folder: &str) -> Option<&'a Self> {
        haystack.iter().find(|v| {
            v.full_name
                // canvas just pre-pends everything with "course files/"
                .strip_prefix("course files/")
                .map_or(false, |v| v.eq(folder))
        })
    }

    pub fn files_url(&self) -> &str {
        &self.files_url
    }
}

fn json_string(json: &serde_json::Value) -> String {
    json.as_str().unwrap_or("").to_string()
}

// TODO: handle version updates
// this happens when lecturers update a file but keeps the same filename
#[derive(Serialize, Deserialize, Debug)]
pub struct CanvasFile {
    id: u32,
    filename: String,
    download_url: String,
}

impl CanvasFile {
    pub fn get(json: &serde_json::Value) -> Vec<Self> {
        let mut i = 0;
        let mut res = vec![];
        while let Some(j) = json.get(i) {
            i += 1;
            if j["uuid"].is_null() {
                continue;
            }
            res.push(Self {
                id: j["id"].as_u64().unwrap_or(0) as u32,
                filename: json_string(&j["filename"]),
                download_url: json_string(&j["url"]),
            })
        }
        res
    }

    pub fn download_url(&self) -> &str {
        &self.download_url
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }
}
