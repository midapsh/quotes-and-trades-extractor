use bincode::Options;
use futures::TryStreamExt;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter}; // for write_all()

use crate::commands::bitmex_subscribe::Args;
use crate::custom_deserializers::quotes_and_trades_deserializer::QuotesAndTrades;
use crate::custom_parsers::bitmex_parser::{get_default_timestamp, BitmexParser};
use crate::data_extractors::bitmex_websocket::BitmexWebsocket;

pub async fn bitmex_process() {
    const COIN: &str = "XBTUSD";
    // NOTE(hspadim): I use this because I need to remove the size of the vector,
    // the first value of the byte stream
    const USIZE_LEN: usize = 8;
    let stream = BitmexWebsocket::connect(Args::WithProduct(vec![
        String::from(format!("quote:{}", COIN)),
        String::from(format!("trade:{}", COIN)),
    ]))
    .await
    .unwrap();

    stream
        .try_for_each(|msg| async {
            let file = OpenOptions::new()
                .append(true)
                .create(true)
                // .open("/var/lib/trading-system/quotes-and-trades-extractor/v0.1/data/bitmex.log")
                .open(format!("bitmex-{}.dat", COIN))
                .await?;
            let mut f_write = BufWriter::new(file);
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
                            let quotes = quotes.data.iter().map(|quote| QuotesAndTrades {
                                default_timestamp: default_timestamp,
                                exchange_timestamp: quote.exchange_timestamp,
                                best_bid_price: quote.best_bid_price,
                                best_bid_size: quote.best_bid_size,
                                best_ask_price: quote.best_ask_price,
                                best_ask_size: quote.best_ask_size,
                                side: u8::MIN,
                                size: f64::NAN,
                                price: f64::NAN,
                            }).collect::<Vec<_>>();
                            let bin_quotes = custom_bincode.serialize(&quotes).unwrap();
                            f_write.write_all(&bin_quotes[USIZE_LEN..]).await?;
                            f_write.flush().await?;
                        }
                        BitmexParser::Trades(trades) => {
                            let trades = trades
                                .data
                                .iter()
                                .map(|trade| QuotesAndTrades {
                                    default_timestamp: default_timestamp,
                                    exchange_timestamp: trade.exchange_timestamp,
                                    best_bid_price: f64::NAN,
                                    best_bid_size: f64::NAN,
                                    best_ask_price: f64::NAN,
                                    best_ask_size: f64::NAN,
                                    side: trade.side,
                                    size: trade.size,
                                    price: trade.price,
                                })
                                .collect::<Vec<_>>();
                            let bin_trades = custom_bincode.serialize(&trades).unwrap();
                            f_write.write_all(&bin_trades[USIZE_LEN..]).await?;
                            f_write.flush().await?;
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
