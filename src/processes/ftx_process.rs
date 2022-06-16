use futures::{SinkExt, StreamExt, TryStreamExt};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt; // for write_all()
use tokio_tungstenite::tungstenite::Message as TMessage;

use crate::commands::ftx_subscribe::{ArgsType, Product};
use crate::data_extractors::ftx_websocket::FTXWebsocket;

pub async fn ftx_process() {
    let stream = FTXWebsocket::connect(ArgsType::Ticker, Product::Name(String::from("btc/usd")))
        .await
        .unwrap();

    let (mut write, read) = stream.split();

    tokio::select! {
        // res = async {
        _ = async {
            read.try_for_each(|msg| async {
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("ftx.log")
                    .await?;
                match msg {
                    tokio_tungstenite::tungstenite::Message::Text(message) => {
                        file.write_all(format!("{}\n", message).as_bytes()).await?
                    }
                    // tokio_tungstenite::tungstenite::Message::Binary(_) => todo!(),
                    // tokio_tungstenite::tungstenite::Message::Ping(message) => todo!(),
                    // tokio_tungstenite::tungstenite::Message::Pong(_) => todo!(),
                    // tokio_tungstenite::tungstenite::Message::Close(_) => todo!(),
                    _other => println!("other"),
                }
                Ok(())
            })
            .await
            .expect("stream fail");
        } => {}
        // res = async {
        //     a.await?
        // }
        // => {
        //     res?;
        // }
        _ = async {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                write
                    .send(TMessage::Text(
                        serde_json::json!({"op": "ping"}).to_string(),
                    ))
                .await
                .expect("stream fail");
                // .await?;
            }
        } => {}
    }
}
