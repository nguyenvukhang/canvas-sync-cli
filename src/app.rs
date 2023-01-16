use crate::api::Api;
use crate::config::Config;
use crate::error::Error;
use crate::types::Folder;
use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
    #[arg(long)]
    dry_run: bool,

    /// Points to a .yml file
    config_path: Option<String>,
}

#[derive(Debug)]
pub struct App {
    args: Args,
}

struct Update {
    course: String,
    // canvas file's full name (includes path)
    full_name: PathBuf,
}

impl App {
    pub fn new(args: Option<Args>) -> Self {
        let args = args.unwrap_or_else(Args::parse);
        log::info!("{args:?}");
        Self { args }
    }

    pub async fn run(&self) -> Result<(), Error> {
        let mut config = Config::load(self.args.config_path.as_ref())?;
        config.load_course_names().await?;
        config.fetch_all_folders().await?;
        config.fetch_all_files().await?;

        config.run(true).await;

        // let mut prev_name = "";
        // let check = "✓".green();
        // let plus = "+".green();

        // if self.args.dry_run {
        //     eprintln!("\n--- DRY RUN (nothing downloaded) ---")
        // }
        //
        // if updates.is_empty() {
        //     println!("\nNo new files found. All up to date!");
        //     return Ok(());
        // }
        //
        // println!("\nUpdate summary:");
        // let mut prev_name = String::new();
        // for update in updates {
        //     match update {
        //         Ok(Update { course, full_name }) => {
        //             if !course.eq(&prev_name) {
        //                 println!("{course}");
        //                 prev_name = course;
        //             }
        //             println!(
        //                 "  {plus} {}",
        //                 full_name.into_os_string().to_string_lossy()
        //             );
        //         }
        //         Err(e) => eprintln!("{e}"),
        //     };
        // }

        Ok(())
    }
}
