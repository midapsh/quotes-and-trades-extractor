use futures::TryStreamExt;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt; // for write_all()


use crate::commands::binance_subscribe::Params;
use crate::data_extractors::binance_coin_websocket::BinanceCoinWebsocket;

pub async fn binance_coin_process() {
    let stream = BinanceCoinWebsocket::connect(Params::WithProduct(vec![
        String::from("btcusd_perp@bookTicker"),
        String::from("btcusd_perp@trade"),
    ]))
    .await
    .unwrap();
    
    stream
        .try_for_each(|msg| async {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("/var/lib/trading-system/quotes-and-trades-extractor/v0.1/data/binance_coin.log")
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
