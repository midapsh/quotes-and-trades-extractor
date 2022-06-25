use bincode::Options;
use futures::TryStreamExt;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter}; // for write_all()

use crate::commands::bitmex_subscribe::Args;
use crate::configs::configuration;
use crate::custom_parsers::bitmex_parser::{get_default_timestamp, BitmexParser, self};
use crate::data_extractors::bitmex_websocket::BitmexWebsocket;

pub async fn bitmex_process() {
    const COIN: &str = "XBTUSD";
    // NOTE(hspadim): I use this because I need to remove the size of the vector,
    // the first value of the byte stream
    const USIZE_LEN: usize = 8;
    // NOTE(hspadim): 480 KBytes = 48 Bytes (Struct) * 1024 (1K) * 10 (10*1024 = 10_240 items)
    const MAX_CAPACITY_FOR_QUOTES: usize = 10 * 1024 * std::mem::size_of::<bitmex_parser::Quote>();
    // NOTE(hspadim): 330 KBytes = 33 Bytes (Struct) * 1024 (1K) * 10 (10*1024 = 10_240 items)
    const MAX_CAPACITY_FOR_TRADES: usize = 10 * 1024 * std::mem::size_of::<bitmex_parser::Trades>();

    let stream = BitmexWebsocket::connect(Args::WithProduct(vec![
        String::from(format!("quote:{}", COIN)),
        String::from(format!("trade:{}", COIN)),
    ]))
    .await
    .unwrap();
    
    stream
        .try_for_each(|msg| async {
            let settings = configuration::get_configuration().unwrap();
            let filepath_quotes = settings
                .data_quotes_path
                .join(format!("bitmex-{}.dat", COIN));
            let file_quotes = OpenOptions::new()
                    .append(true)
                    .create(true)
                    // .open("/var/lib/trading-system/quotes-and-trades-extractor/v0.1/data/bitmex.log")
                    // .open(format!("bitmex-{}.dat", COIN))
                    .open(filepath_quotes)
                    .await?;
            let mut f_write_quotes = BufWriter::with_capacity(MAX_CAPACITY_FOR_QUOTES, file_quotes);
        
            let filepath_trades = settings
                .data_trades_path
                .join(format!("bitmex-{}.dat", COIN));
            let file_trades = OpenOptions::new()
                    .append(true)
                    .create(true)
                    // .open("/var/lib/trading-system/quotes-and-trades-extractor/v0.1/data/bitmex.log")
                    // .open(format!("bitmex-{}.dat", COIN))
                    .open(filepath_trades)
                    .await?;
            let mut f_write_trades = BufWriter::with_capacity(MAX_CAPACITY_FOR_TRADES, file_trades);
            let custom_bincode = bincode::DefaultOptions::new()
                .with_fixint_encoding()
                .allow_trailing_bytes();
            // const FORMAT: &str = "{default_timestamp},{exchange_timestamp},{symbol},{best_bid_price},{best_bid_size},{best_ask_price},{best_ask_size},{side},{size},{price},{exchange_id}\n";
            let default_timestamp = get_default_timestamp();
            match msg {
                tokio_tungstenite::tungstenite::Message::Text(message)
                    if message.find("table").is_some() =>
                {
                    let bitmex_msg: BitmexParser = serde_json::from_str(&message).unwrap();
                    match bitmex_msg {
                        BitmexParser::Quotes(quotes) => {
                            let mut quotes = quotes.data;
                            for quote in &mut quotes {
                                quote.default_timestamp = default_timestamp;
                            }
                            let bin_quotes = custom_bincode.serialize(&quotes).unwrap();
                            f_write_quotes.write_all(&bin_quotes[USIZE_LEN..]).await?;
                            f_write_quotes.flush().await?;
                        }
                        BitmexParser::Trades(trades) => {
                            let mut trades = trades.data;
                            for trade in &mut trades {
                                trade.default_timestamp = default_timestamp;
                            }
                            let bin_trades = custom_bincode.serialize(&trades).unwrap();
                            f_write_trades.write_all(&bin_trades[USIZE_LEN..]).await?;
                            f_write_trades.flush().await?;
                        }
                        _ => (),
                    }
                }
                // tokio_tungstenite::tungstenite::Message::Binary(_) => todo!(),
                // tokio_tungstenite::tungstenite::Message::Ping(_) => todo!(),
                // tokio_tungstenite::tungstenite::Message::Pong(_) => todo!(),
                // tokio_tungstenite::tungstenite::Message::Close(_) => todo!(),
                _other => (),
            }
            Ok(())
        })
        .await
        .expect("stream fail");
}
