use crate::config::{Config, SConfig};
use crate::error::Error;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
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

impl App {
    pub fn new(args: Option<Args>) -> Self {
        let args = args.unwrap_or_else(Args::parse);
        log::info!("{args:?}");
        Self { args }
    }

    /// Returns true if there is a need to continue app execution
    /// after this method.
    async fn handle_command(&self, config: Config) -> Result<bool, Error> {
        use Commands::*;
        let command = match &self.args.command {
            Some(v) => v,
            None => {
                log::info!("mode::[no command]");
                config.hello().await?;
                return Ok(false);
            }
        };
        match command {
            SetToken { token } => {
                log::info!("mode::[set token]");
                let mut sconfig =
                    SConfig::load(self.args.config_path.as_ref())?;
                sconfig.set_token(token);
                sconfig.save()?;
                println!("Token updated!");
                Ok(false)
            }
            Config => {
                log::info!("mode::[config]");
                let config_path = config.get_path()?;
                println!("{}", config_path.to_string_lossy());
                return Ok(false);
            }
            _ => Ok(true),
        }
    }

    pub async fn run(&self) -> Result<(), Error> {
        let config = Config::load(self.args.config_path.as_ref())?;
        match self.handle_command(config).await {
            Ok(true) => {}
            Err(e) => return Err(e),
            _ => return Ok(()),
        }
        let download = match self.args.command {
            Some(Commands::Pull) => true,
            _ => false,
        };
        let mut config = Config::load(self.args.config_path.as_ref())?;
        config.load_course_names().await?;
        config.fetch_all_folders().await?;
        config.fetch_all_files().await?;
        config.run(download).await
    }
}
