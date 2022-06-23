mod commands;
mod configs;
mod custom_deserializers;
mod custom_parsers;
mod data_extractors;
mod processes;


use processes::kraken_futures_process::kraken_futures_process;

#[tokio::main]
async fn main() {
    kraken_futures_process().await;
}
