mod api;
mod config;
mod error;
mod string;
mod sync;
mod traits;
mod types;

use api::Api;
use config::Config;
use error::{Error, Result};
use sync::Sync;
use types::User;

use clap::{Parser, Subcommand};

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
            C::Pull => self.sync(true).await,
            C::Fetch => self.sync(false).await,
        }
    }

    /// Runs a full sync on every folder listed. Only downloads
    /// files of `download` is set to true.
    async fn sync(&self, download: bool) -> Result<()> {
        let cfg_path = self.args.config_path.as_ref();
        let config = Config::load(cfg_path, true)?;
        let api = Api::new(config.access_token());
        let syncers = config
            .folder_maps()
            .iter()
            .map(|fm| Sync::new(&api, fm, download))
            .collect::<Result<Vec<_>>>()?;
        let handles =
            syncers.into_iter().map(|v| v.get_updates()).collect::<Vec<_>>();
        println!("Syncing {} folders...", handles.len());

        // sync at most 5 folders at a time.
        let loloupdates: Vec<Result<Vec<_>>> = api::resolve(handles, 5).await;
        let updates = untangle(loloupdates)?;
        Sync::run(&api, updates, download).await?;
        Ok(())
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

fn untangle<T>(tasks: Vec<Result<Vec<T>>>) -> Result<Vec<T>> {
    let tasks: Result<Vec<Vec<T>>> = tasks.into_iter().collect();
    Ok(tasks?.into_iter().flatten().collect())
}
