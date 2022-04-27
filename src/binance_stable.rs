mod commands;
mod data_extractors;
mod processes;

use processes::binance_stable_process::binance_stable_process;

#[tokio::main]
async fn main() {
    binance_stable_process().await;
}
