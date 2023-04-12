use crate::error::{Error, Result};

/// Parses a url in a url-path config pair to extract course id and
/// full folder name.
///
/// Example input:
/// https://canvas.nus.edu.sg/courses/38518/files/folder/Lectures/Java%20Intro
///
/// Expected output:
/// (38518, "Lectures/Java Intro")
pub fn parse_url(mut url: &str) -> Result<(u32, String)> {
    let err = || Error::InvalidTrackingUrl(url.to_string());
    if url.starts_with("https://") {
        url = &url[8..];
    }
    if url.starts_with("canvas.nus.edu.sg/courses/") {
        url = &url[26..];
    }
    let (id, mut folder) = url.split_once("/").ok_or(err())?;
    let id = id.parse::<u32>().map_err(|_| err())?;
    if folder.starts_with("files") {
        folder = &folder[5..];
    }
    if folder.starts_with("/folder") {
        folder = &folder[7..];
    }
    if folder.starts_with("/") {
        folder = &folder[1..];
    }
    if let Ok(decoded) = urlencoding::decode(folder) {
        return Ok((id, decoded.to_string()));
    }
    Ok((id, folder.to_string()))
}

#[test]
fn test_parse_url() -> Result<()> {
    let (id, path) =
        parse_url("https://canvas.nus.edu.sg/courses/36732/files")?;
    assert_eq!(id, 36732);
    assert_eq!(path, "");
    let (id, path) = parse_url(
        "https://canvas.nus.edu.sg/courses/36732/files/folder/Lecture%20Notes",
    )?;
    assert_eq!(id, 36732);
    assert_eq!(path, "Lecture Notes");
    Ok(())
}

/// Normalize filename by replacing '+' and '-' with '_', and then
/// replacing all "__" with '_'
pub fn normalize_filename(v: &str) -> String {
    let mut v = v.replace("+", "_").replace("-", "_");
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
