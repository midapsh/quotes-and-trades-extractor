use futures::{SinkExt, TryStreamExt};
// use tokio::net::TcpStream;
// use tokio_tungstenite::tungstenite::stream::MaybeTlsStream;
use tokio_tungstenite::{connect_async, tungstenite::Message as TMessage, WebSocketStream};

// use super::errors::bitstamp_errors::CBError;
// use crate::errors::websocket_errors::WSError;

// use tokio_tungstenite::connect_async;

pub struct BitstampWebsocket;

use crate::commands::bitstamp_subscribe::{Data, Subscribe, SubscribeCmd};

impl BitstampWebsocket {
    const URL: &'static str = "wss://ws.bitstamp.net";

    /// Maximum connection age
    /// Maximum connection age is 90 days from the time the connection is established. When that period of time elapses, you will be automatically disconnected and will need to re-connect.
    /// Link: https://www.bitstamp.net/websocket/v2/
    pub async fn connect(
        data: Vec<Data>,
    ) -> core::result::Result<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        tokio_tungstenite::tungstenite::error::Error,
    > {
        let subscriptions = data.into_iter().map(|x| Subscribe {
            _type: SubscribeCmd::Subscribe,
            data: x,
        }).collect();

        Self::connect_with_sub(subscriptions).await
    }

    /// Constructor for extended subcription via Subscribe structure
    pub async fn connect_with_sub(
        subscriptions: Vec<Subscribe>,
    ) -> core::result::Result<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        tokio_tungstenite::tungstenite::error::Error,
    > {
        let (mut stream, _response) = connect_async(Self::URL).await?;
        println!("WebSocket handshake has been successfully completed");

        for subscription in subscriptions.into_iter() {
            let subscribe = serde_json::to_string(&subscription).unwrap();
            stream.send(TMessage::Text(subscribe)).await?;
        }
        println!("subscriptions sent");
        
        Ok(stream)
    }
}
