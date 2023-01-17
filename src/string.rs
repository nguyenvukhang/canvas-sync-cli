use crate::error::{Error, Result};

/// Parses a url in a url-path config pair to extract course id and
/// full folder name.
///
/// Example input:
/// https://canvas.nus.edu.sg/courses/38518/files/folder/Lectures/Java%20Intro
///
/// Expected output:
/// (38518, "Lectures/Java Intro")
pub fn parse_url(url: &str) -> Result<(u32, String)> {
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

/// Normalize filename by replacing '+' and '-' with '_', and then
/// replacing all "__" with '_'
pub fn normalize_filename(v: &str) -> String {
    let mut v = v.replace("+", "_").replace(" ", "_").replace("-", "_");
    let mut len = v.len();
    loop {
        v = v.replace("__", "_");
        len = match v.len() {
            l if l == len => break,
            l => l,
        };
    }
    v
}
