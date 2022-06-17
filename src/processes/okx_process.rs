use futures::TryStreamExt;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt; // for write_all()

use crate::commands::okx_subscribe::{Arg, ChannelsType};
use crate::data_extractors::okx_websocket::OkxWebsocket;

pub async fn okx_process() {
    let stream = OkxWebsocket::connect(vec![
        Arg {
            channel: ChannelsType::Quotes,
            instrument_id: String::from("BTC-USDT"),
        },
        Arg {
            channel: ChannelsType::Trades,
            instrument_id: String::from("BTC-USDT"),
        }
    ])
    .await
    .unwrap();

    stream
        .try_for_each(|msg| async {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("/var/lib/trading-system/quotes-and-trades-extractor/v0.1/data/okx.log")
                .await?;
            match msg {
                tokio_tungstenite::tungstenite::Message::Text(message) => {
                    if message.starts_with("{\"event\":\"subscribe") {
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
