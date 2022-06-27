mod commands;
mod configs;
mod custom_parsers;
mod data_extractors;
mod processes;

use std::fs;

use crate::configs::configuration::get_configuration;
use processes::bitmex_process::BitmexProcess;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    const VENUE: &str = "BITMEX";
    let settings = get_configuration().unwrap();

    let data_quotes_path = settings.data_quotes_path.join(VENUE);
    let data_trades_path = settings.data_trades_path.join(VENUE);

    fs::create_dir_all(&data_quotes_path)
        .expect(format!("Couldn't create '{}' quotes file", VENUE).as_str());
    fs::create_dir_all(&data_trades_path)
        .expect(format!("Couldn't create '{}' trades file", VENUE).as_str());

    let instrument = "XBTUSD";
    let instrument_parsed = "XBTUSD";

    let mut bitmex_process = BitmexProcess::new(
        instrument,
        instrument_parsed,
        &data_quotes_path,
        &data_trades_path,
    );
    bitmex_process.run().await?;
    Ok(())
}
