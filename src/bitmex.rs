mod commands;
mod configs;
mod custom_parsers;
mod data_extractors;
mod processes;
mod models;

use std::fs;

use crate::configs::configuration::get_configuration;
use processes::bitmex_process::BitmexProcess;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    const VENUE: &str = "BITMEX";
    let settings = get_configuration().unwrap();

    let instruments = [
        "AVAXUSDT",
        "UNI_USDT",
        "LINK_USDT",
        "XBTUSD",
        "XBTUSDT",
        "ETHUSD",
        "ETHUSDT",
        "ETH_USDT",
        "LTCUSD",
        "LTCUSDT",
    ];
    let instruments_parsed = [
        "AVAXUSDT",
        "UNI_USDT",
        "LINK_USDT",
        "XBTUSD",
        "XBTUSDT",
        "ETHUSD",
        "ETHUSDT",
        "ETH_USDT",
        "LTCUSD",
        "LTCUSDT",
    ];



    let data_quotes_path = settings.data_quotes_path.join(VENUE);
    let data_trades_path = settings.data_trades_path.join(VENUE);

    fs::create_dir_all(&data_quotes_path)
        .expect(format!("Couldn't create '{}' quotes file", VENUE).as_str());
    fs::create_dir_all(&data_trades_path)
        .expect(format!("Couldn't create '{}' trades file", VENUE).as_str());

    // let instrument = "XBTUSD";
    // let instrument_parsed = "XBTUSD";

    let mut bitmex_process = BitmexProcess::new(
        instruments.to_vec(),
        instruments_parsed.to_vec(),
        &data_quotes_path,
        &data_trades_path,
    );
    bitmex_process.run().await?;
    Ok(())
}
