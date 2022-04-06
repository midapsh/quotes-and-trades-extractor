use futures::TryStreamExt;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt; // for write_all()

use crate::commands::kraken_subscribe::{Options, Products};
use crate::data_extractors::kraken_websocket::KrakenWebsocket;

pub async fn kraken_process() {
    let stream = KrakenWebsocket::connect(
        Products::Name(vec![String::from("XBT/USD")]),
        Options::Orderbook {
            name: "book".to_string(),
            depth: 1000,
        },
    )
    .await
    .unwrap();

    stream
        .try_for_each(|msg| async {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("kraken.log")
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
