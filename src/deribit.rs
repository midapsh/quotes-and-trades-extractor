mod commands;
mod configs;
mod custom_parsers;
mod data_extractors;
mod processes;


use processes::deribit_process::deribit_process;

#[tokio::main]
async fn main() {
    deribit_process().await;
}
