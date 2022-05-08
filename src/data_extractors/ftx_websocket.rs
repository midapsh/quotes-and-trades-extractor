use futures::SinkExt;
// use tokio::net::TcpStream;
// use tokio_tungstenite::tungstenite::stream::MaybeTlsStream;
use tokio_tungstenite::{connect_async, tungstenite::Message as TMessage, WebSocketStream};

// use super::errors::ftx_errors::CBError;
// use crate::errors::websocket_errors::WSError;

// use tokio_tungstenite::connect_async;

pub struct FTXWebsocket;

use crate::commands::ftx_subscribe::{ArgsType, Product, Subscribe, SubscribeCmd};

impl FTXWebsocket {
    const URL: &'static str = "wss://ftx.com/ws/";

    /// Constructor for simple subcription with product_ids and args
    pub async fn connect(
        arg: ArgsType,
        product: Product,
    ) -> core::result::Result<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        tokio_tungstenite::tungstenite::error::Error,
    > {
        let subscribe = Subscribe {
            _type: SubscribeCmd::Subscribe,
            arg,
            product,
        };
        println!("{:?}", serde_json::to_string(&subscribe));

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
        stream
            .send(TMessage::Text(
                serde_json::json!({"op": "ping"}).to_string(),
            ))
            .await?;
        stream.send(TMessage::Text(subscribe)).await?;
        println!("subscription sent");

        Ok(stream)
    }
}
