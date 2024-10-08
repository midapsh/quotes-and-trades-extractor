mod commands;
mod configs;
mod custom_parsers;
mod data_extractors;
mod processes;
mod models;

use processes::binance_coin_process::binance_coin_process;

#[tokio::main]
async fn main() {
    binance_coin_process().await;
}
