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

    #[arg(default_value = "./canvas.json")]
    config_file: String,
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
        let config_file = PathBuf::from(&self.args.config_file);
        let mut config = Config::new(config_file)?;
        let mut api = Api::new(&config)?;

        let user_courses = api.list_courses().await?;
        config.load_course_names(&user_courses)?;
        let mut updates: Vec<Result<Update, Error>> = vec![];

        let mut prev_name = "";
        let check = "✓".green();
        let plus = "+".green();

        for task in config.maps() {
            let course_name = task
                .course_name()
                .ok_or(Error::CourseHasNoName(*task.course_id()))?;
            if !course_name.eq(prev_name) {
                println!("{}", course_name);
                prev_name = course_name;
            }
            let folders = api.course_folders(task.course_id()).await?;
            // TODO: handle this error
            let folder = Folder::find(&folders, task.folder_name()).unwrap();
            println!("  {check} {}", folder.full_name());

            // TODO: handle this error
            let files = api.get_files(folder).await.unwrap();

            for file in files {
                let target = task.local_path().join(file.filename());
                if target.is_file() {
                    continue;
                }
                let res = match self.args.dry_run {
                    true => Ok(()),
                    _ => api.download(file.download_url(), &target).await,
                };

                updates.push(
                    res.map(|_| Update {
                        course: course_name.to_string(),
                        full_name: file.full_name().to_path_buf(),
                    })
                );
            }
        }

        if self.args.dry_run {
            eprintln!("\n--- DRY RUN (nothing downloaded) ---")
        }

        if updates.is_empty() {
            println!("\nNo new files found. All up to date!");
            return Ok(());
        }

        println!("\nUpdate summary:");
        let mut prev_name = String::new();
        for update in updates {
            match update {
                Ok(Update { course, full_name }) => {
                    if !course.eq(&prev_name) {
                        println!("{course}");
                        prev_name = course;
                    }
                    println!(
                        "  {plus} {}",
                        full_name.into_os_string().to_string_lossy()
                    );
                }
                Err(e) => eprintln!("{e}"),
            };
        }

        Ok(())
    }
}
