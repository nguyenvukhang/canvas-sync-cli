mod api;
mod folder_map;
mod app;
mod config;
mod error;
mod types;

use app::App;

#[tokio::main]
async fn main() {
    env_logger::init();
    let app = App::new(None);
    let outcome = app.run().await;
    if let Err(err) = outcome {
        eprintln!("{err}")
    }
}
