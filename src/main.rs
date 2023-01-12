mod api;
mod config;
mod error;
mod types;
use api::Api;
use error::Error;

use std::env;
use std::path::PathBuf;
use types::Folder;

#[tokio::main]
async fn main() {
    let first_arg = env::args().skip(1).next();
    let result = try_main(first_arg.map(PathBuf::from)).await;
    match result {
        Err(e) => eprintln!("{e}"),
        _ => {}
    }
}

async fn try_main(config_path: Option<PathBuf>) -> Result<(), Error> {
    let config = config::get_config(config_path)?;
    let mut api = Api::new(&config)?;
    let mut downloaded = false;
    for task in config.maps() {
        let local_path = match task.local_path() {
            None => continue,
            Some(v) => v,
        };
        let (id, folder) = match (task.course_id(), task.folder_name()) {
            (Some(id), Some(folder)) => (id, folder),
            _ => continue,
        };
        let folders = api.course_folders(*id).await?;
        // TODO: handle this error
        let found = Folder::find(&folders, folder).unwrap();
        // TODO: handle this error
        let files = api.get_files(found).await.unwrap();
        for file in files {
            let target = local_path.join(file.filename());
            if !target.is_file() {
                // TODO: handle this error
                api.download(file.download_url(), &target).await.unwrap();
                println!("downloaded: {:?}", target);
                downloaded = true;
            }
        }
    }
    if !downloaded {
        println!("No new files found. All up to date!")
    }
    Ok(())
}
