mod configs;
mod commands;
mod data_extractors;
mod processes;
mod custom_parsers;

use processes::bitmex_process::bitmex_process;

#[tokio::main]
async fn main() {
    bitmex_process().await;
}
