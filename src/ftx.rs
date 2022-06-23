mod commands;
mod configs;
mod custom_deserializers;
mod custom_parsers;
mod data_extractors;
mod processes;

use processes::ftx_process::ftx_process;

#[tokio::main]
async fn main() {
    ftx_process().await;
}
