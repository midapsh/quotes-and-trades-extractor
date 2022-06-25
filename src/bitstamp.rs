mod commands;
mod configs;
mod custom_parsers;
mod data_extractors;
mod processes;

use processes::bitstamp_process::bitstamp_process;

#[tokio::main]
async fn main() {
    bitstamp_process().await;
}
