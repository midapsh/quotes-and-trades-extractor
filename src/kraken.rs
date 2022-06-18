mod configs;
mod commands;
mod data_extractors;
mod processes;

use processes::kraken_process::kraken_process;

#[tokio::main]
async fn main() {
    kraken_process().await;
}
