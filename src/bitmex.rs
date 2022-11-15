mod commands;
mod configs;
mod custom_parsers;
mod data_extractors;
mod models;
mod workers;
mod utils;


use crate::utils::worker_manager::WorkerManager;
use crate::workers::bitmex_worker::BitmexWorker;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    const VENUE: &str = "BITMEX";

    let mut wm = WorkerManager::<BitmexWorker>::new(1, VENUE.to_string());

    Ok(wm.execute().await)
}
