mod commands;
mod data_extractors;
mod processes;

use processes::okx_process::okx_process;

#[tokio::main]
async fn main() {
    okx_process().await;
}
