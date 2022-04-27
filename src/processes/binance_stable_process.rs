use futures::TryStreamExt;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt; // for write_all()

use crate::commands::binance_subscribe::Params;
use crate::data_extractors::binance_stable_websocket::BinanceStableWebsocket;

pub async fn binance_stable_process() {
    let stream = BinanceStableWebsocket::connect(Params::WithProduct(vec![String::from(
        "btcusdt@depth@0ms",
    )]))
    .await
    .unwrap();

    stream
        .try_for_each(|msg| async {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("binance_stable.log")
                .await?;
            match msg {
                tokio_tungstenite::tungstenite::Message::Text(message) => {
                    if !message.contains("result\":null") {
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
