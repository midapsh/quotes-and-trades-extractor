use futures::SinkExt;
// use tokio::net::TcpStream;
// use tokio_tungstenite::tungstenite::stream::MaybeTlsStream;
use tokio_tungstenite::{connect_async, tungstenite::Message as TMessage, WebSocketStream};

// use super::errors::deribit_errors::CBError;
// use crate::errors::websocket_errors::WSError;

// use tokio_tungstenite::connect_async;

pub struct DeribitWebsocket;

use crate::commands::deribit_subscribe::{Args, IdCmd, JsonRpc, MethodCmd, Subscribe};

impl DeribitWebsocket {
    const URL: &'static str = "wss://test.deribit.com/ws/api/v2/";

    /// Constructor for simple subcription with product_ids and args
    pub async fn connect(
        args: Args,
    ) -> core::result::Result<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        tokio_tungstenite::tungstenite::error::Error,
    > {
        let subscribe = Subscribe {
            json_rpc: JsonRpc::Version("2.0".to_string()),
            id: IdCmd::Id(1),
            _type: MethodCmd::Subscribe,
            args: args,
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
        // stream
        //     .send(TMessage::Text("{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"public/set_heartbeat\",\"params\":{\"interval\":15}}".to_string()))
        //     .await?;
        stream.send(TMessage::Text(subscribe)).await?;
        println!("subscription sent");

        Ok(stream)
    }
}
