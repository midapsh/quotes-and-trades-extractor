use futures::SinkExt;
// use tokio::net::TcpStream;
// use tokio_tungstenite::tungstenite::stream::MaybeTlsStream;
use tokio_tungstenite::{connect_async, tungstenite::Message as TMessage, WebSocketStream};

// use super::errors::bitmex_errors::CBError;
// use crate::errors::websocket_errors::WSError;

// use tokio_tungstenite::connect_async;

pub struct BinanceStableWebsocket;

use crate::commands::binance_subscribe::{IdCmd, Params, Subscribe, SubscribeCmd};

impl BinanceStableWebsocket {
    const URL: &'static str = "wss://fstream.binance.com/ws";

    /// Constructor for simple subcription with params
    pub async fn connect(
        params: Params,
    ) -> core::result::Result<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        tokio_tungstenite::tungstenite::error::Error,
    > {
        let subscribe = Subscribe {
            _type: SubscribeCmd::Subscribe,
            params: params,
            id: IdCmd::Id(1),
        };

        Self::connect_with_sub(subscribe).await
    }

    /// Constructor for extended subcription via Subscribe structure
    pub async fn connect_with_sub(
        subscribe: Subscribe,
    ) -> core::result::Result<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        tokio_tungstenite::tungstenite::error::Error,
    > {
        let (mut stream, _response) = connect_async(Self::URL).await?;
        println!("WebSocket handshake has been successfully completed");

        let subscribe = serde_json::to_string(&subscribe).unwrap();
        stream.send(TMessage::Text(subscribe)).await?;
        println!("subscription sent");

        Ok(stream)
    }
}
