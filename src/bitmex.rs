mod commands;
mod configs;
mod custom_parsers;
mod data_extractors;
mod models;
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
    let list_instruments = [
        // (".BBCH", ".BBCH"),
        // (".BUSDT", ".BUSDT"),
        // (".BLINKT", ".BLINKT"),
        // (".BADAT", ".BADAT"),
        // (".BDOTT", ".BDOTT"),
        // (".BDOGET", ".BDOGET"),
        // (".BSOLT", ".BSOLT"),
        // (".BSUSHIT", ".BSUSHIT"),
        // (".BDOGE", ".BDOGE"),
        // (".BDOT", ".BDOT"),
        // (".BLINK", ".BLINK"),
        // (".BSOL", ".BSOL"),
        // (".BXRPT", ".BXRPT"),
        // (".BBCHT", ".BBCHT"),
        // (".BSHIBT", ".BSHIBT"),
        ("XRPUSD", "XRPUSD"),
        ("BCHUSD", "BCHUSD"),
        ("DOGEUSD", "DOGEUSD"),
        ("BNBUSD", "BNBUSD"),
        ("LINKUSD", "LINKUSD"),
        ("SOLUSD", "SOLUSD"),
        ("LINKUSDT", "LINKUSDT"),
        ("DOGEUSDT", "DOGEUSDT"),
        ("DOTUSDT", "DOTUSDT"),
        ("ADAUSDT", "ADAUSDT"),
        ("BNBUSDT", "BNBUSDT"),
        ("SOLUSDT", "SOLUSDT"),
        ("ADAUSD", "ADAUSD"),
        ("XRPUSDT", "XRPUSDT"),
        ("BCHUSDT", "BCHUSDT"),
        ("DOTUSD", "DOTUSD"),
        ("AVAXUSD", "AVAXUSD"),
        ("SHIBUSDT", "SHIBUSDT"),
        ("AVAXUSDT", "AVAXUSDT"),
        ("UNI_USDT", "UNI_USDT"),
        ("LINK_USDT", "LINK_USDT"),
        // (".BXBT", ".BXBT"),
        // (".BVOL", ".BVOL"),
        // (".BVOL24H", ".BVOL24H"),
        // (".BVOL7D", ".BVOL7D"),
        // (".BETH", ".BETH"),
        // (".BETHT", ".BETHT"),
        // (".BLTC", ".BLTC"),
        // (".BLTCT", ".BLTCT"),
        ("XBTUSD", "XBTUSD"),
        ("XBTUSDT", "XBTUSDT"),
        ("ETHUSD", "ETHUSD"),
        ("ETHUSDT", "ETHUSDT"),
        ("ETH_USDT", "ETH_USDT"),
        ("LTCUSD", "LTCUSD"),
        ("LTCUSDT", "LTCUSDT"),
        // ("ADAU22", "ADAU22"),
        // ("XRPU22", "XRPU22"),
        // ("ETHU22", "ETHU22"),
    ];

    let mut objs: Vec<_> = list_instruments
        .map(|(instrument, instrument_parsed)| {
            let bitmex_process = BitmexProcess::new(
                instrument,
                instrument_parsed,
                &data_quotes_path,
                &data_trades_path,
            );
            bitmex_process
        })
        .into();

    loop {
        let futures = objs.iter_mut().map(|bitmex_process| bitmex_process.run());

        futures::future::try_join_all(futures).await?;
    }
}
