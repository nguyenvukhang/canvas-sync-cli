use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    CanvasEnvNotFound,
    UnableToGetUserData,
    EmptyToken,
    FilesUrlNotFound,
    FolderNotFound(String, String),
    CourseHasNoName(u32),
    CourseNotFound(u32, String),
    InvalidFilename(PathBuf),
    ConfigNotFound(PathBuf),
    DownloadNoParentDir(PathBuf),
    InvalidTrackingUrl(String),
    DownloadErr(String, reqwest::Error),

    // wrapped errors
    ReqwestErr(reqwest::Error),
    SerdeJsonErr(serde_json::Error),
    IoErr(std::io::Error),
    ConfyErr(confy::ConfyError),
}

fn path_to_string<P: AsRef<Path>>(path: P) -> String {
    path.as_ref().to_str().unwrap_or("<Unable to parse filepath>").to_string()
}

const CANVAS_ENV_NOT_FOUND: &str = "Canvas token should be made available at the $CANVAS_TOKEN environment variable.";

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            UnableToGetUserData => write!(f, "Unable to get basic user data. Check again if access token is present and valid."),
            FilesUrlNotFound => write!(f, "files_url not found"),
            FolderNotFound(course, folder_name) => {
                write!(
                    f,
                    "Folder not found. Course: {course}, folder: {folder_name}"
                )
            }
            DownloadErr(url, err) => {
                write!(f, "Failed to download from url {url}, {err}")
            }
            CourseHasNoName(id) => {
                write!(f, "Course should have a name (course_id: {id})")
            }
            CourseNotFound(id, url) => {
                write!(
                    f,
                    "No course found for course_id: {id}, from url `{url}`"
                )
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
            ConfigNotFound(v) => {
                write!(f, "Config not found at `{}`", path_to_string(v))
            }
            // wrapped errors
            ReqwestErr(v) => write!(f, "{v}"),
            IoErr(v) => write!(f, "{v}"),
            SerdeJsonErr(v) => write!(f, "{v}"),
            ConfyErr(v) => write!(f, "{v}"),
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

impl From<confy::ConfyError> for Error {
    fn from(error: confy::ConfyError) -> Self {
        Self::ConfyErr(error)
    }
}
