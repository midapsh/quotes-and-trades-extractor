mod commands;
mod configs;
mod custom_parsers;
mod data_extractors;
mod processes;
mod models;


use processes::kraken_process::kraken_process;

#[tokio::main]
async fn main() {
    kraken_process().await;
}
