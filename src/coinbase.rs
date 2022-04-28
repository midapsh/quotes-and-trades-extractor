mod commands;
mod data_extractors;
mod processes;

use processes::coinbase_process::coinbase_process;

#[tokio::main]
async fn main() {
    coinbase_process().await;
}
