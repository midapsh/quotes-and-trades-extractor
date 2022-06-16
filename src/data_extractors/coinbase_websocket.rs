use futures::SinkExt;
// use tokio::net::TcpStream;
// use tokio_tungstenite::tungstenite::stream::MaybeTlsStream;
use tokio_tungstenite::{connect_async, tungstenite::Message as TMessage, WebSocketStream};

// use super::errors::coinbase_errors::CBError;
// use crate::errors::websocket_errors::WSError;

// use tokio_tungstenite::connect_async;

pub struct CoinbaseWebsocket;

use crate::commands::coinbase_subscribe::{Channel, Subscribe, SubscribeCmd};

impl CoinbaseWebsocket {
    const URL: &'static str = "wss://ws-feed-public.sandbox.pro.coinbase.com";

    /// Constructor for simple subcription with product_ids and channels
    pub async fn connect(
        channels: Vec<Channel>,
    ) -> core::result::Result<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        tokio_tungstenite::tungstenite::error::Error,
    > {
        let subscriptions = Subscribe {
            _type: SubscribeCmd::Subscribe,
            channels,
        };

        Self::connect_with_sub(subscriptions).await
    }

    /// Constructor for extended subcription via Subscribe structure
    pub async fn connect_with_sub(
        subscriptions: Subscribe,
    ) -> core::result::Result<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        tokio_tungstenite::tungstenite::error::Error,
    > {
        let (mut stream, _response) = connect_async(Self::URL).await?;
        println!("WebSocket handshake has been successfully completed");

        let subscribe = serde_json::to_string(&subscriptions).unwrap();
        stream.send(TMessage::Text(subscribe)).await?;
        println!("subscription sent");

        Ok(stream)
    }
}
