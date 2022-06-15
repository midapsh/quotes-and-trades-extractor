use futures::{SinkExt, StreamExt, TryStreamExt};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt; // for write_all()
use tokio_tungstenite::tungstenite::Message as TMessage;

use crate::commands::deribit_subscribe::Args;
use crate::data_extractors::deribit_websocket::DeribitWebsocket;

pub async fn deribit_process() {
    let stream = DeribitWebsocket::connect(Args::Orderbook {
        channels: vec![
            "quote.BTC-PERPETUAL".to_string(),
            "trades.BTC-PERPETUAL.raw".to_string(),
        ],
    })
    .await
    .unwrap();

    let (write, read) = stream.split();
    let _ = read
        .try_fold(write, |mut write, msg| async move {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("deribit.log")
                .await?;
            match msg {
                TMessage::Text(message) => {
                    if message.contains("test_request") {
                        write
                            .send(TMessage::Text(
                                serde_json::json!({
                                    "jsonrpc": "2.0",
                                    "id": 0,
                                    "method": "public/test",
                                    "params": {}
                                })
                                .to_string(),
                            ))
                            .await?
                    } else if message.contains("subscription") {
                        file.write_all(format!("{}\n", message).as_bytes()).await?
                    } else {
                        println!("{}", message)
                    }
                }
                _ => println!("other"),
            }
            Ok(write)
        })
        .await
        .expect("stream fail");
}
