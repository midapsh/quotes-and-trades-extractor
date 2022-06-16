use futures::SinkExt;
// use tokio::net::TcpStream;
// use tokio_tungstenite::tungstenite::stream::MaybeTlsStream;
use tokio_tungstenite::{connect_async, tungstenite::Message as TMessage, WebSocketStream};

// use super::errors::kraken_futures_errors::CBError;
// use crate::errors::websocket_errors::WSError;

// use tokio_tungstenite::connect_async;

pub struct KrakenFuturesWebsocket;

use crate::commands::kraken_futures_subscribe::{FeedType, Subscribe, SubscribeCmd};

impl KrakenFuturesWebsocket {
    const URL: &'static str = "wss://futures.kraken.com/ws/v1";

    /// Constructor for extended subcription via Subscribe structure
    pub async fn connect(
        subscriptions: Vec<Subscribe>,
    ) -> core::result::Result<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        tokio_tungstenite::tungstenite::error::Error,
    > {
        let (mut stream, _response) = connect_async(Self::URL).await?;
        println!("WebSocket handshake has been successfully completed");

        for subscribe in subscriptions {
            let subscribe = serde_json::to_string(&subscribe).unwrap();
            stream.send(TMessage::Text(subscribe)).await?;
        }
        println!("subscription sent");

        Ok(stream)
    }
}
