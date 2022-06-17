use futures::TryStreamExt;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt; // for write_all()

use crate::commands::bitmex_subscribe::Args;
use crate::data_extractors::bitmex_websocket::BitmexWebsocket;

pub async fn bitmex_process() {
    let stream = BitmexWebsocket::connect(Args::WithProduct(vec![
        String::from("quote:XBTUSD"),
        String::from("trade:XBTUSD"),
    ]))
    .await
    .unwrap();

    stream
        .try_for_each(|msg| async {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("/var/lib/trading-system/quotes-and-trades-extractor/v0.1/data/bitmex.log")
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
