mod commands;
mod configs;
mod custom_parsers;
mod data_extractors;
mod processes;
mod models;

use processes::okx_process::okx_process;

#[tokio::main]
async fn main() {
    okx_process().await;
}
