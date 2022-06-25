mod commands;
mod configs;
mod custom_parsers;
mod data_extractors;
mod processes;

use processes::binance_spot_process::binance_spot_process;

#[tokio::main]
async fn main() {
    binance_spot_process().await;
}
