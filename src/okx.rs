mod commands;
mod configs;
mod custom_deserializers;
mod custom_parsers;
mod data_extractors;
mod processes;

use processes::okx_process::okx_process;

#[tokio::main]
async fn main() {
    okx_process().await;
}
