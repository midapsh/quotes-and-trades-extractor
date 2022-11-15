mod commands;
mod configs;
mod custom_parsers;
mod data_extractors;
mod models;
mod workers;
mod worker_manager;
mod utils;


use crate::configs::configuration::get_configuration;
use worker_manager::WorkerManager;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    const VENUE: &str = "BITMEX";
    let settings = get_configuration().unwrap();

    let wm = WorkerManager::new(15, VENUE);

    Ok(wm.execute().await);

}
