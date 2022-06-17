use futures::TryStreamExt;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt; // for write_all()

use crate::commands::kraken_futures_subscribe::{FeedType, Subscribe, SubscribeCmd};
use crate::data_extractors::kraken_futures_websocket::KrakenFuturesWebsocket;


pub async fn kraken_futures_process() {
    let stream =
        KrakenFuturesWebsocket::connect(vec![
            Subscribe {
                _type: SubscribeCmd::Subscribe,
                product_ids: vec![String::from("PI_XBTUSD")],
                feed: FeedType::Trade,
            },
            Subscribe {
                _type: SubscribeCmd::Subscribe,
                product_ids: vec![String::from("PI_XBTUSD")],
                feed: FeedType::Ticker,
            },
        ])
        .await
        .unwrap();

    stream
        .try_for_each(|msg| async {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("/var/lib/trading-system/quotes-and-trades-extractor/v0.1/data/kraken_futures.log")
                .await?;
            match msg {
                tokio_tungstenite::tungstenite::Message::Text(message) => {
                    if message.starts_with("{\"event") {
                        ()
                    } else {
                        file.write_all(format!("{}\n", message).as_bytes()).await?
                    }
                }
                // tokio_tungstenite::tungstenite::Message::Binary(_) => todo!(),
                // tokio_tungstenite::tungstenite::Message::Ping(_) => todo!(),
                // tokio_tungstenite::tungstenite::Message::Pong(_) => todo!(),
                // tokio_tungstenite::tungstenite::Message::Close(_) => todo!(),
                _other => println!("other"),
            }
            Ok(())
        })
        .await
        .expect("stream fail");
}
