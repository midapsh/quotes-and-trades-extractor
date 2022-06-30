mod commands;
mod configs;
mod custom_parsers;
mod data_extractors;
mod processes;
mod models;

use processes::ftx_process::ftx_process;

#[tokio::main]
async fn main() {
    ftx_process().await;
}
