mod commands;
mod configs;
mod custom_parsers;
mod data_extractors;
mod processes;

use processes::bitmex_process::BitmexProcess;

#[tokio::main]
async fn main() {
    let coin = "XBTUSD";
    let mut bitmex_process = BitmexProcess::new(coin);
    bitmex_process.run().await;
}
