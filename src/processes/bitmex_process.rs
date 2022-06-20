use futures::TryStreamExt;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt; // for write_all()

use crate::commands::bitmex_subscribe::Args;
use crate::custom_parsers::bitmex_parser::{BitmexParser, get_default_timestamp};
use crate::data_extractors::bitmex_websocket::BitmexWebsocket;

pub async fn bitmex_process() {
    const COIN: &str = "XBTUSD";
    let stream = BitmexWebsocket::connect(Args::WithProduct(vec![
        String::from(format!("quote:{}", COIN)),
        String::from(format!("trade:{}", COIN)),
    ]))
    .await
    .unwrap();

    stream
        .try_for_each(|msg| async {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                // .open("/var/lib/trading-system/quotes-and-trades-extractor/v0.1/data/bitmex.log")
                .open(format!("bitmex-{}.csv", COIN))
                .await?;
            // const FORMAT: &str = "{default_timestamp},{exchange_timestamp},{symbol},{best_bid_price},{best_bid_size},{best_ask_price},{best_ask_size},{side},{size},{price},{exchange_id}\n";
            let default_timestamp = get_default_timestamp();
            match msg {
                    tokio_tungstenite::tungstenite::Message::Text(message) if message.find("table").is_some() => {
                    let bitmex_msg: BitmexParser = serde_json::from_str(&message).unwrap();
                    match bitmex_msg {
                        BitmexParser::Quotes(quotes) => {
                            let quotes_messages = quotes.data.into_iter().map(|quote| {
                                return format!(
                                    // "{default_timestamp},{exchange_timestamp},{symbol},{best_bid_price},{best_bid_size},{best_ask_price},{best_ask_size},{side},{size},{price}\n",
                                    "{default_timestamp},{exchange_timestamp},{best_bid_price},{best_bid_size},{best_ask_price},{best_ask_size},{side},{size},{price}\n",
                                    default_timestamp=default_timestamp,
                                    exchange_timestamp=quote.exchange_timestamp,
                                    // symbol=quote.symbol,
                                    best_bid_price=quote.best_bid_price,
                                    best_bid_size=quote.best_bid_size,
                                    best_ask_price=quote.best_ask_price,
                                    best_ask_size=quote.best_ask_size,
                                    side="",
                                    size="",
                                    price="",
                                );
                            }).collect::<String>();
                            file.write_all(quotes_messages.as_bytes()).await?
                            
                        },
                        BitmexParser::Trades(trades) => {
                            let trades_messages = trades.data.into_iter().map(|trade| {
                                return format!(
                                    // "{default_timestamp},{exchange_timestamp},{symbol},{best_bid_price},{best_bid_size},{best_ask_price},{best_ask_size},{side},{size},{price}\n",
                                    "{default_timestamp},{exchange_timestamp},{best_bid_price},{best_bid_size},{best_ask_price},{best_ask_size},{side},{size},{price}\n",
                                    default_timestamp=default_timestamp,
                                    exchange_timestamp=trade.exchange_timestamp,
                                    // symbol=trade.symbol,
                                    best_bid_price="",
                                    best_bid_size="",
                                    best_ask_price="",
                                    best_ask_size="",
                                    side=trade.side,
                                    size=trade.size,
                                    price=trade.price,
                                );
                            }).collect::<String>();
                            file.write_all(trades_messages.as_bytes()).await?
                        },
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
