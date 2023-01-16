use crate::error::Error;
use std::path::PathBuf;

/// Parses a url in a url-path config pair to extract course id and
/// full folder name.
///
/// Example input:
/// https://canvas.nus.edu.sg/courses/38518/files/folder/Lectures/Java%20Intro
///
/// Expected output:
/// (38518, "Lectures/Java Intro")
pub fn parse_url(url: &str) -> Result<(u32, String), Error> {
    let err = || Error::InvalidTrackingUrl(url.to_string());
    let url =
        url.strip_prefix("https://canvas.nus.edu.sg/courses/").ok_or(err())?;
    let (id, folder) = url.split_once("/").ok_or(err())?;
    let id = id.parse::<u32>().map_err(|_| err())?;
    let folder = folder.strip_prefix("files/folder/").ok_or(err())?;
    if let Ok(decoded) = urlencoding::decode(folder) {
        return Ok((id, decoded.to_string()));
    }
    Ok((id, folder.to_string()))
}

/// Replace tilde with home.
pub fn replace_tilde(v: &str) -> Option<PathBuf> {
    if let Some(homeless) = v.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return Some(home.join(homeless));
        }
    }
    None
}
