mod api;
mod config;
mod error;
mod string;
mod traits;
mod types;

use api::Api;
use clap::{Parser, Subcommand};
use config::Config;
use error::{Error, Result};
use futures::Future;
use traits::*;
use types::{FolderMap, Update, User};

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

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
    SetToken {
        token: String,
    },
    Config {
        #[arg(short, long)]
        edit: bool,
    },
    Fetch,
    Pull,
}

#[derive(Debug)]
pub struct App {
    args: Args,
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
    let tracked_remote_dir = fm.remote_dir()?;

    let folders: Vec<(u32, String)> = api
        .course_folders(course_id)
        .await?
        .as_array()
        .map(|f| {
            f.iter()
                .filter_map(|v| v.to_remote_folder(&tracked_remote_dir))
                .collect()
        })
        .unwrap_or_default();

    // Each list of folders should at least match the root folder
    if folders.is_empty() {
        let url = fm.url().to_string();
        return Err(Error::NoFoldersFoundInCourse { url });
    }

    let local_dir = fm.local_dir();
    let handles = folders
        .into_iter()
        .map(|(id, rd)| (local_dir.join(&rd), id, PathBuf::from(rd)))
        .map(|(local_dir, folder_id, remote_dir)| async move {
            let files = api.files(folder_id).await?.to_value_vec();
            if download {
                std::fs::create_dir_all(&local_dir)?;
            }
            let files = files
                .into_iter()
                .filter_map(|f| {
                    let url = f["url"].as_str()?.to_string();
                    let filename = f.to_normalized_filename()?;
                    Some((url, filename))
                })
                .collect::<Vec<_>>();

            let updates = files
                .iter()
                .map(|(_, filename)| Update {
                    course_id,
                    remote_path: remote_dir.join(&filename),
                })
                .collect();

            if !download {
                return Ok((vec![], updates));
            }

            let downloads = files
                .into_iter()
                .filter_map(|(url, filename)| {
                    let local_path = local_dir.join(&filename);
                    (!local_path.is_file())
                        .then(|| api.clone().download(url, local_path))
                })
                .collect();

            Ok((downloads, updates))
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
            C::Config { edit: false } => {
                println!("{}", Config::path()?.to_string_lossy());
                Ok(())
            }
            C::Config { edit: true } => {
                let editor = std::env::var("EDITOR").map_err(|_| {
                    Error::of("Unable to get an editor from $EDITOR.")
                })?;
                Command::new(editor).arg(Config::path()?).spawn()?.wait()?;
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
            println!("! Fetch only. Nothing downloaded.");
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
            match course_names.get(&update.course_id) {
                Some(v) => println!("{v}"),
                None => println!(
                    "Error: failed to fetch course with id {}",
                    update.course_id
                ),
            }
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
