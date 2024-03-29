use crate::BINARY_NAME;
use std::fmt;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EmptyToken,
    InvalidToken,
    CanvasError { msg: String, url: String },
    UnableToGetConfigPath,
    Debug(String),
    DownloadNoParentDir(PathBuf),
    InvalidTrackingUrl(String),
    NoFoldersFoundInCourse { url: String },
    DownloadErr(String, reqwest::Error),

    // wrapped errors
    ReqwestErr(reqwest::Error),
    SerdeJsonErr(serde_json::Error),
    IoErr(std::io::Error),
    ConfyErr(confy::ConfyError),
}

fn token_instructions(pre: &str) -> String {
    format!(
        "\
{pre}

To obtain a token, go to your profile settings at
https://canvas.nus.edu.sg/profile/settings
and create a new access token.

Run `{BINARY_NAME} set-token <token>` to set the token,
and then try to run `{BINARY_NAME}` again.
"
    )
}

fn display(err: &Error, f: &mut fmt::Formatter) -> fmt::Result {
    macro_rules! p { ($($arg:tt)*) => { write!(f, $($arg)*) }; }
    use Error::*;
    match err {
        Debug(msg) => p!("{msg}"),
        CanvasError { msg, url } => p!("Canvas error: {msg}\nurl: {url}"),
        EmptyToken => p!("{}", token_instructions("No token provided.")),
        InvalidToken => {
            p!("{}", token_instructions("Invalid access token."))
        }
        UnableToGetConfigPath => p!("Unable to get config path."),
        DownloadErr(url, err) => {
            p!("Failed to download from url {url}, {err}")
        }
        InvalidTrackingUrl(v) => p!("Invalid url: {v}"),
        NoFoldersFoundInCourse { url } => {
            p!("No folders found in course: {url}")
        }
        DownloadNoParentDir(v) => {
            write!(
                f,
                "Bad download target: `{}` (directory does not exist).",
                v.to_string_lossy()
            )
        }
        // wrapped errors
        ReqwestErr(v) => p!("{v}"),
        IoErr(v) => p!("{v}"),
        SerdeJsonErr(v) => p!("{v}"),
        ConfyErr(v) => p!("{v}"),
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display(self, f)
    }
}

impl Error {
    pub fn of(msg: &str) -> Self {
        Error::Debug(msg.to_string())
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
