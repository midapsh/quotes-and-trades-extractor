use futures::TryStreamExt;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt; // for write_all()

use crate::commands::bitstamp_subscribe::Data;
use crate::data_extractors::bitstamp_websocket::BitstampWebsocket;

pub async fn bitstamp_process() {
    let stream = BitstampWebsocket::connect(vec![
        Data {
            channel: String::from(format!("{}{}", "order_book_", "btcusd")),
        },
        Data {
            channel: String::from(format!("{}{}", "live_trades_", "btcusd")),
        },
    ])
    .await
    .unwrap();

    stream
        .try_for_each(|msg| async {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("bitstamp.log")
                .await?;
            match msg {
                tokio_tungstenite::tungstenite::Message::Text(message) => {
                    if message.starts_with("{\"event\":\"bts:subscription_succeeded\"") {
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
