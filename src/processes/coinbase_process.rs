use futures::TryStreamExt;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt; // for write_all()

use crate::commands::coinbase_subscribe::{Channel, ChannelType};
use crate::data_extractors::coinbase_websocket::CoinbaseWebsocket;

pub async fn coinbase_process() {
    let stream = CoinbaseWebsocket::connect(vec![
        // NOTE(hspadim): Don`t have quotes messages
        Channel::WithProduct {
            channel: ChannelType::Level2,
            products_ids: vec![String::from("BTC-USD")],
        },
        // NOTE(hspadim): Save best bid/ask
        Channel::WithProduct {
            channel: ChannelType::Ticker,
            products_ids: vec![String::from("BTC-USD")],
        },
    ])
    .await
    .unwrap();

    stream
        .try_for_each(|msg| async {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("/var/lib/trading-system/quotes-and-trades-extractor/v0.1/data/coinbase.log")
                .await?;
            match msg {
                tokio_tungstenite::tungstenite::Message::Text(message) => {
                    file.write_all(format!("{}\n", message).as_bytes()).await?
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
