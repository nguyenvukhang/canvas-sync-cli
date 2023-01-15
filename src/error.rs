use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    CanvasEnvNotFound,
    EmptyToken,
    CourseHasNoName(u32),
    CourseNotFound(u32),
    InvalidFilename(PathBuf),
    ReqwestErr(reqwest::Error),
    SerdeJsonErr(serde_json::Error),
    IoErr(std::io::Error),
    ConfigNotFound(PathBuf),
    DownloadNoParentDir(PathBuf),
    InvalidTrackingUrl(String),
    DownloadErr(String, reqwest::Error),
}

fn path_to_string<P: AsRef<Path>>(path: P) -> String {
    path.as_ref().to_str().unwrap_or("<Unable to parse filepath>").to_string()
}

const CANVAS_ENV_NOT_FOUND: &str = "Canvas token should be made available at the $CANVAS_TOKEN environment variable.";

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            DownloadErr(url, err) => {
                write!(f, "Failed to download from url {url}, {err}")
            }
            CourseHasNoName(id) => {
                write!(f, "Course should have a name (course_id: {id})")
            }
            CourseNotFound(id) => {
                write!(f, "No course found for course_id: {id}")
            }
            CanvasEnvNotFound => write!(f, "{CANVAS_ENV_NOT_FOUND}"),
            EmptyToken => write!(f, "No token provided"),
            InvalidTrackingUrl(v) => write!(f, "Invalid url: {v}"),
            DownloadNoParentDir(v) => {
                write!(
                    f,
                    "Download target `{}` has no parent.",
                    path_to_string(v)
                )
            }
            InvalidFilename(v) => {
                write!(f, "Invalid filename: `{}`", path_to_string(v))
            }
            ReqwestErr(v) => write!(f, "{v}"),
            IoErr(v) => write!(f, "{v}"),
            SerdeJsonErr(v) => write!(f, "{v}"),
            ConfigNotFound(v) => {
                write!(f, "Config not found at `{}`", path_to_string(v))
            }
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestErr(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeJsonErr(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoErr(error)
    }
}
