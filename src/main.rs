mod api;
mod config;
mod error;
mod string;
mod traits;
mod types;

use api::Api;
use clap::{Parser, Subcommand};
use colored::Colorize;
use config::Config;
use error::{Error, Result};
use futures::Future;
use std::collections::HashMap;
use std::path::PathBuf;
use string::normalize_filename;
use traits::*;
use types::{FolderMap, User};

// const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub const BINARY_NAME: &str = "canvas-sync";

#[derive(Parser, Debug)]
#[command(about = "hello")]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Points to a .yml file
    config_path: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    SetToken { token: String },
    Config,
    Fetch,
    Pull,
}

#[derive(Debug)]
pub struct App {
    args: Args,
}

#[derive(Debug)]
struct Update {
    course_id: u32,
    remote_path: PathBuf,
}

fn download_folder(
    api: &Api,
    course_id: u32,
    remote_path: String,
    files: Vec<serde_json::Value>,
    local_dir: PathBuf,
    download: bool,
) -> Result<(Vec<impl Future<Output = Result<()>>>, Vec<Update>)> {
    let local_dir = local_dir.to_path_buf();
    if download {
        std::fs::create_dir_all(&local_dir)?;
    }
    let mut updates = vec![];
    let remote_path = PathBuf::from(remote_path);
    let downloads = files
        .into_iter()
        .filter_map(|f| {
            let filename = f["filename"].to_str();
            let filename = normalize_filename(filename);
            let local_path = local_dir.join(&filename);
            log::info!("{local_path:?}");
            match local_path.is_file() {
                false => {
                    updates.push(Update {
                        course_id,
                        remote_path: remote_path.join(&filename),
                    });
                    Some((local_path, f["url"].to_str().to_string()))
                }
                true => None,
            }
        })
        .filter(|_| download)
        .map(|(local_path, url)| api.clone().download(url, local_path));
    Ok((downloads.collect(), updates))
}

/// Runs a full sync on a folder
async fn sync_folder(
    api: &Api,
    fm: &FolderMap,
    download: bool,
) -> Result<Vec<Update>> {
    if !fm.parent_exists() {
        return Err(Error::DownloadNoParentDir(fm.local_dir()));
    }
    let course_id = fm.course_id()?;
    let remote_path = fm.remote_path()?;
    let local_dir = fm.local_dir();
    let course_folders = api.course_folders(course_id).await?;
    let mut folders = course_folders.to_value_vec();
    let with_trailing_slash = format!("{remote_path}/");
    folders.retain(|v| !v["id"].is_null() && !v["full_name"].is_null());
    let folders: Vec<(u32, String)> = folders
        .into_iter()
        .filter_map(|v| {
            let id = (&v["id"]).to_u32();
            let rp = v["full_name"].to_str();
            let rp = rp.strip_prefix("course files/")?;
            if rp.eq(&remote_path) {
                return Some((id, "".to_string()));
            }
            if rp.starts_with(&with_trailing_slash) {
                return Some((id, (&rp[remote_path.len() + 1..]).to_string()));
            }
            None
        })
        .collect();

    // Each list of folders should at least match the root folder
    if folders.is_empty() {
        return Err(Error::InvalidTrackingUrl(fm.url().to_string()));
    }

    let handles = folders
        .into_iter()
        .map(|(id, rp)| (local_dir.join(&rp), id, rp))
        .map(|(local_dir, folder_id, remote_path)| async move {
            let files = api.files(folder_id).await?.to_value_vec();
            download_folder(
                api,
                course_id,
                remote_path,
                files,
                local_dir,
                download,
            )
        });
    // fetch at most 5 `api.files()` in a row
    let a: Vec<Result<(Vec<_>, Vec<_>)>> = api::resolve(handles, 5).await;
    let a: Result<Vec<(Vec<_>, Vec<_>)>> = a.into_iter().collect();
    let (downloads, updates): (Vec<_>, Vec<_>) = a?.into_iter().unzip();
    let updates = updates.into_iter().flat_map(|v| v).collect::<Vec<_>>();
    let downloads = downloads.into_iter().flat_map(|v| v).collect::<Vec<_>>();
    // fetch at most 5 `api.download()` in a row
    let results = api::resolve(downloads, 5).await;
    let results: Result<()> = results.into_iter().collect();
    results.map(|_| updates)
}

impl App {
    pub fn new(args: Option<Args>) -> Self {
        let args = args.unwrap_or_else(Args::parse);
        log::info!("{args:?}");
        Self { args }
    }

    /// Main entry point
    pub async fn run(&self) -> Result<()> {
        let cfg_path = self.args.config_path.as_ref();
        let command = match &self.args.command {
            Some(v) => v,
            None => {
                let config = Config::load(cfg_path, true)?;
                let api = Api::new(config.access_token());
                let profile = api.profile().await?;
                let user = User::from(profile);
                user.display();
                return Ok(());
            }
        };
        use Commands as C;
        match command {
            C::SetToken { token } => {
                let mut config = Config::load(cfg_path, false)?;
                config.set_token(token);
                config.save()?;
                println!(
                    "\
New token set! Try running `{BINARY_NAME}` to verify it.
"
                );
                Ok(())
            }
            C::Config => {
                let config = Config::load(cfg_path, false)?;
                let path = config.path();
                if path.contains(' ') {
                    println!("\"{path}\"");
                } else {
                    println!("{path}");
                }
                Ok(())
            }
            C::Pull => self.update(true).await,
            C::Fetch => self.update(false).await,
        }
    }

    /// Runs a full update on every folder listed. Only downloads
    /// files of `download` is set to true.
    async fn update(&self, download: bool) -> Result<()> {
        let cfg_path = self.args.config_path.as_ref();
        let config = Config::load(cfg_path, true)?;
        let api = Api::new(config.access_token());
        let result = config
            .folder_maps()
            .iter()
            .map(|fm| sync_folder(&api, fm, download))
            .collect::<Vec<_>>();
        println!("Syncing {} folders...", result.len());
        // sync at most 5 folders at a time.
        let loloupdates = api::resolve(result, 5).await;
        let mut updates: Vec<Update> = loloupdates
            .into_iter()
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flat_map(|v| v)
            .collect();
        let user_courses = api.courses().await?.to_value_vec();
        let course_hash = user_courses
            .into_iter()
            .map(|j| {
                let course_id = j["id"].to_u32();
                let course_name = j["name"].to_str().to_string();
                (course_id, course_name)
            })
            .collect::<HashMap<_, _>>();
        updates.sort_by(|a, b| {
            course_hash.get(&a.course_id).cmp(&course_hash.get(&b.course_id))
        });
        display_updates(&updates, &course_hash);
        if !download && !updates.is_empty() {
            println!("{}", "Fetch only. Nothing downloaded.".yellow());
        }
        Ok(())
    }
}

fn display_updates(updates: &Vec<Update>, course_names: &HashMap<u32, String>) {
    if updates.is_empty() {
        println!("No new files found. All up to date!");
        return;
    }
    let mut prev_id = 0;
    for update in updates {
        if update.course_id != prev_id {
            prev_id = update.course_id;
            println!("{}", course_names.get(&update.course_id).unwrap());
        }
        println!("  + {}", update.remote_path.to_string_lossy());
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let app = App::new(None);
    let outcome = app.run().await;
    if let Err(err) = outcome {
        eprintln!("{err}")
    }
}
