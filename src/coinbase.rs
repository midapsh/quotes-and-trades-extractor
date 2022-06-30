mod commands;
mod configs;
mod custom_parsers;
mod data_extractors;
mod processes;
mod models;

use processes::coinbase_process::coinbase_process;

#[tokio::main]
async fn main() {
    coinbase_process().await;
}
