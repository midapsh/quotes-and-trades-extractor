mod configs;
mod commands;
mod data_extractors;
mod processes;

use processes::deribit_process::deribit_process;

#[tokio::main]
async fn main() {
    deribit_process().await;
}
