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
use futures::executor::ThreadPool;
use futures::Future;
use traits::*;
use types::{Download, FolderMap, Tsq, Update, User};

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Condvar, Mutex};

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

struct Sync<'a> {
    api: &'a Api,
    fm: &'a FolderMap,
    course_id: u32,
    remote_dir: String,
    download: bool,
}

impl<'a> Sync<'a> {
    fn new(api: &'a Api, fm: &'a FolderMap, download: bool) -> Result<Self> {
        if !fm.parent_exists() {
            return Err(Error::DownloadNoParentDir(fm.local_dir()));
        }
        let course_id = fm.course_id()?;
        let remote_dir = fm.remote_dir()?;
        Ok(Self { api, fm, download, course_id, remote_dir })
    }

    /// Resolve all async tasks embedded in `tasks`.
    /// `tasks` is a list of `Result` enums, each containing a list of
    /// downloads and a list of updates.
    ///
    /// This function will wait for all downloads to finish, while
    /// joining all updates into one list.
    async fn parallel(
        tasks: Vec<
            Result<(Vec<impl Future<Output = Result<()>>>, Vec<Update>)>,
        >,
    ) -> Result<Vec<Update>> {
        let a: Result<Vec<(Vec<_>, Vec<_>)>> = tasks.into_iter().collect();
        let (downloads, updates): (Vec<_>, Vec<_>) = a?.into_iter().unzip();
        let updates = updates.into_iter().flatten().collect::<Vec<_>>();
        let downloads = downloads.into_iter().flatten();
        // fetch at most 5 `api.download()` in a row
        let downloads = api::resolve(downloads, 5).await;
        downloads.into_iter().collect::<Result<Vec<()>>>()?;
        Ok(updates)
    }

    /// Terminal function of the `Sync` struct. Returns a list of all
    /// downloadables (url -> local path) pairs, and a list of all
    /// updates found.
    async fn run(self) -> Result<(Vec<Download>, Vec<Update>)> {
        let folders = self.api.course_folders(self.course_id).await?;
        let folders: Vec<(u32, String)> = folders
            .as_array()
            .ok_or(Error::NoFoldersFoundInCourse {
                url: self.fm.url().to_string(),
            })?
            .into_iter()
            .filter_map(|v| v.to_remote_folder(&self.remote_dir))
            .collect();

        if folders.is_empty() {
            let url = self.fm.url().to_string();
            return Err(Error::NoFoldersFoundInCourse { url });
        }

        let local_dir = self.fm.local_dir();

        // List of pairs. Left one is list of url-filename pairs
        // Right one is remote directory on canvas.
        let loaded_files = {
            let t = folders.iter().map(|(folder_id, remote_dir)| async move {
                let files = self.api.files(*folder_id).await?;
                let files = files
                    .as_array()
                    .ok_or(Error::NoFoldersFoundInCourse {
                        url: self.fm.url().to_string(),
                    })?
                    .into_iter()
                    .filter_map(|f| {
                        let url = f["url"].as_str()?.to_string();
                        let filename = f.to_normalized_filename()?;
                        Some((url, filename))
                    })
                    .collect::<Vec<_>>();
                Ok((files, PathBuf::from(remote_dir)))
            });
            let t = api::resolve(t, 5).await;
            let t: Result<Vec<(Vec<_>, _)>> = t.into_iter().collect();
            t
        }?;

        let tasks: Vec<Result<(Vec<_>, Vec<_>)>> = loaded_files
            .into_iter()
            .map(|(a, b)| (local_dir.join(&b), a, b))
            .map(|(local_dir, mut files, remote_dir)| {
                if self.download {
                    std::fs::create_dir_all(&local_dir)?;
                }

                files.retain(|f| !local_dir.join(&f.1).is_file());

                let updates = files
                    .iter()
                    .map(|(_, filename)| Update {
                        course_id: self.course_id,
                        remote_path: remote_dir.join(&filename),
                    })
                    .collect();

                if !self.download {
                    return Ok((vec![], updates));
                }

                let downloads = files
                    .into_iter()
                    .filter_map(|(url, filename)| {
                        let local_path = local_dir.join(&filename);
                        (!local_path.is_file())
                            .then(|| Download { url, local_path })
                    })
                    .collect();

                Ok((downloads, updates))
            })
            .collect();

        let tasks: Result<Vec<(Vec<_>, Vec<_>)>> = tasks.into_iter().collect();
        let (downloads, updates): (Vec<Vec<_>>, Vec<Vec<_>>) =
            tasks?.into_iter().unzip();
        let downloads = downloads.into_iter().flatten().collect();
        let updates = updates.into_iter().flatten().collect();
        Ok((downloads, updates))
    }
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

    async fn download_all(downloads: Vec<Download>, api: &Api) -> Result<()> {
        let handles = downloads
            .into_iter()
            .map(|d| api.clone().download(d.url, d.local_path));
        let results = api::resolve(handles, 5).await;
        let results: Result<Vec<_>> = results.into_iter().collect();
        results.map(|_| ())
    }

    /// Runs a full update on every folder listed. Only downloads
    /// files of `download` is set to true.
    async fn update(&self, download: bool) -> Result<()> {
        let cfg_path = self.args.config_path.as_ref();
        let config = Config::load(cfg_path, true)?;
        let api = Api::new(config.access_token());
        let syncers = config
            .folder_maps()
            .iter()
            .map(|fm| Sync::new(&api, fm, download))
            .collect::<Result<Vec<_>>>()?;
        let results = syncers.into_iter().map(|v| v.run()).collect::<Vec<_>>();
        println!("Syncing {} folders...", results.len());
        // sync at most 5 folders at a time.
        let loloupdates = api::resolve(results, 5).await;

        let (downloads, mut updates) = untangle(loloupdates)?;
        App::download_all(downloads, &api).await?;

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

fn untangle<A, B>(
    tasks: Vec<Result<(Vec<A>, Vec<B>)>>,
) -> Result<(Vec<A>, Vec<B>)> {
    let tasks: Result<Vec<(Vec<A>, Vec<B>)>> = tasks.into_iter().collect();
    let (a, b): (Vec<Vec<A>>, Vec<Vec<B>>) = tasks?.into_iter().unzip();
    let a = a.into_iter().flatten().collect();
    let b = b.into_iter().flatten().collect();
    Ok((a, b))
}
