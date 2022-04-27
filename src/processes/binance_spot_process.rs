use futures::TryStreamExt;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt; // for write_all()

use crate::commands::binance_subscribe::Params;
use crate::data_extractors::binance_spot_websocket::BinanceSpotWebsocket;

pub async fn binance_spot_process() {
    let stream = BinanceSpotWebsocket::connect(Params::WithProduct(vec![String::from(
        "btcusdt@depth@100ms",
    )]))
    .await
    .unwrap();

    stream
        .try_for_each(|msg| async {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("binance_spot.log")
                .await?;
            match msg {
                tokio_tungstenite::tungstenite::Message::Text(message) => {
                    if message.contains("stream") {
                        file.write_all(format!("{}\n", message).as_bytes()).await?
                    } else {
                        println!("{:?}", message);
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
